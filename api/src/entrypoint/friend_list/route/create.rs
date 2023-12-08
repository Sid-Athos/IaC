use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::entrypoint::friend_list::route::response::user_response::UserResponse;
use crate::entrypoint::friend_list::route::request::create_friend_list_request::CreateFriendListRequest;
use crate::entrypoint::friend_list::route::response::friend_list_response::FriendListResponse;
use crate::service::friend_list_service::FriendListService;
use crate::service::user_service::UserService;

#[utoipa::path(
    post,
    path = "/friend_list/",
    responses(
        (status = 200, description = "FriendList entry created", body = FriendListResponse,),
        (status = 401, description = "Invalid token",),
    ),
    request_body = CreateFriendListRequest,
    security(
    ("api-key" = []),
    ("bearer" = [])
    ),
    tag="friend_list"
)]
pub async fn friend_list_create(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Json(friend_list): Json<CreateFriendListRequest>) -> Result<Json<FriendListResponse>, StatusCode> {
    println!("lol");
    let friend_list_service = FriendListService::new(
        FriendListRepository::new(pool.clone()),
    );
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone()),
    );

    let friend_list = friend_list_service.create_friend_request(friend_list.recipient_id,user.id).await?;
    let recipient = UserResponse::from_domain(user_service.get_user_by_id(friend_list.recipient_id).await?);
    let applicant = UserResponse::from_domain(user_service.get_user_by_id(friend_list.applicant_id).await?);

    Ok(Json(FriendListResponse::from_domain(friend_list, applicant, recipient)))
}