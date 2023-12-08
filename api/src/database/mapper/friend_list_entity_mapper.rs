use crate::database::entity::friend_list_entity::FriendListEntity;
use crate::domain::model::friend_list::FriendList;

pub struct FriendListEntityMapper;

impl FriendListEntityMapper {

    pub fn entity_to_domain(friend_list_entity: FriendListEntity) -> FriendList {
        FriendList {
            id: friend_list_entity.id,
            applicant_id: friend_list_entity.applicant_id,
            recipient_id: friend_list_entity.recipient_id,
            accepted: friend_list_entity.accepted,
        }
    }
}