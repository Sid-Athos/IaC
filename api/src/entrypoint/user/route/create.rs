use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::user_service::UserService;

#[utoipa::path(
    post,
    path = "/user/create",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created", body = UserResponse,),
        (status = 409, description = "User email already exist",),
    ),
    security(
        ("api-key" = [])
    ),
    tag = "user"
)]
pub async fn user_create(State(pool): State<ConnectionPool>, Json(user): Json<CreateUserRequest>) -> Result<Json<UserResponse>, StatusCode> {
    tracing::info!("Received create user request {:?}", user);
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.create_user(CreateUserCommand::new(user)).await?;

    Ok(Json(UserResponse::from_domain(user)))
}