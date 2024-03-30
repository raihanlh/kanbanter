pub const INSERT_BOARD_QUERY: &str = 
    "INSERT INTO boards (name, description, created_at, updated_at) VALUES (?, ?, ?, ?) ON CONFLICT(board_id) DO NOTHING";
pub const GET_BOARD_BY_ID_QUERY: &str = 
    "SELECT board_id, name, description, created_at, updated_at FROM boards WHERE board_id = ?";
pub const GET_ALL_BOARD_QUERY: &str = 
    "SELECT board_id, name, description, created_at, updated_at FROM boards";