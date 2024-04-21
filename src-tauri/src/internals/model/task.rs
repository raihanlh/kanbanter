use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Task {
    pub task_id: i64,
    pub title: String,
    pub description: String,
    pub board_id: i64,
    pub position: i32,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub deleted_at: Option<chrono::DateTime<chrono::Local>>,
}


#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct GetTaskFilter {
    pub is_archived: bool,
}