// Boards
pub const INSERT_BOARD_QUERY: &str = 
    "INSERT INTO boards (name, description, created_at, updated_at, position) VALUES (?, ?, ?, ?, ?) ON CONFLICT(board_id) DO NOTHING";
pub const GET_BOARD_BY_ID_QUERY: &str = 
    "SELECT board_id, name, description, created_at, updated_at, position FROM boards WHERE board_id = ? AND deleted_at IS NULL";
pub const GET_ALL_BOARD_QUERY: &str = 
    "SELECT board_id, name, description, created_at, updated_at, position FROM boards";
pub const UPDATE_BOARD_QUERY: &str = 
    "UPDATE boards SET name = ?, description = ?, updated_at = ?, position = ? WHERE board_id = ? AND deleted_at IS NULL";
pub const GET_HIGHEST_BOARD_POSITION: &str =
    "SELECT MAX(position) AS max_position FROM boards WHERE deleted_at IS NULL ORDER BY position DESC LIMIT 1";
pub const ARCHIVE_BOARD_BY_BOARD_ID: &str =
    "UPDATE boards SET deleted_at = CURRENT_TIMESTAMP WHERE board_id = ?";
pub const DELETE_BOARD_BY_BOARD_ID: &str =
    "DELETE FROM boards WHERE board_id = ?";
pub const GET_LOWEST_BOARD_POSITION: &str =
    "SELECT MIN(position) AS min_position, board_id FROM boards WHERE deleted_at IS NULL ORDER BY position ASC LIMIT 1";

// Tasks
pub const INSERT_TASK_QUERY: &str = 
    "INSERT INTO tasks (title, description, board_id, created_at, updated_at, position) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT(task_id) DO NOTHING";
pub const GET_TASK_BY_ID_QUERY: &str = 
    "SELECT task_id, board_id, title, description, created_at, updated_at, position FROM tasks WHERE task_id = ? AND deleted_at IS NULL";
pub const GET_ALL_TASK_QUERY: &str = 
    "SELECT task_id, board_id, title, description, created_at, updated_at, position FROM tasks";
pub const GET_TASKS_BY_BOARD_ID_QUERY: &str = 
    "SELECT task_id, board_id, title, description, created_at, updated_at, position FROM tasks WHERE board_id = ?";
pub const UPDATE_TASK_QUERY: &str = 
    "UPDATE tasks SET title = ?, description = ?, board_id = ?, position = ?, updated_at = ? WHERE task_id = ? and  deleted_at IS NULL";
pub const GET_HIGHEST_TASK_POSITION: &str =
    "SELECT MAX(position) AS max_position FROM tasks WHERE board_id = ? AND deleted_at IS NULL";
pub const ARCHIVE_TASK_BY_TASK_ID: &str =
    "UPDATE tasks SET deleted_at = CURRENT_TIMESTAMP WHERE task_id = ?";
pub const ARCHIVE_TASK_BY_BOARD_ID: &str =
    "UPDATE tasks SET deleted_at = CURRENT_TIMESTAMP WHERE board_id = ?";
pub const DELETE_TASK_BY_TASK_ID: &str =
    "DELETE FROM tasks WHERE task_id = ?";
pub const DELETE_TASK_BY_BOARD_ID: &str =
    "DELETE FROM tasks WHERE board_id = ?";
pub const UNARCHIVE_TASK_BY_TASK_ID: &str =
    "UPDATE tasks SET board_id = ?, deleted_at = NULL WHERE task_id = ?";