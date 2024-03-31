use sqlx::{Pool, Sqlite};

use crate::internals::{model::board::Board, repository::sqlite::{mapper::map_sqlresult_board, queries::GET_ALL_BOARD_QUERY}};

pub async fn get_all(db: &Pool<Sqlite>) -> Vec<Box<Board>> {
    let result = 
      sqlx::query(GET_ALL_BOARD_QUERY).fetch_all(db).await.unwrap();

    let mut boards = Vec::default();
    for (_idx,row) in result.iter().enumerate() {
        boards.push(map_sqlresult_board(row))
    }

    boards
}
