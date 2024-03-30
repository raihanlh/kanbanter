use chrono::{DateTime, Local};
use sqlx::{Pool, Sqlite, Row};

use crate::internals::{model::board::Board, repository::sqlx::queries::GET_ALL_BOARD_QUERY};

pub async fn get_all(db: &Pool<Sqlite>) -> Vec<Box<Board>> {
    let result = 
      sqlx::query(GET_ALL_BOARD_QUERY).fetch_all(db).await.unwrap();

    let mut boards = Vec::default();
    for (_idx, row) in result.iter().enumerate() {
        boards.push(Box::new(Board {
          board_id: row.get::<i64, &str>("board_id"),
          name: row.get::<String, &str>("name"),
          description: row.get::<String, &str>("description"),
          created_at: row.get::<DateTime<Local>, &str>("created_at"),
          updated_at: row.get::<DateTime<Local>, &str>("updated_at"),
          deleted_at: Option::None
        }))
    }

    boards
}
