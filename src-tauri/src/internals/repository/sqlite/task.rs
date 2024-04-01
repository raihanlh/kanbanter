use sqlx::{Pool, Sqlite};

use crate::internals::model::task::Task;

use super::{repository::TaskRepository, task_impl::insert::insert};

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

}