use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreateUserRequest {
    pub nickname: String,
    pub email: String,
    pub password: String,
}