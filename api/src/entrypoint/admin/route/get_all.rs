use axum::extract::{State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::admin::route::response::user_response::UserResponse;
use crate::service::user_service::UserService;

#[utoipa::path(
    get,
    path = "/admin/",
    responses(
        (status = 200, description = "List of Users", body = Vec<UserResponse>,),
        (status = 401, description = "Invalid token",),
    ),
security(
("api-key" = []),
("bearer" = [])
),
    tag = "admin"
)]
pub async fn get_all(State(pool): State<ConnectionPool>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let users = user_service.get_all_users().await?;

    Ok(Json(UserResponse::from_vec_domain(users)))
}