import { Task } from "@/model/task";
import { invoke } from "@tauri-apps/api/tauri";

export interface GetAllTasksFilter {
  is_archived: boolean;
}

export const getAllTasks = async (filter?: GetAllTasksFilter) => {
  if (!filter) {
    filter = {
      is_archived: false,
    };
  }

  try {
    let res = await invoke<Task[]>("get_all_tasks", {
      filter,
    });
    return res;
  } catch (e) {
    console.log(e);
  }
};
