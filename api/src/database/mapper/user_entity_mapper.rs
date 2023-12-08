use crate::database::entity::user_entity::UserEntity;
use crate::domain::model::user::User;

pub struct UserEntityMapper;

impl UserEntityMapper {

    pub fn entity_to_domain(user_entity: UserEntity) -> User {
        User {
            id: user_entity.id,
            pseudo: user_entity.pseudo,
            email: user_entity.email,
            password: user_entity.password,
            admin: user_entity.admin,
            experience: user_entity.experience,
            level: user_entity.level
        }
    }
}