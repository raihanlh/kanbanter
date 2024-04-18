use sqlx::{Pool, Sqlite};

use crate::internals::{
    model::task::{GetTaskFilter, Task},
    repository::sqlite::{mapper::map_sqlresult_task, queries::GET_TASKS_BY_BOARD_ID_QUERY},
};

pub async fn get_by_board_id(db: &Pool<Sqlite>, board_id: i64, filter: GetTaskFilter) -> Vec<Box<Task>> {
    let result = sqlx::query(GET_TASKS_BY_BOARD_ID_QUERY)
        .bind(board_id)
        .fetch_all(db)
        .await
        .unwrap();

    let mut tasks = Vec::default();
    for (_idx, row) in result.iter().enumerate() {
        tasks.push(map_sqlresult_task(row))
    }

    tasks
}
