use std::sync::Arc;

use async_trait::async_trait;

use crate::internals::{
    model::board::Board,
    repository::sqlite::repository::{BoardRepository, TaskRepository},
};

use super::board::BoardUsecase;

pub struct BoardUsecaseImpl {
    repo: Arc<Box<dyn BoardRepository + Send + Sync>>,
    repo_task: Arc<Box<dyn TaskRepository + Send + Sync>>,
}

impl BoardUsecaseImpl {
    pub fn new(
        repo: Arc<Box<dyn BoardRepository + Send + Sync>>,
        repo_task: Arc<Box<dyn TaskRepository + Send + Sync>>,
    ) -> Self {
        BoardUsecaseImpl { repo, repo_task }
    }
}

#[async_trait]
impl BoardUsecase for BoardUsecaseImpl {
    async fn create_new_board(&self, mut board: Board) -> Box<Board> {
        board.position = self.repo.get_highest_board_position().await;
        board.created_at = chrono::Local::now();
        board.updated_at = chrono::Local::now();
        self.repo.insert(board).await
    }

    async fn get_board_by_id(&self, id: i64) -> Box<Board> {
        self.repo.get_by_id(id).await
    }

    async fn get_all_boards(&self) -> Vec<Box<Board>> {
        let mut boards = self.repo.get_all().await;

        for (idx, board) in boards.clone().iter().enumerate() {
            let tasks = self.repo_task.get_by_board_id(board.board_id).await;
            boards[idx].tasks = tasks.clone();
        }

        boards
    }
    async fn update_board_by_id(&self, board: Board) -> Box<Board> {
        self.repo.update(board).await
    }

    async fn get_highest_board_position(&self) -> i32 {
        self.repo.get_highest_board_position().await
    }

    async fn delete_board_by_id(&self, id: i64) -> bool {
        self.repo.delete(id).await
    }

    async fn archive_board_by_id(&self, id: i64) ->  bool {
        self.repo.archive(id).await
    }
}
