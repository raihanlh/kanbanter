use sqlx::{Pool, Sqlite};

use crate::internals::{model::task::Task, repository::sqlite::queries::UPDATE_TASK_QUERY};

use super::get_by_id::get_by_id;

pub async fn update(db: &Pool<Sqlite>, task: Task) -> Box<Task> {
    let now = chrono::Local::now();
    sqlx::query(
        UPDATE_TASK_QUERY,
    )
    .bind(&task.title)
    .bind(&task.description)
    .bind(&task.board_id)
    .bind(now)
    .bind(&task.position)
    .execute(db)
    .await
    .unwrap();

    get_by_id(db, task.task_id).await
}
