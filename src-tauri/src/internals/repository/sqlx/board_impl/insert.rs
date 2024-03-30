// use chrono::{DateTime, FixedOffset, Local, TimeZone};
use sqlx::{Pool, Sqlite};

use crate::internals::{model::board::Board, repository::sqlx::queries::INSERT_BOARD_QUERY};

pub async fn insert(db: &Pool<Sqlite>, board: Board) -> Box<Board> {
    let result = sqlx::query(
        INSERT_BOARD_QUERY,
    )
    .bind(&board.name)
    .bind(&board.description)
    .bind(&board.created_at)
    .bind(&board.updated_at)
    .execute(db)
    .await
    .unwrap();

    let res: Box<Board> = Box::new(Board { 
        board_id: result.last_insert_rowid(), 
        name: board.name.clone(), 
        description: board.description.clone(), 
        created_at: board.created_at, 
        updated_at: board.updated_at, 
        deleted_at: Option::None 
    });

    res
}