// Boards
pub const INSERT_BOARD_QUERY: &str = 
    "INSERT INTO boards (name, description, created_at, updated_at, position) VALUES (?, ?, ?, ?, ?) ON CONFLICT(board_id) DO NOTHING";
pub const GET_BOARD_BY_ID_QUERY: &str = 
    "SELECT board_id, name, description, created_at, updated_at, position FROM boards WHERE board_id = ? AND deleted_at IS NULL";
pub const GET_ALL_BOARD_QUERY: &str = 
    "SELECT board_id, name, description, created_at, updated_at, position FROM boards WHERE deleted_at IS NULL";
pub const UPDATE_BOARD_QUERY: &str = 
    "UPDATE boards SET name = ?, description = ?, created_at = ?, updated_at = ?, position = ? WHERE board_id = ? AND deleted_at IS NULL";
pub const GET_HIGHEST_BOARD_POSITION: &str =
    "SELECT MAX(position) AS max_position FROM boards WHERE deleted_at IS NULL";

// Tasks
pub const INSERT_TASK_QUERY: &str = 
    "INSERT INTO tasks (title, description, board_id, position, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT(task_id) DO NOTHING";
pub const GET_TASK_BY_ID_QUERY: &str = 
    "SELECT task_id, name, description, created_at, updated_at FROM tasks WHERE task_id = ? AND deleted_at IS NULL";
pub const GET_ALL_TASK_QUERY: &str = 
    "SELECT task_id, name, description, created_at, updated_at FROM tasks AND deleted_at IS NULL";
pub const UPDATE_TASK_QUERY: &str = 
    "UPDATE tasks SET title = ?, description = ?, board_id = ?, position = ?, created_at = ?, updated_at = ? WHERE task_id = ? and  deleted_at IS NULL";
pub const GET_HIGHEST_TASK_POSITION: &str =
    "SELECT MAX(position) AS max_position FROM tasks WHERE task_id = ? WHERE deleted_at IS NULL";