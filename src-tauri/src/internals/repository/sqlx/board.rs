use sqlx::{Pool, Sqlite};

use super::{board_impl::insert::insert, repository::BoardRepository};
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
        let mut board = Box::new(Board::default());
        board.board_id = id;
        board
    }

    async fn get_all(&self) -> Vec<Box<Board>> {
        let board = Box::new(Board::default());
        vec![board]
    }
}
