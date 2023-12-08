use tokio_postgres::Row;

pub struct GamesEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub min_players: i32,
    pub max_players: i32,
    pub language : String,
    pub code : Option<String>,
    pub user_id : i32
}

impl GamesEntity {
    pub fn new(row: Row) -> Self {
        GamesEntity {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            min_players: row.get("min_players"),
            max_players: row.get("max_players"),
            language: row.get("language"),
            code : row.get("code"),
            user_id : row.get("user_id")
        }
    }
    pub fn new_without_code(row : Row) -> Self {
        GamesEntity {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            min_players: row.get("min_players"),
            max_players: row.get("max_players"),
            language: row.get("language"),
            code : None,
            user_id : row.get("user_id")
        }
    }
}
