CREATE INDEX idx_task_id_deleted_at ON tasks (task_id, deleted_at);

CREATE INDEX idx_board_id_deleted_at ON boards (board_id, deleted_at);