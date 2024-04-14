import { invoke } from "@tauri-apps/api/tauri";

export const deleteBoardById = async (id: number) => {
  try {
    let res = await invoke<boolean>("delete_board", { id });
    return res;
  } catch (e) {
    console.log(e);
  }
};
