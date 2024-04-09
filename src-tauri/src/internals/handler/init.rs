use std::env;

use sqlx::SqlitePool;
use tauri::{command, Result};

use crate::{
    internals::{
        model::{board::Board, res_data::ResData, task::Task},
        repository::sqlite::{
            board::BoardRepositoryImpl, res_data::ResDataRepoImpl, task::TaskRepositoryImpl,
        },
        usecase::{
            board::BoardUsecase, board_impl::BoardUsecaseImpl, task::TaskUsecase,
            task_impl::TaskUsecaseImpl,
        },
    },
    ResDataHandler, ResDataHandlerImpl,
};

pub async fn init() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_boards,
            update_task_by_id,
            update_board,
            delete_board
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
pub async fn create_new_board(board: Board) -> Result<Box<Board>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let board_uc = BoardUsecaseImpl::new(board_repo.clone(), task_repo.clone());

    Ok(board_uc.create_new_board(board).await)
}


#[command]
pub async fn update_board(board: Board) -> Result<Box<Board>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let board_uc = BoardUsecaseImpl::new(board_repo.clone(), task_repo.clone());

    Ok(board_uc.update_board_by_id(board).await)
}

#[command]
pub async fn delete_board(id: i64) -> Result<bool> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let board_uc = BoardUsecaseImpl::new(board_repo.clone(), task_repo.clone());

    Ok(board_uc.delete_board_by_id(id).await)
}

#[command]
pub async fn get_all_boards() -> Result<Vec<Box<Board>>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let board_uc = BoardUsecaseImpl::new(board_repo.clone(), task_repo.clone());

    Ok(board_uc.get_all_boards().await)
}

// Task Handler

#[command]
pub async fn create_new_task(task: Task) -> Result<Box<Task>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let task_uc = TaskUsecaseImpl::new(task_repo.clone());

    Ok(task_uc.create_new_task(task).await)
}

#[command]
pub async fn update_task_by_id(task: Task) -> Result<Box<Task>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let task_uc = TaskUsecaseImpl::new(task_repo.clone());

    Ok(task_uc.update_task_by_id(task).await)
}

#[command]
pub async fn delete_task_by_id(id: i64) -> Result<bool> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let task_uc = TaskUsecaseImpl::new(task_repo.clone());

    Ok(task_uc.delete_task_by_id(id).await)
}

#[command]
pub async fn get_task_by_id(id: i64) -> Result<Box<Task>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let task_uc = TaskUsecaseImpl::new(task_repo.clone());

    Ok(task_uc.get_task_by_id(id).await)
}
