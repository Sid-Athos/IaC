

use axum::http::StatusCode;



use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::domain::error::{database_error_to_status_code};
use crate::domain::model::friend_list::FriendList;

pub struct FriendListService {
    pub friend_list_repository: FriendListRepository
}

impl FriendListService {
    pub fn new(friend_list_repository: FriendListRepository) -> Self {
        FriendListService {
            friend_list_repository
        }
    }

    pub async fn create_friend_request(&self, recipient_id: i32, applicant_id: i32) -> Result<FriendList, StatusCode> {
        if recipient_id == applicant_id {
            return Err(StatusCode::BAD_REQUEST);
        }

        let exist = self.friend_list_repository.get_friend_list_request_by_users(applicant_id, recipient_id).await.map_err(database_error_to_status_code);

        return if exist.is_ok() {
            if exist.clone()?.recipient_id == applicant_id && exist.clone()?.accepted == false {
                return self.friend_list_repository.accept_friend_list_request(exist?.id,applicant_id).await.map_err(database_error_to_status_code)
            };
            Err(StatusCode::BAD_REQUEST)
        } else {
            self.friend_list_repository.create_friend_list_request(recipient_id, applicant_id).await.map_err(database_error_to_status_code)
        }
    }

    pub async fn delete_friend(&self, request_id: i32, user_id: i32) -> Result<FriendList, StatusCode> {
        self.friend_list_repository.delete_friend_list_request(request_id, user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn accept_friend_request(&self, request_id : i32, user_id : i32) -> Result<FriendList, StatusCode> {
        self.friend_list_repository.accept_friend_list_request(request_id, user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_all_friends(&self, user_id : i32) -> Result<Vec<FriendList>, StatusCode> {
        self.friend_list_repository.get_friend_list_by_user( user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_all_requests(&self, user_id : i32) -> Result<Vec<FriendList>, StatusCode> {
        self.friend_list_repository.get_friend_list_requests(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_request(&self, user_id1: i32, user_id2: i32) -> Result<FriendList, StatusCode> {
        self.friend_list_repository.get_friend_list_request_by_users_id(user_id1, user_id2).await.map_err(database_error_to_status_code)
    }
}