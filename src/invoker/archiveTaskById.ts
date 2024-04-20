import { invoke } from "@tauri-apps/api/tauri";

export const archiveTaskById = async (id: number) => {
  try {
    let res = await invoke<boolean>("archive_task_by_id", { id });
    return res;
  } catch (e) {
    console.log(e);
  }
};