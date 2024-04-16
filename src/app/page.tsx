"use client";

import { createBoard } from "@/invoker/createBoard";
import { getAllBoards } from "@/invoker/getAllBoards";
import { Board } from "@/model/board";
import { CreateBoardDialogProps } from "@/module/dialog/CreateBoardDialog";
import { NextPage } from "next";
import dynamic from "next/dynamic";
import { useEffect, useState } from "react";
import { CiSquarePlus } from "react-icons/ci";

const KanbanBoard = dynamic(() => import("@/components/board/KanbanBoard"), {
  ssr: false,
});
const CreateBoardDialog = dynamic<CreateBoardDialogProps>(
  () => import("@/module/dialog/CreateBoardDialog"),
  {
    ssr: false,
  }
);

const Home: NextPage = () => {
  const [open, setOpen] = useState(false);
  const [boards, setBoards] = useState<Board[]>([]);

  const initBoards = async () => {
    const res = await getAllBoards();
    setBoards(res);
  };

  useEffect(() => {
    initBoards();
  }, []);

  return (
    <main>
      <header className="w-screen flex justify-end">
        <button
          className="m-4 bg-transparent hover:bg-gray-700 rounded"
          onClick={(e) => {
            e.preventDefault();
            setOpen(true);
          }}
        >
          <CiSquarePlus size={36} />
        </button>
      </header>
      <CreateBoardDialog
        open={open}
        setOpen={setOpen}
        onSubmit={async (name, description) => {
          try {
            await createBoard(name, description);
            const res = await getAllBoards();
            setBoards(res);
          } catch (e) {
            console.log(e);
          }
        }}
      />
      <KanbanBoard boards={boards} setBoards={setBoards} />
    </main>
  );
};

export default Home;
