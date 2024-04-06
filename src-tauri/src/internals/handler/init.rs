use std::env;

use sqlx::SqlitePool;
use tauri::{command, Result};

use crate::{init_db, internals::{model::board::Board, repository::sqlite::board::BoardRepositoryImpl, usecase::{board::BoardUsecase, board_impl::BoardUsecaseImpl}}};

pub async fn init() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_boards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
pub async fn get_all_boards() -> Result<Vec<Box<Board>>> {
    let db_url = env::var("DB_URL").unwrap();
    println!("{}", db_url);
    init_db(db_url.as_str()).await;

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let board_uc = BoardUsecaseImpl::new(board_repo.clone());

    Ok(board_uc.get_all_boards().await)
}
