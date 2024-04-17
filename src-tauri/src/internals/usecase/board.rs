use async_trait::async_trait;

use crate::internals::model::board::Board;

#[async_trait]
pub trait BoardUsecase {
    async fn create_new_board(&self, board: Board) -> Box<Board>;
    async fn get_board_by_id(&self, id: i64) -> Box<Board>;
    async fn get_all_boards(&self) -> Vec<Box<Board>>;
    async fn update_board_by_id(&self, board: Board) -> Box<Board>;
    async fn get_highest_board_position(&self) ->  i32;
    async fn delete_board_by_id(&self, id: i64) ->  bool;
    async fn archive_board_by_id(&self, id: i64) ->  bool;
}