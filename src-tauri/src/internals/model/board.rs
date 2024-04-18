use serde::{Deserialize, Serialize};

use super::task::Task;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Board {
    pub board_id: i64,
    pub name: String,
    pub description: String,
    pub position: i32,
    pub tasks: Vec<Box<Task>>,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub deleted_at: Option<chrono::DateTime<chrono::Local>>,
}

pub struct GetAllBoardFilter {
    pub is_archived: bool,
}