use async_trait::async_trait;

use crate::internals::model::{board::{Board, GetAllBoardFilter}, res_data::ResData, task::{GetTaskFilter, Task}};

pub trait ResDataRepository {
    fn get_all_data(&self) -> Vec<ResData>;
}

#[async_trait]
pub trait BoardRepository {
    async fn insert(&self, board: Board) -> Box<Board>;
    async fn get_by_id(&self, id: i64) -> Box<Board>;
    async fn get_all(&self, filter: GetAllBoardFilter) -> Vec<Box<Board>>;
    async fn update(&self, board: Board) -> Box<Board>;
    async fn get_highest_board_position(&self) -> i32;
    async fn delete(&self, id: i64) -> bool;
    async fn archive(&self, id: i64) -> bool;
    async fn get_lowest_board_position(&self) -> (i32, i64);
}

#[async_trait]
pub trait TaskRepository {
    async fn insert(&self, task: Task) -> Box<Task>;
    async fn get_by_id(&self, id: i64) -> Box<Task>;
    async fn get_all(&self, filter: GetTaskFilter) -> Vec<Box<Task>>;
    async fn get_highest_task_position(&self, board_id: i64) -> i32;
    async fn update(&self, task: Task) -> Box<Task>;
    async fn delete(&self, id: i64) -> bool;
    async fn get_by_board_id(&self, board_id: i64, filter: GetTaskFilter) -> Vec<Box<Task>>;
    async fn archive(&self, id: i64) -> bool;
    async fn unarchive(&self, id: i64, destination_board_id: i64) -> bool;
}
