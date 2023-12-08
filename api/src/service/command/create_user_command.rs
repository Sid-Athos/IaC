use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;

#[derive(Debug)]
pub struct CreateUserCommand {
    pub pseudo: String,
    pub email: String,
    pub password: String,
}

impl CreateUserCommand {

    pub fn new(request: CreateUserRequest) -> Self {
        CreateUserCommand {
            pseudo: request.nickname,
            email: request.email,
            password: request.password,
        }
    }
}