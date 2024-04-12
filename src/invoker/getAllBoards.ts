import { Board } from "@/model/board";
import { invoke } from "@tauri-apps/api";

export const getAllBoards = async () => {
  try {
    let res = await invoke<Board[]>("get_all_boards", {});
    return res;
  } catch (e) {
    console.log(e);
  }
};
