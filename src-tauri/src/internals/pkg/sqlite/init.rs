use sqlx::{migrate::MigrateDatabase, Sqlite};

pub async fn init_db(db_url: &str) {
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