use sqlx::{Pool, Sqlite};

use crate::internals::repository::sqlite::queries::ARCHIVE_TASK_BY_TASK_ID;

pub async fn archive(db: &Pool<Sqlite>, id: i64) -> bool {
    match sqlx::query(ARCHIVE_TASK_BY_TASK_ID).bind(id).execute(db).await {
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
                panic!("Error while deleting task: {}", err.to_string());
            }
        }
    };
}


