use sqlx::{Pool, Sqlite};

use super::{
    board_impl::{get_all::get_all, get_by_id::get_by_id, insert::insert},
    repository::BoardRepository,
};
use crate::internals::model::board::Board;

pub struct BoardRepositoryImpl<'a> {
    db: &'a Pool<Sqlite>,
}

impl<'a> BoardRepositoryImpl<'a> {
    pub fn new(db: &'a Pool<Sqlite>) -> Self {
        BoardRepositoryImpl { db }
    }
}

impl<'a> BoardRepository for BoardRepositoryImpl<'a> {
    async fn insert(&self, board: Board) -> Box<Board> {
        insert(&self.db, board).await.clone()
    }

    async fn get_by_id(&self, id: i64) -> Box<Board> {
        let board = get_by_id(&self.db, id).await.clone();
        board
    }

    async fn get_all(&self) -> Vec<Box<Board>> {
        let boards = get_all(&self.db).await.clone();
        boards
    }
}
