interface Task {
  task_id: number;
  board_id: number;
  title: string;
  description: string;
  position: number;
  created_at: Date;
  updated_at: Date;
  deleted_at: Date;
}