use chrono::{DateTime, Local};
use sqlx::{Pool, Row, Sqlite};

use crate::internals::{model::board::Board, repository::sqlx::queries::GET_BOARD_BY_ID_QUERY};

pub async fn get_by_id(db: &Pool<Sqlite>, id: i64) -> Box<Board> {
    let result = sqlx::query(GET_BOARD_BY_ID_QUERY)
        .bind(id)
        .fetch_one(db)
        .await
        .unwrap();

    let board = Box::new(Board {
        board_id: result.get::<i64, &str>("board_id"),
        name: result.get::<String, &str>("name"),
        description: result.get::<String, &str>("description"),
        created_at: result.get::<DateTime<Local>, &str>("created_at"),
        updated_at: result.get::<DateTime<Local>, &str>("updated_at"),
        deleted_at: Option::None,
    });

    board
}
