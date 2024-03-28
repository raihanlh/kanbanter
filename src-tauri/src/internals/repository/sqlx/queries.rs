pub const INSERT_BOARD_QUERY: &str = 
    "INSERT INTO boards (name, description, created_at, updated_at) VALUES (?, ?, ?, ?) ON CONFLICT(board_id) DO NOTHING";