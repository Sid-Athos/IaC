use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::admin::route::request::update_user_request::UpdateUserRequest;
use crate::entrypoint::admin::route::response::user_response::UserResponse;
use crate::service::command::updata_user_command::UpdateUserCommand;
use crate::service::user_service::UserService;

#[utoipa::path(
    put,
    path = "/admin/{user_id}",
    params(
        ("user_id" = String,),
    ),
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
    tag="admin"
)]
pub async fn update_user(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>, Json(update_request): Json<UpdateUserRequest>) -> Result<Json<UserResponse>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let command = UpdateUserCommand::new(user_id, update_request.pseudo, update_request.email, update_request.password);

    let user = user_service.update_user(command).await?;

    Ok(Json(UserResponse::from_domain(user)))
}