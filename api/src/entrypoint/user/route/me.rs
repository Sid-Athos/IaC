use axum::{Extension, Json};
use axum::http::StatusCode;
use crate::domain::model::user::User;
use crate::entrypoint::user::route::response::user_response::UserResponse;

#[utoipa::path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 401, description = "Invalid token",),
    ),
    security(
        ("api-key" = []),
        ("bearer" = [])
    ),
    tag = "user"
)]
pub async fn me(Extension(user): Extension<User>) -> Result<Json<UserResponse>, StatusCode> {
    Ok(Json(UserResponse::from_domain(user)))
}