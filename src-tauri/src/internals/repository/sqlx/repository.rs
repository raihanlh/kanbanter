use crate::internals::model::{board::Board, res_data::ResData, task::Task};

pub trait ResDataRepository {
    fn get_all_data(&self) -> Vec<ResData>;
}

pub trait BoardRepository {
    fn insert(&self, board: Board) -> impl std::future::Future<Output = Box<Board>> + Send;
    fn get_by_id(&self, id: i64) -> impl std::future::Future<Output = Box<Board>> + Send;
    fn get_all(&self) -> impl std::future::Future<Output = Vec<Box<Board>>> + Send;
    fn update(&self, board: Board) -> impl std::future::Future<Output = Box<Board>> + Send;
    fn get_highest_board_position(&self) -> impl std::future::Future<Output = i32> + Send;
}

pub trait TaskRepository {
    fn insert(&self, task: Task) -> impl std::future::Future<Output = Task> + Send;
    fn get_by_id(&self, id: i64) -> impl std::future::Future<Output = Task> + Send;
    fn get_all(&self) -> impl std::future::Future<Output = Vec<Task>> + Send;
}