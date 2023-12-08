use crate::entrypoint::user::route::request::login_request::LoginRequest;

pub struct LoginCommand {
    pub pseudo: String,
    pub password: String,
}

impl LoginCommand {
    pub fn new(login_request: LoginRequest) -> Self {
        LoginCommand {
            pseudo: login_request.nickname,
            password: login_request.password,
        }
    }
}