use sqlx::{Pool, Sqlite};

use crate::internals::{model::board::{Board, GetAllBoardFilter}, repository::sqlite::{mapper::map_sqlresult_board, queries::GET_ALL_BOARD_QUERY}};

pub async fn get_all(db: &Pool<Sqlite>, filter: GetAllBoardFilter) -> Vec<Box<Board>> {
  let mut query = GET_ALL_BOARD_QUERY.to_string().to_owned();
    if filter.is_archived {
        query.push_str(" AND deleted_at IS NOT NULL")
    } else {
        query.push_str(" AND deleted_at IS NULL")
    }
    query.push_str(" ORDER BY position ASC");

    let result = 
      sqlx::query(GET_ALL_BOARD_QUERY).fetch_all(db).await.unwrap();

    let mut boards = Vec::default();
    for (_idx,row) in result.iter().enumerate() {
        boards.push(map_sqlresult_board(row))
    }

    boards
}
