use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use super::{
    board_impl::{
        archive::archive, delete::delete, get_all::get_all, get_by_id::get_by_id, get_highest_board_position::get_highest_board_position, insert::insert, update::update
    },
    repository::BoardRepository,
};
use crate::internals::model::board::{Board, GetAllBoardFilter};

#[derive(Clone)]
pub struct BoardRepositoryImpl {
    db: Pool<Sqlite>,
}

impl BoardRepositoryImpl {
    pub async fn new(db: Pool<Sqlite>) -> Arc<Box<dyn BoardRepository + Send + Sync>> {
        Arc::new(Box::new(BoardRepositoryImpl { db }))
    }
}

#[async_trait]
impl BoardRepository for BoardRepositoryImpl {
    async fn insert(&self, board: Board) -> Box<Board> {
        insert(self.db.clone(), board).await
    }

    async fn get_by_id(&self, id: i64) -> Box<Board> {
        get_by_id(&self.db, id).await
    }

    async fn get_all(&self, filter: GetAllBoardFilter) -> Vec<Box<Board>> {
        get_all(&self.db, filter).await
    }

    async fn update(&self, board: Board) -> Box<Board> {
        update(&self.db, board).await
    }

    async fn get_highest_board_position(&self) -> i32 {
        get_highest_board_position(&self.db).await
    }

    async fn delete(&self, id: i64) -> bool {
        delete(&self.db, id).await
    }

    async fn archive(&self, id: i64) -> bool {
        archive(&self.db, id).await
    }

}
