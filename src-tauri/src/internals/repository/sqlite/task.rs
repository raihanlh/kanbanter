use sqlx::{Pool, Sqlite};

use crate::internals::model::task::Task;

use super::{repository::TaskRepository, task_impl::{get_all::get_all, get_by_id::get_by_id, insert::insert}};

pub struct TaskRepositoryImpl<'a> {
    db: &'a Pool<Sqlite>,
}

impl<'a> TaskRepositoryImpl<'a> {
    pub fn new(db: &'a Pool<Sqlite>) -> Self {
        TaskRepositoryImpl { db }
    }
}

impl<'a> TaskRepository for TaskRepositoryImpl<'a> {
    async fn insert(&self, task: Task) -> Box<Task> {
        insert(&self.db, task).await
    }

    async fn get_by_id(&self, id: i64) -> Box<Task> {
        get_by_id(&self.db, id).await
    }
    
    async fn get_all(&self) -> Vec<Box<Task>> {
        get_all(&self.db).await
    }


}