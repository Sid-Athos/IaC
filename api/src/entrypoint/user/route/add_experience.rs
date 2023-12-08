use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::user::User;

use crate::entrypoint::user::route::response::user_response::UserResponse;

use crate::service::user_service::UserService;

#[utoipa::path(
    put,
    path = "/user/add_experience",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 401, description = "Invalid token",),
        (status = 409, description = "Pseudo already used",),
    ),
    security(
        ("api-key" = []),
        ("bearer" = [])
    ),
    request_body = UpdateUserRequest,
    tag="user"
)]
pub async fn add_experience(State(pool): State<ConnectionPool>, Extension(user): Extension<User>) -> Result<Json<UserResponse>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.add_experience(user.id).await?;

    Ok(Json(UserResponse::from_domain(user)))
}