use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RankingRequest {
    pub user_id: i32,
    pub game_id: i32,
}