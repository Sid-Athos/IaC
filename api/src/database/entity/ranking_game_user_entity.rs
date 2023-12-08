use tokio_postgres::Row;

pub struct RankByGameEntity {
    pub rank: i32,
    pub nb_games: i32,
    pub pseudo : String,
    pub game : String
}

impl RankByGameEntity {
    pub fn new(row: Row) -> Self {
        RankByGameEntity {
            rank: row.get("rank"),
            nb_games : row.get("nb_games"),
            pseudo: row.get("pseudo"),
            game: row.get("name")
        }
    }
}
