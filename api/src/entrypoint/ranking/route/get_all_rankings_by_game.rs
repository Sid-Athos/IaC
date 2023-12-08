use axum::{Json};


use axum::extract::{Path, State};
use axum::http::StatusCode;




use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;
use crate::domain::model::ranking_by_game::RankingByGame;

use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;
use crate::service::ranking_service::RankingService;



pub async fn get_all_ranking_by_games(State(pool): State<ConnectionPool>) -> Result<Json<Vec<RankingByGame>>, StatusCode> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone()),
    );

    let result = ranking_service.get_all_rankings_by_game().await?;

    Ok(Json(result))
}