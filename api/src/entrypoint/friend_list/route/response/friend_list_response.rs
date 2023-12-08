use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::friend_list::FriendList;
use crate::entrypoint::friend_list::route::response::user_response::UserResponse;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FriendListResponse {
    pub id: i32,
    pub applicant_id: i32,
    pub recipient_id: i32,
    pub applicant: UserResponse,
    pub recipient: UserResponse,
    pub accepted: bool,
}

impl FriendListResponse {
    pub fn from_domain(friend_list: FriendList, applicant: UserResponse, recipient: UserResponse) -> Self {
        FriendListResponse {
            id: friend_list.id,
            applicant_id: friend_list.applicant_id,
            recipient_id: friend_list.recipient_id,
            applicant,
            recipient,
            accepted: friend_list.accepted,
        }
    }
}