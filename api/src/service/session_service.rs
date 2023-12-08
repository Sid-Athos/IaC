use axum::http::StatusCode;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::session::Session;
use crate::domain::model::user::User;

pub struct SessionService {
    pub user_repository: UserRepository,
    pub session_repository: SessionRepository,
}

impl SessionService {

    pub fn new(user_repository: UserRepository, session_repository: SessionRepository) -> Self {
        SessionService {
            user_repository,
            session_repository,
        }
    }

    pub async fn delete_token(&self, token: String) -> Result<Session, StatusCode> {
        self.session_repository.delete_token(token).await.map_err(database_error_to_status_code)
    }

    pub async fn get_user_by_token(&self, token: String) -> Result<User, StatusCode> {
        let token = self.session_repository.get_by_token(token).await.map_err(database_error_to_status_code)?;

        self.user_repository.get_user_by_id(token.user_id).await.map_err(database_error_to_status_code)
    }
}