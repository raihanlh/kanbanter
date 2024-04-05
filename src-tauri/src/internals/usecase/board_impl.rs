use std::sync::Arc;

use async_trait::async_trait;

use crate::internals::{model::board::Board, repository::sqlite::repository::BoardRepository};

use super::board::BoardUsecase;

pub struct BoardUsecaseImpl {
    repo: Arc<Box<dyn BoardRepository + Send + Sync>>,
}

impl BoardUsecaseImpl {
    pub fn new(repo: Arc<Box<dyn BoardRepository + Send + Sync>>) -> Self {
        BoardUsecaseImpl { repo }
    }
}

#[async_trait]
impl BoardUsecase for BoardUsecaseImpl {
    async fn create_new_board(&self, board: Board) -> Box<Board> {
        self.repo.insert(board).await
    }

    async fn get_board_by_id(&self, id: i64) -> Box<Board> {
        self.repo.get_by_id(id).await
    }

    async fn get_all_boards(&self) -> Vec<Box<Board>> {
        self.repo.get_all().await
    }
    async fn update_board_by_id(&self, board: Board) -> Box<Board> {
        self.repo.update(board).await
    }

    async fn get_highest_board_position(&self, id: i64) ->  i32 {
        self.repo.get_highest_board_position().await
    }

    async fn delete_board_by_id(&self, id: i64) ->  bool {
        self.repo.delete(id).await
    }
}