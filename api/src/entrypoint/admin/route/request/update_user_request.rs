use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub pseudo: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}