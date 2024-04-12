export interface Board {
  board_id?: number;
  name: string;
  description: string;
  position?: number;
  tasks?: Task[];
  created_at?: Date;
  updated_at?: Date;
  deleted_at?: Date;
}
