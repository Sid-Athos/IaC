use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;
use crate::entrypoint::ranking::route::request::ranking_request::RankingRequest;
use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;



use crate::service::ranking_service::RankingService;


#[utoipa::path(
    post,
    path = "/ranking/",
    request_body = RankingRequest,
    responses(
        (status = 200, description = "Ranking initialised", body = RankingResponse,),
        (status = 409, description = "Ranking already init",),
    ),
    security(
    ("api-key" = []),
    ("bearer" = [])
    ),
    tag="ranking"
)]
pub async fn init_ranking(State(pool): State<ConnectionPool>, Json(ranking): Json<RankingRequest>) -> Result<Json<RankingResponse>, StatusCode> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone())
    );

    let ranking = ranking_service.init_ranking(ranking.user_id,ranking.game_id).await?;

    Ok(Json(RankingResponse::from_domain(ranking, pool).await))
}