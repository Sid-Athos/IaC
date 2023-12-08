use tokio_postgres::Row;

#[derive(Debug)]
pub struct FriendListEntity {
    pub id: i32,
    pub applicant_id: i32,
    pub recipient_id: i32,
    pub accepted: bool,
}

impl FriendListEntity {
    pub fn new(row: Row) -> Self {
        FriendListEntity {
            id : row.get("id"),
            applicant_id: row.get("applicant_id"),
            recipient_id: row.get("recipient_id"),
            accepted: row.get("accepted"),
        }
    }
}