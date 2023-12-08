

use axum::{
    extract::{Path, State},
    Json,
};
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::user_service::UserService;


#[utoipa::path(
get,
path = "/user/search/{pseudo}",
responses(
(status = 200, description = "Todo marked done successfully"),
(status = 404, description = "Todo not found")
),
params(
("pseudo" = String, Path, description = "Todo database id")
),
security(
("api-key" = []),
("bearer" = [])
),
tag="user"
)]
pub async fn search_user(Path(pseudo): Path<String>, State(pool): State<ConnectionPool> ) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let users = user_service.search_user(pseudo).await?;

    Ok(Json(UserResponse::from_vec_domain(users)))
}

#[utoipa::path(
get,
path = "/user/other_players/{user_id}",
responses(
(status = 200, description = "Todo marked done successfully"),
(status = 404, description = "Todo not found")
),
params(
("user_id" = i32, Path, description = "Todo database id")
),
security(
("api-key" = []),
("bearer" = [])
),
tag="user"
)]
pub async fn get_other_players(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );


    let users = user_service.get_everyone_but_me(user_id).await?;

    Ok(Json(UserResponse::from_vec_domain(users)))
}
