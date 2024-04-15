import { invoke } from "@tauri-apps/api/tauri";

export const deleteTaskById = async (id: number) => {
  try {
    let res = await invoke<boolean>("delete_task_by_id", { id });
    return res;
  } catch (e) {
    console.log(e);
  }
};
