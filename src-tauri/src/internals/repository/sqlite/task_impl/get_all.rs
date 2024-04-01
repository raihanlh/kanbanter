use sqlx::{Pool, Sqlite};

use crate::internals::{model::task::Task, repository::sqlite::{mapper::map_sqlresult_task, queries::GET_ALL_TASK_QUERY}};

pub async fn get_all(db: &Pool<Sqlite>) -> Vec<Box<Task>> {
    let result = 
      sqlx::query(GET_ALL_TASK_QUERY).fetch_all(db).await.unwrap();

    let mut tasks = Vec::default();
    for (_idx,row) in result.iter().enumerate() {
        tasks.push(map_sqlresult_task(row))
    }

    tasks
}
