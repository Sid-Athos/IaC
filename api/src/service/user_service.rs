use bcrypt::{hash, verify};
use axum::http::StatusCode;

use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use regex::{Regex, RegexSet};

use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::{database_error_to_status_code, internal_error};
use crate::domain::model::session::Session;
use crate::domain::model::user::User;
use crate::service::command::change_password_command::ChangePasswordCommand;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::command::login_command::LoginCommand;
use crate::service::command::updata_user_command::UpdateUserCommand;

pub struct UserService {
    pub user_repository: UserRepository,
    pub session_repository: SessionRepository,
}

impl UserService {

    pub fn new(user_repository: UserRepository, session_repository: SessionRepository) -> Self {
        UserService {
            user_repository,
            session_repository,
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, StatusCode> {
        self.user_repository.get_user_by_id(user_id).await.map_err(database_error_to_status_code)
    }



    pub async fn create_user(&self, mut user: CreateUserCommand) -> Result<User, StatusCode> {
        let password_regex_set = RegexSet::new(&[
            r"[A-Z]",
            r"[a-z]",
            r"\d",
            r"[a-zA-Z0-9]"
        ]).unwrap();

        let password_matches : Vec<_> = password_regex_set.matches(user.password.as_str()).into_iter().collect();

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if password_matches.len() != 4 && user.password.len() < 8
        {
            return Err(StatusCode::BAD_REQUEST);
        }

        if !email_regex.is_match(&*user.email) {
            return Err(StatusCode::BAD_REQUEST);
        }

        user.password = hash(user.password, 4).map_err(internal_error)?;

        self.user_repository.create_user(user).await.map_err(database_error_to_status_code)
    }

    pub async fn login_user(&self, user: LoginCommand) -> Result<Session, StatusCode> {
        let user_bdd = self.user_repository.get_user_by_pseudo(user.pseudo).await.map_err(database_error_to_status_code)?;

        if verify(user.password, &user_bdd.password).map_err(internal_error)? {

            let token: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(256)
                .map(char::from)
                .collect();

            self.session_repository.create_session(user_bdd.id, token).await.map_err(database_error_to_status_code)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    pub async fn search_user(&self, search: String) -> Result<Vec<User>, StatusCode> {
        self.user_repository.search_user(search).await.map_err(database_error_to_status_code)
    }

    pub async fn delete_account(&self, user_id: i32) -> Result<User, StatusCode> {
        self.session_repository.delete_all_for_user(user_id).await.map_err(database_error_to_status_code)?;
        self.user_repository.delete_account(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn change_password(&self, change_password_command: ChangePasswordCommand) -> Result<User, StatusCode> {
        let user_bdd = self.user_repository.get_user_by_id(change_password_command.user_id).await.map_err(database_error_to_status_code)?;

        if verify(change_password_command.old_password, &user_bdd.password).map_err(internal_error)? {
            let new_password = hash(change_password_command.new_password, 4).map_err(internal_error)?;
            self.user_repository.change_password(change_password_command.user_id, new_password).await.map_err(database_error_to_status_code)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, StatusCode> {
        self.user_repository.get_all_users().await.map_err(database_error_to_status_code)
    }

    pub async fn get_everyone_but_me(&self, user_id: i32) -> Result<Vec<User>, StatusCode> {
        self.user_repository.get_everyone_but_me(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn give_admin_role(&self, user_id: i32) -> Result<User, StatusCode> {
        self.user_repository.give_admin_role(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn remove_admin_role(&self, user_id: i32) -> Result<User, StatusCode> {
        self.user_repository.remove_admin_role(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn update_user(&self, update_user_command: UpdateUserCommand) -> Result<User, StatusCode> {
        let mut user = self.user_repository.get_user_by_id(update_user_command.id).await.map_err(database_error_to_status_code)?;

        if update_user_command.pseudo.is_some() {
            user = self.user_repository.change_pseudo(update_user_command.pseudo.unwrap_or_default(), update_user_command.id).await.map_err(database_error_to_status_code)?;
        }

        if update_user_command.email.is_some() {
            user = self.user_repository.change_email(update_user_command.email.unwrap_or_default(), update_user_command.id).await.map_err(database_error_to_status_code)?;
        }

        if update_user_command.password.is_some() {
            let new_password = hash(update_user_command.password.unwrap_or_default(), 4).map_err(internal_error)?;
            user = self.user_repository.change_password(update_user_command.id, new_password).await.map_err(database_error_to_status_code)?;
        }

        Ok(user)
    }

    pub async fn add_experience(&self, user_id : i32) -> Result<User, StatusCode> {
        let mut user = self.user_repository.get_user_by_id(user_id).await.map_err(database_error_to_status_code)?;
        user.experience = user.experience + 1;
        if user.experience >= user.level*10 {
            user.experience = 0;
            user.level = user.level + 1;
        }
        self.user_repository.add_experience(user_id,user.experience,user.level).await.map_err(database_error_to_status_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let regex_set = RegexSet::new(&[
            r"[A-Z]",
            r"[a-z]",
            r"\d",
            r"[a-zA-Z0-9]"
        ]).unwrap();
        let lol : Vec<_> = regex_set.matches("dqsdq").into_iter().collect();
        assert_eq!(lol.len(), 2);
        let lal : Vec<_> = regex_set.matches("1ascbfjsU").into_iter().collect();
        assert_eq!(lal.len(), 4)
    }


}