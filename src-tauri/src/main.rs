// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod internals;

use crate::internals::model::board::Board;
use crate::internals::model::res_data::ResData;
use crate::internals::repository::sqlite::board::BoardRepositoryImpl;
use crate::internals::repository::sqlite::repository::BoardRepository;
use crate::internals::repository::sqlite::res_data::ResDataRepoImpl;

use internals::repository::sqlite::repository::ResDataRepository;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use tauri::Result;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DB_URL").unwrap();
    println!("{}", db_url);
    init_db(db_url.as_str()).await;

    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    print!("{}", crate_dir);
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    let board_repo = BoardRepositoryImpl::new(db.clone()).await;

    let new_board = board_repo
        .insert(Board {
            board_id: 0,
            name: "Testing".to_string(),
            description: "desc test".to_string(),
            position: board_repo.get_highest_board_position().await + 1,
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
            deleted_at: Option::None,
        })
        .await;

    let mut board = board_repo.get_by_id(new_board.board_id).await;
    println!("{:?}", serde_json::to_string(&*board).unwrap());

    board.name = format!("{} (updated)", board.name);
    board.description = format!("{} (updated)", board.description);

    let updated_board = board_repo.update(*board).await;
    println!("{:?}", serde_json::to_string(&*updated_board).unwrap());

    let boards = board_repo.get_all().await;
    for (_idx, board) in boards.iter().enumerate() {
        println!("{:?}", serde_json::to_string(&*board).unwrap());
    }

    println!("{:?}", board_repo.get_highest_board_position().await);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_all_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn init_db(db_url: &str) {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("78fya9s8djiasj");
    format!("Hello, {}!", name)
}

pub trait ResDataHandler {
    fn get_all_data(&self) -> Result<Vec<ResData>>;
}

pub struct ResDataHandlerImpl {
    pub res_data_repo: Box<dyn ResDataRepository>,
}

impl ResDataHandlerImpl {
    pub fn new(res_data_repo: Box<dyn ResDataRepository>) -> Self {
        ResDataHandlerImpl { res_data_repo }
    }
}

impl ResDataHandler for ResDataHandlerImpl {
    fn get_all_data(&self) -> Result<Vec<ResData>> {
        let repo = ResDataRepoImpl::new();
        let res = repo.get_all_data();

        Ok(res)
    }
}

#[tauri::command]
fn get_all_data() -> Result<Vec<ResData>> {
    let repo = ResDataRepoImpl::new();
    let handler = ResDataHandlerImpl::new(Box::new(repo));
    let res = handler.get_all_data();

    res
}
