use sqlx::{Pool, Sqlite};

use crate::internals::repository::sqlite::queries::DELETE_BOARD_BY_BOARD_ID;

pub async fn delete(db: &Pool<Sqlite>, id: i64) -> bool {
    match sqlx::query(DELETE_BOARD_BY_BOARD_ID).bind(id).execute(db).await {
        Ok(result) => {
          return result.rows_affected() > 0;
        },
        Err(err) => {
            // Check if the error represents an empty result
            if let sqlx::Error::RowNotFound = err {
                // If the error represents an empty result, return 0
                return false;
            } else {
                // If it's another error, panic
                panic!("Error while deleting board: {}", err.to_string());
            }
        }
    };
}


