use async_trait::async_trait;

use crate::internals::model::task::{GetTaskFilter, Task};

#[async_trait]
pub trait TaskUsecase {
    async fn create_new_task(&self, task: Task) -> Box<Task>;
    async fn get_task_by_id(&self, id: i64) -> Box<Task>;
    async fn get_all_tasks(&self, filter: GetTaskFilter) -> Vec<Box<Task>>;
    async fn get_by_board_id(&self, board_id: i64, filter: GetTaskFilter) -> Vec<Box<Task>>;
    async fn update_task_by_id(&self, task: Task) -> Box<Task>;
    async fn get_highest_task_position(&self, id: i64) ->  i32;
    async fn delete_task_by_id(&self, id: i64) ->  bool;
    async fn archive_task_by_id(&self, id: i64) ->  bool;
    async fn unarchive_task_by_id(&self, id: i64) ->  bool;
}