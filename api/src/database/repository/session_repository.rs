use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_duplicate_key, database_error_not_found, DatabaseError};
use crate::database::entity::session_entity::SessionEntity;
use crate::database::mapper::session_entity_mapper::SessionEntityMapper;
use crate::domain::model::session::Session;

pub struct SessionRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl SessionRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        SessionRepository {
            connection
        }
    }

    pub async fn get_by_token(&self, token: String) -> Result<Session, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.session WHERE token = $1", &[&token])
            .await
            .map_err(database_error_not_found)?;

        let result = SessionEntity::new(row);

        Ok(SessionEntityMapper::entity_to_domain(result))
    }

    pub async fn create_session(&self, user_id: i32, token: String) -> Result<Session, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.session (user_id, token) VALUES ($1, $2) RETURNING *", &[&user_id, &token])
            .await
            .map_err(database_error_duplicate_key)?;

        let result = SessionEntity::new(row);

        Ok(SessionEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_token(&self, token: String) -> Result<Session, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("DELETE FROM coding_games.session WHERE token = $1 RETURNING *", &[&token])
            .await
            .map_err(database_error_not_found)?;

        let result = SessionEntity::new(row);

        Ok(SessionEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_all_for_user(&self, user_id: i32) -> Result<Vec<Session>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn.query("DELETE FROM coding_games.session WHERE user_id = $1 RETURNING *", &[&user_id])
                            .await
                            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(SessionEntityMapper::entity_to_domain(SessionEntity::new(row)));
        }

        Ok(result)
    }
}
