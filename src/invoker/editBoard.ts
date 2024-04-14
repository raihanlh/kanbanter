import { Board } from "@/model/board";
import { invoke } from "@tauri-apps/api/tauri";

export const editBoard = async (board: Board) => {
  try {
    let res = await invoke<boolean>("update_board", { board });
    return res;
  } catch (e) {
    console.log(e);
  }
};
