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
}