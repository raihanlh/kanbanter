use std::sync::Arc;

use async_trait::async_trait;

use crate::internals::{
    model::task::{GetTaskFilter, Task},
    repository::sqlite::repository::{BoardRepository, TaskRepository},
};

use super::task::TaskUsecase;

pub struct TaskUsecaseImpl {
    repo: Arc<Box<dyn TaskRepository + Send + Sync>>,
    repo_board: Arc<Box<dyn BoardRepository + Send + Sync>>,
}

impl TaskUsecaseImpl {
    pub fn new(
        repo: Arc<Box<dyn TaskRepository + Send + Sync>>,
        repo_board: Arc<Box<dyn BoardRepository + Send + Sync>>,
    ) -> Self {
        TaskUsecaseImpl { repo, repo_board }
    }
}

#[async_trait]
impl TaskUsecase for TaskUsecaseImpl {
    async fn create_new_task(&self, mut task: Task) -> Box<Task> {
        task.position = self.repo.get_highest_task_position(task.board_id).await;
        task.created_at = chrono::Local::now();
        task.updated_at = chrono::Local::now();
        self.repo.insert(task).await
    }
    async fn get_task_by_id(&self, id: i64) -> Box<Task> {
        self.repo.get_by_id(id).await
    }
    async fn get_all_tasks(&self, filter: GetTaskFilter) -> Vec<Box<Task>> {
        self.repo.get_all(filter).await
    }
    async fn update_task_by_id(&self, task: Task) -> Box<Task> {
        self.repo.update(task).await
    }
    async fn get_highest_task_position(&self, id: i64) -> i32 {
        self.repo.get_highest_task_position(id).await
    }
    async fn delete_task_by_id(&self, id: i64) -> bool {
        self.repo.delete(id).await
    }
    async fn archive_task_by_id(&self, id: i64) -> bool {
        self.repo.archive(id).await
    }
    async fn get_by_board_id(&self, board_id: i64, filter: GetTaskFilter) -> Vec<Box<Task>> {
        self.repo.get_by_board_id(board_id, filter).await
    }
    async fn unarchive_task_by_id(&self, id: i64) -> bool {
        let (_, board_id) = self.repo_board.get_lowest_board_position().await;
        self.repo.unarchive(id, board_id).await
    }
}
