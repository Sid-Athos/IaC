use crate::database::entity::session_entity::SessionEntity;
use crate::domain::model::session::Session;

pub struct SessionEntityMapper;

impl SessionEntityMapper {

    pub fn entity_to_domain(entity: SessionEntity) -> Session {
        Session {
            id: entity.id,
            user_id: entity.user_id,
            token: entity.token,
        }
    }
}