use sqlx::{Pool, Sqlite};

use crate::internals::{model::task::Task, repository::sqlite::{mapper::map_sqlresult_task, queries::GET_TASK_BY_ID_QUERY}};

pub async fn get_by_id(db: &Pool<Sqlite>, id: i64) -> Box<Task> {
    let result = sqlx::query(GET_TASK_BY_ID_QUERY)
        .bind(id)
        .fetch_one(db)
        .await
        .unwrap();
    
    let task = map_sqlresult_task(&result);

    task
}
