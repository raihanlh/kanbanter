import { invoke } from "@tauri-apps/api/tauri";

export const createTask = async (
  boardId: number,
  title: string,
  description: string
) => {
  try {
    let res = await invoke<boolean>("create_new_task", {
      boardId,
      title,
      description,
    });
    return res;
  } catch (e) {
    console.log(e);
  }
};
