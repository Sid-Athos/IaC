use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub nickname: String,
    pub password: String,
}
