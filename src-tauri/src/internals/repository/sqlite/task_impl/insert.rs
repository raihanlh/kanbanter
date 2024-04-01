use sqlx::{Pool, Sqlite};

use crate::internals::{model::task::Task, repository::sqlite::queries::INSERT_TASK_QUERY};

pub async fn insert(db: &Pool<Sqlite>, task: Task) -> Box<Task> {
    let result = sqlx::query(
        INSERT_TASK_QUERY,
    )
    .bind(&task.title)
    .bind(&task.description)
    .bind(&task.board_id)
    .bind(&task.created_at)
    .bind(&task.updated_at)
    .bind(&task.position)
    .execute(db)
    .await
    .unwrap();

    let res: Box<Task> = Box::new(Task { 
        task_id: result.last_insert_rowid(), 
        title: task.title, 
        description: task.description, 
        board_id: task.board_id, 
        position: task.position,
        created_at: task.created_at, 
        updated_at: task.updated_at, 
        deleted_at: Option::None,
    });

    res
}