use axum::{Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;



use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;

use crate::service::ranking_service::RankingService;


#[utoipa::path(
    delete,
    path = "/ranking/game/{game_id}",
    responses(
        (status = 200, description = "User deleted", body = UserResponse),
        (status = 401, description = "Invalid token",),
    ),
security(
("api-key" = []),
("bearer" = [])
),
    tag="ranking"
)]
pub async fn delete_by_game(State(pool): State<ConnectionPool>, Path(game_id) : Path<i32>) -> Result<Json<Vec<RankingResponse>>, StatusCode> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone())
    );

    let result = ranking_service.delete_ranking_by_game(game_id).await?;

    Ok(Json(RankingResponse::from_vec_domain(result, pool).await))
}