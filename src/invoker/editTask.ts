import { Task } from "@/model/task";
import { invoke } from "@tauri-apps/api/tauri";

export const editTask = async (task: Task) => {
  try {
    let res = await invoke<boolean>("update_task", { task });
    return res;
  } catch (e) {
    console.log(e);
  }
};
