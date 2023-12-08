use crate::database::entity::games_entity::GamesEntity;

use crate::domain::model::games::Game;


pub struct GamesEntityMapper;

impl GamesEntityMapper {

    pub fn entity_to_domain(games_entity: GamesEntity) -> Game {
        Game {
            id: games_entity.id,
            min_players: games_entity.min_players,
            max_players: games_entity.max_players,
            language: games_entity.language,
            code: games_entity.code,
            description: games_entity.description,
            name: games_entity.name,
            user_id: games_entity.user_id,
        }
    }
}