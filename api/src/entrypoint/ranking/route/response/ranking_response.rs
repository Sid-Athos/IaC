use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::database::init::ConnectionPool;
use crate::database::repository::games_repository::GamesRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::games::Game;
use crate::domain::model::ranking::Ranking;
use crate::domain::model::user::User;
use crate::entrypoint::ranking::route::response::game_response::GameResponse;
use crate::entrypoint::ranking::route::response::user_response::UserResponse;
use crate::service::games_service::GamesService;
use crate::service::user_service::UserService;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RankingResponse {
    pub id: i32,
    pub rank : i32,
    pub user : UserResponse,
    pub game : GameResponse,
    pub nb_games : i32
}

impl RankingResponse {
    pub async fn from_domain(ranking: Ranking, pool: ConnectionPool) -> Self {

        let user_service = UserService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );
        let game_service = GamesService::new(
            GamesRepository::new(pool.clone())
        );

        RankingResponse {
            id: ranking.id,
            rank: ranking.rank,
            user: UserResponse::from_domain(user_service.get_user_by_id(ranking.user_id).await.unwrap()),
            game: GameResponse::from_domain(game_service.get_game_by_id(ranking.game_id).await.unwrap()),
            nb_games: ranking.nb_games
        }
    }

    pub async fn from_vec_domain(rankings: Vec<Ranking>, pool: ConnectionPool) -> Vec<Self> {
        let mut rankings_responses = Vec::new();

        for ranking in rankings {
            rankings_responses.push(RankingResponse::from_domain(ranking, pool.clone()).await);
        }

        return rankings_responses;
    }
}