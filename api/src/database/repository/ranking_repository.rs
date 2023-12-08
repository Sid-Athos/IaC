use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_duplicate_key, database_error_not_found, DatabaseError};
use crate::database::entity::ranking_entity::RankingEntity;
use crate::database::entity::ranking_game_user_entity::RankByGameEntity;
use crate::database::mapper::ranking_by_game_entity_mapper::RankingByGameEntityMapper;
use crate::database::mapper::ranking_entity_mapper::RankingEntityMapper;
use crate::domain::model::ranking::Ranking;
use crate::domain::model::ranking_by_game::RankingByGame;

pub struct RankingRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl RankingRepository {
    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        RankingRepository {
            connection
        }
    }

    pub async fn create_ranking(&self, game_id: i32, user_id: i32) -> Result<Ranking, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.ranking (game_id, user_id) VALUES ($1, $2) RETURNING *", &[&game_id, &user_id])
            .await
            .map_err(database_error_duplicate_key)?;

        let result = RankingEntity::new(row);

        Ok(RankingEntityMapper::entity_to_domain(result))
    }

    pub async fn get_one_ranking(&self, game_id: i32, user_id: i32) -> Result<Ranking, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT r.* FROM coding_games.ranking r WHERE game_id = $1 AND user_id = $2", &[&game_id, &user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = RankingEntity::new(row);

        Ok(RankingEntityMapper::entity_to_domain(result))
    }

    pub async fn get_all_rankings_by_game(&self) -> Result<Vec<RankingByGame>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT r.rank, r.nb_games, u.pseudo, g.name from coding_games.ranking as r JOIN coding_games.user as u on r.user_id = u.id JOIN coding_games.game as g on r.game_id = g.id ORDER BY r.game_id, r.rank DESC;", &[])
            .await
            .map_err(database_error_duplicate_key)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingByGameEntityMapper::entity_to_domain(RankByGameEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn update_ranking(&self, game_id: i32, user_id: i32, new_ranking : i32) -> Result<Ranking, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("UPDATE coding_games.ranking SET rank = $1,nb_games = nb_games + 1 WHERE game_id = $2 AND user_id = $3 RETURNING *", &[&new_ranking,&game_id, &user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = RankingEntity::new(row);

        Ok(RankingEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_rankings_by_user_id(&self, user_id: i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("DELETE FROM coding_games.ranking WHERE user_id = $1 RETURNING *", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn delete_rankings_by_game_id(&self,  game_id: i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("DELETE FROM coding_games.ranking WHERE game_id = $1 RETURNING *", &[&game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_ranking_by_friends(&self, user_id: i32, game_id : i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT r.*
                        FROM coding_games.ranking r
                 JOIN coding_games.user u ON u.id = r.user_id
                 JOIN coding_games.game g ON g.id = r.game_id
                    WHERE u.id IN (
                        SELECT DISTINCT CASE
                                            WHEN fl.applicant_id = $1 THEN fl.recipient_id
                                            WHEN fl.recipient_id = $1 THEN fl.applicant_id
                                            END AS friend_id
                        FROM coding_games.friend_list fl
                        WHERE fl.accepted = true AND (fl.applicant_id = $1 OR fl.recipient_id = $1)
                        )
                      AND g.id = $2 ORDER BY r.rank DESC", &[&user_id,&game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_ranking_by_game(&self, game_id : i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT r.* FROM coding_games.ranking r WHERE game_id = $1 ORDER BY rank DESC", &[&game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_ranking_by_user(&self, user_id : i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT r.* FROM coding_games.ranking r WHERE user_id = $1 ORDER BY rank DESC", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }



}