// use chrono::{DateTime, FixedOffset, Local, TimeZone};
use sqlx::{Pool, Sqlite};

use crate::internals::{model::board::Board, repository::sqlx::queries::UPDATE_BOARD_QUERY};

use super::get_by_id::get_by_id;

pub async fn update(db: &Pool<Sqlite>, board: Board) -> Box<Board> {
    let now = chrono::Local::now();
    sqlx::query(
        UPDATE_BOARD_QUERY,
    )
    .bind(&board.name)
    .bind(&board.description)
    .bind(now)
    .bind(&board.position)
    .bind(&board.board_id)
    .execute(db)
    .await
    .unwrap();

    get_by_id(db, board.board_id).await
}
