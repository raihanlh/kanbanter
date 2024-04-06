use std::env;

use sqlx::SqlitePool;
use tauri::{command, Result};

use crate::{init_db, internals::{model::{board::Board, res_data::ResData}, repository::sqlite::{board::BoardRepositoryImpl, res_data::ResDataRepoImpl}, usecase::{board::BoardUsecase, board_impl::BoardUsecaseImpl}}, ResDataHandler, ResDataHandlerImpl};

pub async fn init() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_boards, get_all_data])
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

#[tauri::command]
fn get_all_data() -> Result<Vec<ResData>> {
    let repo = ResDataRepoImpl::new();
    let handler = ResDataHandlerImpl::new(Box::new(repo));
    let res = handler.get_all_data();

    res
}