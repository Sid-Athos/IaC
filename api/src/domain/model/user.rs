use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
    pub experience : i32,
    pub level : i32
}
