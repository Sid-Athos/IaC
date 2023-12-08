use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct RankingByGame {
    pub rank: i32,
    pub nb_games: i32,
    pub pseudo : String,
    pub game : String
}