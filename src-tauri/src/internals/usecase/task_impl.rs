use std::sync::Arc;

use async_trait::async_trait;

use crate::internals::{model::task::Task, repository::sqlite::repository::TaskRepository};

use super::task::TaskUsecase;

pub struct TaskUsecaseImpl {
  repo: Arc<Box<dyn TaskRepository + Send + Sync>>,
}

impl TaskUsecaseImpl {
  pub fn new(repo: Arc<Box<dyn TaskRepository + Send + Sync>>) -> Self {
      TaskUsecaseImpl { repo }
  }
}

#[async_trait]
impl TaskUsecase for TaskUsecaseImpl {
    async fn create_new_task(&self, task: Task) -> Box<Task> {
      self.repo.insert(task).await
    }
    async fn get_task_by_id(&self, id: i64) -> Box<Task> {
      self.repo.get_by_id(id).await
    }
    async fn get_all_tasks(&self) -> Vec<Box<Task>> {
      self.repo.get_all().await
    }
    // async fn get_all_tasks_by_board_id(&self, board_id: i64) -> Vec<Box<Task>> {
    //   self.repo.get_all_by_board_id(board_id)
    // }
    async fn update_task_by_id(&self, task: Task) -> Box<Task> {
      self.repo.update(task).await
    }
    async fn get_highest_task_position(&self, id: i64) -> i32 {
      self.repo.get_highest_task_position(id).await
    }
    async fn delete_task_by_id(&self, id: i64) -> bool {
      self.repo.delete(id).await
    }
}