use sqlx::{Pool, Sqlite};

use crate::internals::{model::board::Board, repository::sqlx::{mapper::map_sqlresult_board, queries::GET_BOARD_BY_ID_QUERY}};

pub async fn get_by_id(db: &Pool<Sqlite>, id: i64) -> Box<Board> {
    let result = sqlx::query(GET_BOARD_BY_ID_QUERY)
        .bind(id)
        .fetch_one(db)
        .await
        .unwrap();

    let board = map_sqlresult_board(&result);

    board
}
