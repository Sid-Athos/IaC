use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::session::Session;
use crate::entrypoint::user::route::response::session_response::SessionResponse;
use crate::service::session_service::SessionService;

#[utoipa::path(
    post,
    path = "/user/logout",
    responses(
        (status = 200, description = "User disconnected", body = SessionResponse),
        (status = 401, description = "Invalid token",),
    ),
    security(
        (   "api-key" = []),
        ("bearer" = [])
    ),
    tag = "user"
)]
pub async fn user_logout(State(pool): State<ConnectionPool>, Extension(session): Extension<Session>) -> Result<Json<SessionResponse>, StatusCode> {
    let session_service = SessionService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let session = session_service.delete_token(session.token).await?;

    Ok(Json(SessionResponse::from_domain(session)))
}