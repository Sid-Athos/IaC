use axum::{Json};


use axum::extract::{Path, State};
use axum::http::StatusCode;




use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;

use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;
use crate::service::ranking_service::RankingService;

#[utoipa::path(
    get,
    path = "/ranking/game/{game_id}",
    params(
        ("user_id" = String,),
    ),
    responses(
        (status = 200, description = "Rankings found", body = RankingResponse,),
        (status = 404, description = "Game not found",),
    ),
security(
("api-key" = []),
("bearer" = [])
),
    tag="ranking"
)]
pub async fn get_ranking_by_game_id(State(pool): State<ConnectionPool>, Path(game_id): Path<i32>) -> Result<Json<Vec<RankingResponse>>, StatusCode> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone()),
    );

    let result = ranking_service.get_ranking_by_game(game_id).await?;

    Ok(Json(RankingResponse::from_vec_domain(result, pool).await))
}