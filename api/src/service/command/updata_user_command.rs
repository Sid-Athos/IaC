

pub struct UpdateUserCommand {
    pub id: i32,
    pub pseudo: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserCommand {

    pub fn new(user_id: i32, pseudo: Option<String>, email: Option<String>, password: Option<String>) -> Self {
        UpdateUserCommand {
            id: user_id,
            pseudo,
            email,
            password,
        }
    }
}