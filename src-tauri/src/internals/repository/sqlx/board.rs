use sqlx::{Pool, Sqlite};

use super::{
    board_impl::{get_all::get_all, get_by_id::get_by_id, get_highest_board_position::get_highest_board_position, insert::insert, update::update},
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
        insert(&self.db, board).await
    }

    async fn get_by_id(&self, id: i64) -> Box<Board> {
        get_by_id(&self.db, id).await
    }

    async fn get_all(&self) -> Vec<Box<Board>> {
        get_all(&self.db).await
    }

    async fn update(&self, board: Board) -> Box<Board> {
        update(&self.db, board).await
    }

    async fn get_highest_board_position(&self) -> i32 {
        get_highest_board_position(&self.db).await
    }
}
