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
    delete,
    path = "/user/",
    responses(
        (status = 200, description = "User deleted", body = UserResponse),
        (status = 401, description = "Invalid token",),
    ),
security(
("api-key" = []),
("bearer" = [])
),
    tag="user"
)]
pub async fn delete_user(State(pool): State<ConnectionPool>, Extension(user): Extension<User>) -> Result<Json<UserResponse>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.delete_account(user.id).await?;

    Ok(Json(UserResponse::from_domain(user)))
}