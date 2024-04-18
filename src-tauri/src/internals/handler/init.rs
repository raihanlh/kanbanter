use std::env;

use sqlx::SqlitePool;
use tauri::{command, Result};

use crate::internals::{
    model::{board::{Board, GetAllBoardFilter}, task::Task},
    repository::sqlite::{board::BoardRepositoryImpl, task::TaskRepositoryImpl},
    usecase::{
        board::BoardUsecase, board_impl::BoardUsecaseImpl, task::TaskUsecase,
        task_impl::TaskUsecaseImpl,
    },
};

pub async fn init() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_boards,
            create_new_board,
            update_board,
            delete_board,
            archive_board,
            create_new_task,
            update_task,
            delete_task_by_id,
            archive_task_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
pub async fn create_new_board(name: String, description: String) -> Result<Box<Board>> {
    let mut board: Board = Board::default();
    board.name = name;
    board.description = description;

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
pub async fn archive_board(id: i64) -> Result<bool> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let board_uc = BoardUsecaseImpl::new(board_repo.clone(), task_repo.clone());

    Ok(board_uc.archive_board_by_id(id).await)
}

#[command]
pub async fn get_all_boards(filter: GetAllBoardFilter) -> Result<Vec<Box<Board>>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let board_uc = BoardUsecaseImpl::new(board_repo.clone(), task_repo.clone());

    Ok(board_uc.get_all_boards(filter).await)
}

// Task Handler

#[command]
pub async fn create_new_task(
    board_id: i64,
    title: String,
    description: String,
) -> Result<Box<Task>> {
    let db_url = env::var("DB_URL").unwrap();

    let mut task = Task::default();
    task.board_id = board_id;
    task.title = title;
    task.description = description;

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let task_uc = TaskUsecaseImpl::new(task_repo.clone());

    Ok(task_uc.create_new_task(task).await)
}

#[command]
pub async fn update_task(task: Task) -> Result<Box<Task>> {
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
pub async fn archive_task_by_id(id: i64) -> Result<bool> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let task_uc = TaskUsecaseImpl::new(task_repo.clone());

    Ok(task_uc.archive_task_by_id(id).await)
}

#[command]
pub async fn get_task_by_id(id: i64) -> Result<Box<Task>> {
    let db_url = env::var("DB_URL").unwrap();

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;
    let task_uc = TaskUsecaseImpl::new(task_repo.clone());

    Ok(task_uc.get_task_by_id(id).await)
}
