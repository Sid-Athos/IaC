use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateRankingRequest {
    pub winner_id: i32,
    pub losers_id: Vec<i32>,
    pub game_id: i32,
    pub tie : bool,
    pub average_rank : i32
}