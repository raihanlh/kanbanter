"use client";

import React, { FC, useEffect, useState } from "react";
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
import { invoke } from "@tauri-apps/api/tauri";
import { Board } from "@/model/board";
import { DropdownMenu } from "../menu/DropdownMenu";
import { BsThreeDotsVertical } from "react-icons/bs";
import { deleteBoardById } from "@/invoker/deleteBoardById";
import { getAllBoards } from "@/invoker/getAllBoards";
import CreateBoardDialog from "@/module/dialog/CreateBoardDialog";
import { editBoard } from "@/invoker/editBoard";
import CreateTaskDialog from "@/module/dialog/CreateTaskDialog";
import { createTask } from "@/invoker/createTask";
import { deleteTaskById } from "@/invoker/deleteTaskById";
import { editTask } from "@/invoker/editTask";
import { Task } from "@/model/task";
import { CiSquarePlus } from "react-icons/ci";
import Tippy from "@tippyjs/react/headless";
import { Editor, generateHTML } from "@tiptap/react";
import Bold from "@tiptap/extension-bold";
import Document from "@tiptap/extension-document";
import Paragraph from "@tiptap/extension-paragraph";
import Text from "@tiptap/extension-text";
import TextEditor from "../textEditor/TextEditor";

const grid = 8;

const reorder = (
  boards: Board[],
  startDroppableId: number,
  startIndex: number,
  endDroppableId: number,
  endIndex: number
): Board[] => {
  let newBoards = boards;
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

  removed.board_id = Number(endDroppableId);
  resultDest.splice(endIndex, 0, removed);
  if (idxSource >= 0 && idxDestination >= 0) {
    newBoards[idxSource].tasks = resultSource.map((task, index) => {
      task.position = index + 1;
      return task;
    });
    newBoards[idxDestination].tasks = resultDest.map((task, index) => {
      let newTask = Object.assign({}, task);
      newTask.position = index + 1;
      return newTask;
    });
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

export interface KanbanBoardProps {
  boards: Board[];
  setBoards: React.Dispatch<React.SetStateAction<Board[]>>;
}

const KanbanBoard: React.FC<KanbanBoardProps> = ({ boards, setBoards }) => {
  const [boardEdit, setBoardEdit] = useState<Board | null>(null);
  const [open, setOpen] = useState<boolean>(false);

  const [boardAddTask, setBoardAddTask] = useState<Board | null>(null);
  const [openAddTask, setOpenAddTask] = useState<boolean>(false);

  const [taskEdit, setTaskEdit] = useState<Task | null>(null);
  const [openTaskEdit, setOpenTaskEdit] = useState<boolean>(false);

  const onDragEnd: OnDragEndResponder = async (result: DropResult) => {
    // dropped outside the list
    if (!result.destination) {
      return;
    }

    let newBoards = reorder(
      Array.from(boards),
      Number(result.source.droppableId),
      result.source.index,
      Number(result.destination.droppableId),
      result.destination.index
    );

    for (let board of newBoards) {
      for (let task of board.tasks) {
        await editTask(task);
      }
    }

    await invoke<Board[]>("get_all_boards", {})
      .then((result) => {
        setBoards(result);
      })
      .catch(console.error);
  };

  return (
    <div>
      {boardEdit && (
        <CreateBoardDialog
          title="Edit swimlane"
          open={open && Boolean(boardEdit)}
          setOpen={setOpen}
          boardName={boardEdit.name}
          boardContent={boardEdit.description}
          onSubmit={async (name, description) => {
            try {
              let newBoard = Object.assign({}, boardEdit);
              if (newBoard) {
                newBoard.name = name;
                newBoard.description = description;
                let res = await editBoard(newBoard);

                console.log(res);

                let newBoards = await getAllBoards();
                setBoards(newBoards);
              }

              setBoardEdit(null);
              setOpen(false);
            } catch (e) {
              console.log(e);
            }
          }}
        />
      )}
      {boardAddTask && (
        <CreateTaskDialog
          title={`Add new task on ${boardAddTask?.name}`}
          open={openAddTask && Boolean(boardAddTask)}
          setOpen={setOpenAddTask}
          onSubmit={async (name, description) => {
            try {
              if (boardAddTask) {
                let res = await createTask(
                  boardAddTask.board_id,
                  name,
                  description
                );

                console.log(res);

                let newBoards = await getAllBoards();
                setBoards(newBoards);
              }
              setBoardAddTask(null);
              setOpen(false);
            } catch (e) {
              console.log(e);
            }
          }}
        />
      )}
      {taskEdit && (
        <CreateBoardDialog
          title="Edit task"
          open={openTaskEdit && Boolean(taskEdit)}
          setOpen={setOpenTaskEdit}
          boardName={taskEdit.title}
          boardContent={taskEdit.description}
          onSubmit={async (name, description) => {
            try {
              let newTask = Object.assign({}, taskEdit);
              if (newTask) {
                newTask.title = name;
                newTask.description = description;
                let res = await editTask(newTask);

                console.log(res);

                let newBoards = await getAllBoards();
                setBoards(newBoards);
              }

              setTaskEdit(null);
              setOpenTaskEdit(false);
            } catch (e) {
              console.log(e);
            }
          }}
        />
      )}
      <DragDropContext onDragEnd={onDragEnd}>
        <div className="flex flex-row">
          {boards &&
            boards?.map((board) => (
              <div key={board.board_id} className="mx-2 rounded">
                <div className="flex justify-between rounded">
                  <h4>{board.name}</h4>
                  <DropdownMenu
                    text={<BsThreeDotsVertical />}
                    dropdownItems={[
                      {
                        text: "Edit",
                        onClick: (e) => {
                          try {
                            e.preventDefault();

                            setBoardEdit(board);
                            setOpen(true);
                          } catch (e) {
                            console.log(e);
                          }
                        },
                      },
                      {
                        text: "Delete",
                        onClick: async (e) => {
                          try {
                            e.preventDefault();
                            await deleteBoardById(board.board_id);

                            let newBoards = await getAllBoards();
                            setBoards(newBoards);
                          } catch (e) {
                            console.log(e);
                          }
                        },
                      },
                    ]}
                  />
                </div>
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
                      className="rounded"
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
                                <Tippy
                                  interactive={true}
                                  placement="auto"
                                  arrow={true}
                                  render={(attrs) => (
                                    <div
                                      className="rounded bg-gray-700 m-3 p-3 space-y-3"
                                      tabIndex={-1}
                                      {...attrs}
                                    >
                                      <h3>{item.title}</h3>
                                      <div className="space-y-3">
                                        <TextEditor
                                          editable={false}
                                          enableMenuBar={false}
                                          onUpdate={() => {}}
                                          content={item.description}
                                          editorProps={{
                                            attributes: {
                                              class: "mt-3",
                                            },
                                          }}
                                        />
                                      </div>
                                    </div>
                                  )}
                                >
                                  <div
                                    ref={provided.innerRef}
                                    {...provided.draggableProps}
                                    {...provided.dragHandleProps}
                                    style={getItemStyle(
                                      snapshot.isDragging,
                                      provided.draggableProps.style
                                    )}
                                    className="rounded"
                                  >
                                    <div className="flex justify-between">
                                      <h4>{item.title}</h4>
                                      <DropdownMenu
                                        text={<BsThreeDotsVertical />}
                                        dropdownItems={[
                                          {
                                            text: "Edit",
                                            onClick: (e) => {
                                              try {
                                                e.preventDefault();

                                                setTaskEdit(item);
                                                setOpenTaskEdit(true);
                                              } catch (e) {
                                                console.log(e);
                                              }
                                            },
                                          },
                                          {
                                            text: "Delete",
                                            onClick: async (e) => {
                                              try {
                                                e.preventDefault();
                                                await deleteTaskById(
                                                  item.task_id
                                                );

                                                let newBoards =
                                                  await getAllBoards();
                                                setBoards(newBoards);
                                              } catch (e) {
                                                console.log(e);
                                              }
                                            },
                                          },
                                        ]}
                                      />
                                    </div>
                                  </div>
                                </Tippy>
                              )}
                            </Draggable>
                          )
                      )}
                      {provided.placeholder}
                    </div>
                  )}
                </Droppable>
                <div className="flex justify-end">
                  <button
                    className="my-4 bg-transparent hover:bg-gray-700 rounded"
                    onClick={(e) => {
                      e.preventDefault();
                      setBoardAddTask(board);
                      setOpenAddTask(true);
                    }}
                  >
                    <CiSquarePlus size={30} />
                  </button>
                </div>
              </div>
            ))}
        </div>
      </DragDropContext>
    </div>
  );
};

export default KanbanBoard;
