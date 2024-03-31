use sqlx::{Pool, Row, Sqlite};

use crate::internals::repository::sqlx::queries::GET_HIGHEST_BOARD_POSITION;

pub async fn get_highest_board_position(db: &Pool<Sqlite>) -> i32 {
    let result = match sqlx::query(GET_HIGHEST_BOARD_POSITION)
        .fetch_optional(db)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            // Check if the error represents an empty result
            if let sqlx::Error::RowNotFound = err {
                // If the error represents an empty result, return 0
                return 0;
            } else {
                // If it's another error, panic
                panic!("Error fetching highest board position: {:?}", err);
            }
        }
    };

    // Check if the result is None
    if let Some(result) = result {
        // If result is not None, it means the query returned a valid value
        // Return the position obtained from the query
        result.get::<i32, &str>("max_position")
    } else {
        // This branch should not be reached, as an empty result should be handled above
        panic!("Unexpected empty result while fetching highest board position");
    }
}
