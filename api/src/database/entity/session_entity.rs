use tokio_postgres::Row;

pub struct SessionEntity {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

impl SessionEntity {
    pub fn new(row: Row) -> Self {
        SessionEntity {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
        }
    }
}
