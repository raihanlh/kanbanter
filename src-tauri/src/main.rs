// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod internals;

use crate::internals::model::res_data::ResData;
use crate::internals::pkg::sqlite::seeder::seed;
use crate::internals::repository::sqlite::res_data::ResDataRepoImpl;

use internals::pkg::sqlite::migrate::migrate_db;
use internals::repository::sqlite::repository::ResDataRepository;
use tauri::Result;

use dotenv::dotenv;
use std::env;

use crate::internals::handler::init::init;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DB_URL").unwrap();
 
    migrate_db(db_url.to_owned()).await;
    seed(db_url.to_owned()).await;

    init().await;
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
