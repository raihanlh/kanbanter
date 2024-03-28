use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResData {
    pub id: i64,
    pub content: String,
}