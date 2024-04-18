import { Board } from "@/model/board";
import { invoke } from "@tauri-apps/api/tauri";

export interface GetAllBoardsFilter {
  is_archived: boolean;
}

export const getAllBoards = async (filter?: GetAllBoardsFilter) => {
  if (!filter) {
    filter = {
      is_archived: false,
    };
  }

  try {
    let res = await invoke<Board[]>("get_all_boards", {
      filter,
    });
    return res;
  } catch (e) {
    console.log(e);
  }
};
