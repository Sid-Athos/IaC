use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::user::route::request::login_request::LoginRequest;
use crate::entrypoint::user::route::response::session_response::SessionResponse;
use crate::service::command::login_command::LoginCommand;
use crate::service::user_service::UserService;

#[utoipa::path(
    post,
    path = "/user/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User connected", body = SessionResponse,),
        (status = 401, description = "Invalid password",),
        (status = 404, description = "User not found",),
    ),
    security(
        ("api-key" = [])
    ),
    tag = "user"
)]
pub async fn user_login(State(pool): State<ConnectionPool>, Json(user): Json<LoginRequest>) -> Result<Json<SessionResponse>, StatusCode> {
    tracing::info!("Calling login service for user {:?}", user);
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let session = user_service.login_user(LoginCommand::new(user)).await?;
    tracing::info!("Creating session for user {:?}", session);

    Ok(Json(SessionResponse::from_domain(session)))
}