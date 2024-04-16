import { Board } from "@/model/board";
import { invoke } from "@tauri-apps/api/tauri";

export const createBoard = async (name: string, description: string) => {
  try {
    let res = await invoke<Board>("create_new_board", {
      name,
      description,
    });
    return res;
  } catch (e) {
    console.log(e);
  }
};
