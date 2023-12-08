use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::user_service::UserService;

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    params(
        ("user_id" = String,),
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse,),
        (status = 404, description = "User not found",),
    ),
    security(
        ("api-key" = []),
        ("bearer" = [])
    ),
    tag = "user"
)]
pub async fn user_get(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>) -> Result<Json<UserResponse>, StatusCode> {
    tracing::info!("Calling user_get for user_id {}", user_id);
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.get_user_by_id(user_id).await?;

    tracing::info!("Found {:?}", user);

    Ok(Json(UserResponse::from_domain(user)))
}