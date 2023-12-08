use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::user::User;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub admin: bool,
}

impl UserResponse {
    pub fn from_domain(user: User) -> Self {
        UserResponse {
            id: user.id,
            pseudo: user.pseudo,
            email: user.email,
            admin: user.admin,
        }
    }

    pub fn from_vec_domain(users: Vec<User>) -> Vec<Self> {
        users.into_iter().map(|user| UserResponse::from_domain(user)).collect()
    }
}