use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::user::User;

use crate::entrypoint::friend_list::route::response::friend_list_response::FriendListResponse;
use crate::entrypoint::friend_list::route::response::user_response::UserResponse;
use crate::service::friend_list_service::FriendListService;
use crate::service::user_service::UserService;

#[utoipa::path(
    get,
    path = "/friend_list/requests",
    responses(
        (status = 200, description = "Friends requests found", body = Vec<UserResponse>),
        (status = 401, description = "Invalid token",),
    ),
security(
("api-key" = []),
("bearer" = [])
),
    tag="friend_list"
)]
pub async fn show_friend_request(State(pool): State<ConnectionPool>, Extension(user): Extension<User>) -> Result<Json<Vec<FriendListResponse>>, StatusCode> {
    let friend_list_service = FriendListService::new(
        FriendListRepository::new(pool.clone()),
    );
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone()),
    );

    let result = friend_list_service.get_all_requests(user.id).await?;

    let mut responses = Vec::new();
    for friend in result {
        let recipient = UserResponse::from_domain(user_service.get_user_by_id(friend.recipient_id).await?);
        let applicant = UserResponse::from_domain(user_service.get_user_by_id(friend.applicant_id).await?);
        responses.push(FriendListResponse::from_domain(friend, applicant, recipient))
    }

    Ok(Json(responses))
}