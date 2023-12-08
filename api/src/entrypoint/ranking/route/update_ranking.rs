use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;



use crate::entrypoint::ranking::route::request::update_ranking_request::UpdateRankingRequest;
use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;



use crate::service::ranking_service::RankingService;


#[utoipa::path(
    put,
    path = "/ranking/",
    request_body = UpdateRankingRequest,
    responses(
        (status = 200, description = "Ranking updated", body = RankingResponse,),
    ),
    security(
    ("api-key" = []),
    ("bearer" = [])
    ),
    tag="ranking"
)]
pub async fn update_ranking(State(pool): State<ConnectionPool>, Json(ranking): Json<UpdateRankingRequest>) -> Result<Json<Vec<RankingResponse>>, StatusCode> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone())
    );

    let rankings = ranking_service.update_ranking3(ranking.game_id,ranking.losers_id,ranking.tie,ranking.winner_id, ranking.average_rank).await?;

    Ok(Json(RankingResponse::from_vec_domain(rankings, pool).await))
}