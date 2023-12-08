use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::entrypoint::user::route::request::change_password_request::ChangePasswordRequest;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::command::change_password_command::ChangePasswordCommand;
use crate::service::user_service::UserService;

#[utoipa::path(
    put,
    path = "/user/change_password",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 401, description = "Invalid token or password",),
    ),
    security(
        ("api-key" = []),
        ("bearer" = [])
    ),
    request_body = ChangePasswordRequest,

    tag="user"
)]
pub async fn change_password(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Json(password): Json<ChangePasswordRequest>) -> Result<Json<UserResponse>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let command= ChangePasswordCommand::new(user.id, password);

    let user = user_service.change_password(command).await?;

    Ok(Json(UserResponse::from_domain(user)))
}