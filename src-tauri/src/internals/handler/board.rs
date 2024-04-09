
// use std::sync::Arc;

// use tauri::{command, Result};

// use crate::internals::{model::board::Board, usecase::board::BoardUsecase};

// #[command]
// pub async fn create_new_board(
//     board_usecase: Arc<Box<dyn BoardUsecase + Send + Sync>>,
//     board: Board,
// ) -> Result<Box<Board>> {
//     Ok(board_usecase.create_new_board(board).await)
// }

// #[command]
// pub async fn get_board_by_id(
//     board_usecase: Arc<Box<dyn BoardUsecase + Send + Sync>>,
//     id: i64,
// ) -> Result<Box<Board>> {
//     Ok(board_usecase.get_board_by_id(id).await)
// }

// #[command]
// pub async fn get_all_boards(
//     board_usecase: Arc<Box<dyn BoardUsecase + Send + Sync>>,
// ) -> Result<Vec<Box<Board>>> {
//     Ok(board_usecase.get_all_boards().await)
// }

// #[command]
// pub async fn update_board_by_id(&self, board: Board) -> Box<Board> {}

// #[command]
// pub async fn get_highest_board_position(&self, id: i64) -> i32 {}

// #[command]
// pub async fn delete_board_by_id(&self, id: i64) -> bool {}
