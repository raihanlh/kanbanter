use chrono::{DateTime, Local};
use sqlx::{sqlite::SqliteRow, Row};

use crate::internals::model::{board::Board, task::Task};

pub fn map_sqlresult_board(row: &SqliteRow) -> Box<Board> {
    Box::new(Board {
        board_id: row.get::<i64, &str>("board_id"),
        name: row.get::<String, &str>("name"),
        description: row.get::<String, &str>("description"),
        position: row.get::<i32, &str>("position"),
        created_at: row.get::<DateTime<Local>, &str>("created_at"),
        tasks: vec![],
        updated_at: row.get::<DateTime<Local>, &str>("updated_at"),
        deleted_at: Option::None,
    })
}

pub fn map_sqlresult_task(row: &SqliteRow) -> Box<Task> {
    Box::new(Task {
        task_id: row.get::<i64, &str>("task_id"),
        board_id: row.get::<i64, &str>("board_id"),
        title: row.get::<String, &str>("title"),
        description: row.get::<String, &str>("description"),
        position: row.get::<i32, &str>("position"),
        created_at: row.get::<DateTime<Local>, &str>("created_at"),
        updated_at: row.get::<DateTime<Local>, &str>("updated_at"),
        deleted_at: Option::None,
    })
}
