use async_trait::async_trait;

use crate::internals::model::board::Board;

#[async_trait]
pub trait BoardUsecase {
    async fn create_new_board(&self, board: Board) -> Box<Board>;
}