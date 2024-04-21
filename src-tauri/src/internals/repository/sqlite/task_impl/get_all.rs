use sqlx::{Pool, Sqlite};

use crate::internals::{
    model::task::{GetTaskFilter, Task},
    repository::sqlite::{mapper::map_sqlresult_task, queries::GET_ALL_TASK_QUERY},
};

pub async fn get_all(db: &Pool<Sqlite>, filter: GetTaskFilter) -> Vec<Box<Task>> {
    let mut query = GET_ALL_TASK_QUERY.to_string().to_owned();
    if filter.is_archived {
        query.push_str(" AND deleted_at IS NOT NULL")
    } else {
        query.push_str(" AND deleted_at IS NULL")
    }

    query.push_str(" ORDER BY position ASC");
    let result = sqlx::query(query.as_str()).fetch_all(db).await.unwrap();

    let mut tasks = Vec::default();
    for (_idx, row) in result.iter().enumerate() {
        tasks.push(map_sqlresult_task(row))
    }

    tasks
}
