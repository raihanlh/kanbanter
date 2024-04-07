"use client";

import React, { useEffect, useState } from "react";
import {
  DragDropContext,
  Droppable,
  Draggable,
  DraggableProvided,
  DraggableStateSnapshot,
  DroppableProvided,
  DroppableStateSnapshot,
  OnDragEndResponder,
  DropResult,
} from "@hello-pangea/dnd";
import { invoke } from "@tauri-apps/api";
import { NextPage } from "next";

const getItems = (count: number) =>
  Array.from({ length: count }, (v, k) => k).map((k) => ({
    id: `item-${k}`,
    content: `item ${k}`,
  }));

const grid = 8;

const reorder = (
  boards: Board[],
  startDroppableId: number,
  startIndex: number,
  endDroppableId: number,
  endIndex: number
): Board[] => {
  let newBoards = boards;
  if (startDroppableId == endDroppableId) {
    let idx: number = -1;
    newBoards.forEach((board, index) => {
      if (board.board_id == startDroppableId) {
        idx = index;
      }
    });
    const result = Array.from(
      newBoards.filter((board) => board.board_id == startDroppableId)
    )[0].tasks;
    const [removed] = result.splice(startIndex, 1);
    result.splice(endIndex, 0, removed);
    if (idx > -1) {
      newBoards[idx].tasks = result;
    }
  } else {
    console.log("HERE 2");
    let idxSource: number = -1;
    let idxDestination: number = -1;

    newBoards.forEach((board, index) => {
      if (board.board_id == startDroppableId) {
        idxSource = index;
      }
      if (board.board_id == endDroppableId) {
        idxDestination = index;
      }
    });

    const resultSource = Array.from(
      newBoards.filter((board) => board.board_id == startDroppableId)
    )[0].tasks;

    const resultDest = Array.from(
      newBoards.filter((board) => board.board_id == endDroppableId)
    )[0].tasks;

    const [removed] = resultSource.splice(startIndex, 1);

    resultDest.splice(endIndex, 0, removed);
    if (idxSource >= 0 && idxDestination >= 0) {
      newBoards[idxSource].tasks = resultSource;
      newBoards[idxDestination].tasks = resultDest;
    }
  }

  return newBoards;
};

const getListStyle = (isDraggingOver: boolean) => ({
  background: isDraggingOver ? "lightblue" : "lightgrey",
  padding: grid,
  width: 250,
});

const getItemStyle = (isDragging: any, draggableStyle: any) => ({
  // some basic styles to make the items look a bit nicer
  userSelect: "none",
  padding: grid * 2,
  margin: `0 0 ${grid}px 0`,

  // change background colour if dragging
  background: isDragging ? "lightgreen" : "grey",

  // styles we need to apply on draggables
  ...draggableStyle,
});

interface ResData {
  id: number;
  content: string;
}

interface Task {
  task_id: number;
  board_id: number;
  title: string;
  description: string;
  position: number;
  created_at: Date;
  updated_at: Date;
  deleted_at: Date;
}

interface Board {
  board_id: number;
  name: string;
  description: string;
  position: number;
  tasks: Task[];
  created_at: Date;
  updated_at: Date;
  deleted_at: Date;
}

const Home: NextPage = () => {
  const [boards, setBoards] = useState<Board[]>([]);
  const [res, setRes] = useState<ResData[]>([]);

  useEffect(() => {
    invoke<ResData[]>("get_all_data", {})
      .then((result) => {
        setRes(result);
      })
      .catch(console.error);
    invoke<Board[]>("get_all_boards", {})
      .then((result) => {
        setBoards(result);
      })
      .catch(console.error);
  }, []);

  const onDragEnd: OnDragEndResponder = (result: DropResult) => {
    console.log(result);
    // dropped outside the list
    if (!result.destination) {
      return;
    }

    const newBoards = reorder(
      Array.from(boards),
      Number(result.source.droppableId),
      result.source.index,
      Number(result.destination.droppableId),
      result.destination.index
    );

    setBoards(newBoards);
  };

  return (
    <main>
      <h1>DND PAGE</h1>
      <DragDropContext onDragEnd={onDragEnd}>
        <div className="flex flex-row">
          {boards &&
            boards?.map((board) => (
              <div key={board.board_id} className="mx-2">
                <h4>{board.name}</h4>
                <Droppable
                  droppableId={`${board.board_id}`}
                  key={board.board_id}
                >
                  {(
                    provided: DroppableProvided,
                    snapshot: DroppableStateSnapshot
                  ) => (
                    <div
                      {...provided.droppableProps}
                      ref={provided.innerRef}
                      style={getListStyle(snapshot.isDraggingOver)}
                    >
                      {board.tasks.map(
                        (item, index) =>
                          item && (
                            <Draggable
                              key={`${board.board_id}-${item.task_id}`}
                              draggableId={`${board.board_id}-${item.task_id}`}
                              index={index}
                            >
                              {(
                                provided: DraggableProvided,
                                snapshot: DraggableStateSnapshot
                              ) => (
                                <div
                                  ref={provided.innerRef}
                                  {...provided.draggableProps}
                                  {...provided.dragHandleProps}
                                  style={getItemStyle(
                                    snapshot.isDragging,
                                    provided.draggableProps.style
                                  )}
                                >
                                  {item.title}
                                </div>
                              )}
                            </Draggable>
                          )
                      )}
                      {provided.placeholder}
                    </div>
                  )}
                </Droppable>
              </div>
            ))}
        </div>
      </DragDropContext>
      {/* <Greet />
      <div>
        <Link
          href={"/dnd"}
          content="DND"
        >
          <h3>DND Page</h3>
        </Link>
      </div> */}
      {/* <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          Get started by editing&nbsp;
          <code className="font-mono font-bold">src/app/page.tsx</code>
        </p>
        <div className="fixed bottom-0 left-0 flex h-48 w-full items-end justify-center bg-gradient-to-t from-white via-white dark:from-black dark:via-black lg:static lg:h-auto lg:w-auto lg:bg-none">
          <a
            className="pointer-events-none flex place-items-center gap-2 p-8 lg:pointer-events-auto lg:p-0"
            href="https://vercel.com?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            By{" "}
            <Image
              src="/vercel.svg"
              alt="Vercel Logo"
              className="dark:invert"
              width={100}
              height={24}
              priority
            />
          </a>
        </div>
      </div>

      <div className="relative flex place-items-center before:absolute before:h-[300px] before:w-full sm:before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-full sm:after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
        <Image
          className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
          src="/next.svg"
          alt="Next.js Logo"
          width={180}
          height={37}
          priority
        />
      </div>

      <div className="mb-32 grid text-center lg:max-w-5xl lg:w-full lg:mb-0 lg:grid-cols-4 lg:text-left">
        <a
          href="https://nextjs.org/docs?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Docs{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Find in-depth information about Next.js features and API.
          </p>
        </a>

        <a
          href="https://nextjs.org/learn?utm_source=create-next-app&utm_medium=appdir-template-tw&utm_campaign=create-next-app"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Learn{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Learn about Next.js in an interactive course with&nbsp;quizzes!
          </p>
        </a>

        <a
          href="https://vercel.com/templates?framework=next.js&utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Templates{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Explore starter templates for Next.js.
          </p>
        </a>

        <a
          href="https://vercel.com/new?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Deploy{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50 text-balance`}>
            Instantly deploy your Next.js site to a shareable URL with Vercel.
          </p>
        </a>
      </div> */}
    </main>
  );
};

export default Home;
