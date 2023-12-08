use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_duplicate_key, database_error_not_found, DatabaseError};
use crate::database::entity::friend_list_entity::FriendListEntity;
use crate::domain::model::friend_list::FriendList;
use crate::database::mapper::friend_list_entity_mapper::FriendListEntityMapper;

pub struct FriendListRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl FriendListRepository {
    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        FriendListRepository {
            connection
        }
    }

    pub async fn get_friend_list_requests(&self, user_id: i32) -> Result<Vec<FriendList>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.friend_list WHERE ( recipient_id = $1 OR applicant_id = $1 ) AND accepted = false", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(FriendListEntityMapper::entity_to_domain(FriendListEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn create_friend_list_request(&self, recipient_id: i32, applicant_id: i32) -> Result<FriendList, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.friend_list (recipient_id, applicant_id) VALUES ($1, $2) RETURNING *", &[&recipient_id, &applicant_id])
            .await
            .map_err(database_error_duplicate_key)?;

        let result = FriendListEntity::new(row);

        Ok(FriendListEntityMapper::entity_to_domain(result))
    }

    pub async fn accept_friend_list_request(&self, request_id: i32, user_id : i32) -> Result<FriendList, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("UPDATE coding_games.friend_list SET accepted = true WHERE id = $1 AND recipient_id = $2 AND accepted = false RETURNING *", &[&request_id, &user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = FriendListEntity::new(row);

        Ok(FriendListEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_friend_list_request(&self, request_id: i32, user_id : i32) -> Result<FriendList, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("DELETE FROM coding_games.friend_list WHERE id = $1 AND ( applicant_id = $2 OR recipient_id = $2 ) RETURNING *", &[&request_id,&user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = FriendListEntity::new(row);

        Ok(FriendListEntityMapper::entity_to_domain(result))
    }

    pub async fn get_friend_list_request_by_users(&self, user_1: i32, user_2 : i32) -> Result<FriendList, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.friend_list WHERE (applicant_id = $1 AND recipient_id = $2) OR (applicant_id = $2 AND recipient_id = $1)", &[&user_1, &user_2])
            .await
            .map_err(database_error_not_found)?;

        let result = FriendListEntity::new(row);
        Ok(FriendListEntityMapper::entity_to_domain(result))
    }

    pub async fn get_friend_list_request_by_users_id(&self, user_id1: i32, user_id2 : i32) -> Result<FriendList, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.friend_list WHERE (applicant_id = $1 AND recipient_id = $2) OR (applicant_id = $2 AND recipient_id = $1)", &[&user_id1,&user_id2])
            .await
            .map_err(database_error_not_found)?;

        let result = FriendListEntity::new(row);
        Ok(FriendListEntityMapper::entity_to_domain(result))
    }

    pub async fn get_friend_list_by_user(&self, user_id: i32) -> Result<Vec<FriendList>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.friend_list WHERE ( recipient_id = $1 OR applicant_id = $1 ) AND accepted = true", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(FriendListEntityMapper::entity_to_domain(FriendListEntity::new(row)));
        }

        Ok(result)
    }





}