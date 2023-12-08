use crate::database::entity::ranking_game_user_entity::RankByGameEntity;
use crate::domain::model::ranking::Ranking;
use crate::domain::model::ranking_by_game::RankingByGame;

pub struct RankingByGameEntityMapper;

impl RankingByGameEntityMapper {

    pub fn entity_to_domain(entity: RankByGameEntity) -> RankingByGame {
        RankingByGame {
            rank: entity.rank,
            nb_games : entity.nb_games,
            pseudo: entity.pseudo,
            game: entity.game
        }
    }
}