use crate::entrypoint::user::route::request::change_password_request::ChangePasswordRequest;

pub struct ChangePasswordCommand {
    pub user_id: i32,
    pub old_password: String,
    pub new_password: String,
}

impl ChangePasswordCommand {
    pub fn new(user_id: i32, change_password_request: ChangePasswordRequest) -> Self {
        ChangePasswordCommand {
            user_id,
            old_password: change_password_request.old_password,
            new_password: change_password_request.new_password,
        }
    }
}