use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Ranking {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub rank : i32,
    pub nb_games : i32
}