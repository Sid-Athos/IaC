use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}