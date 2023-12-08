use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct FriendList {
    pub id: i32,
    pub applicant_id: i32,
    pub recipient_id: i32,
    pub accepted: bool,
}