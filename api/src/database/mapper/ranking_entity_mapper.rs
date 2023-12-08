use crate::database::entity::ranking_entity::RankingEntity;
use crate::domain::model::ranking::Ranking;

pub struct RankingEntityMapper;

impl RankingEntityMapper {

    pub fn entity_to_domain(entity: RankingEntity) -> Ranking {
        Ranking {
            id: entity.id,
            user_id: entity.user_id,
            game_id: entity.game_id,
            rank: entity.rank,
            nb_games : entity.nb_games
        }
    }
}