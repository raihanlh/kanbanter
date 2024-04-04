use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::internals::model::task::Task;

use super::{repository::TaskRepository, task_impl::{delete::delete, get_all::get_all, get_by_id::get_by_id, get_highest_task_position::get_highest_task_position, insert::insert, update::update}};

#[derive(Clone)]
pub struct TaskRepositoryImpl {
    db: Pool<Sqlite>,
}

impl TaskRepositoryImpl {
    pub fn new(db: Pool<Sqlite>) -> Self {
        TaskRepositoryImpl { db }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn insert(&self, task: Task) -> Box<Task> {
        insert(&self.db, task).await
    }

    async fn get_by_id(&self, id: i64) -> Box<Task> {
        get_by_id(&self.db, id).await
    }
    
    async fn get_all(&self) -> Vec<Box<Task>> {
        get_all(&self.db).await
    }

    async fn get_highest_task_position(&self, board_id: i64) -> i32 {
        get_highest_task_position(&self.db, board_id).await
    }

    async fn update(&self, task: Task) -> Box<Task> {
        update(&self.db, task).await
    }

    async fn delete(&self, id: i64) -> bool {
        delete(&self.db, id).await
    }
}