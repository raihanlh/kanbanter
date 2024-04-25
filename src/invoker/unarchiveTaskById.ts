import { invoke } from "@tauri-apps/api/tauri";

export const unarchiveTaskById = async (id: number) => {
  try {
    let res = await invoke<boolean>("unarchive_task_by_id", { id });
    return res;
  } catch (e) {
    console.log(e);
  }
};