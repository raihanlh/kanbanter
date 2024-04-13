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
import { invoke } from "@tauri-apps/api";
import { Board } from "@/model/board";
import { DropdownMenu } from "../menu/DropdownMenu";
import { BsThreeDotsVertical } from "react-icons/bs";
import { deleteBoardById } from "@/invoker/deleteBoardById";
import { getAllBoards } from "@/invoker/getAllBoards";
import CreateBoardDialog from "@/module/dialog/CreateBoardDialog";
import { editBoard } from "@/invoker/editBoard";

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

interface ResData {
  id: number;
  content: string;
}

export interface KanbanBoardProps {
  boards: Board[];
  setBoards: React.Dispatch<React.SetStateAction<Board[]>>;
}

const KanbanBoard: React.FC<KanbanBoardProps> = ({ boards, setBoards }) => {
  const [boardEdit, setBoardEdit] = useState<Board | null>(null);
  const [open, setOpen] = useState<boolean>(false);

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
        await invoke<Task>("update_task_by_id", { task })
          .then((result) => {
            console.log(result);
          })
          .catch(console.error);
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
          title="Edit board"
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
      <DragDropContext onDragEnd={onDragEnd}>
        <div className="flex flex-row">
          {boards &&
            boards?.map((board) => (
              <div key={board.board_id} className="mx-2">
                <div className="flex justify-between">
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
    </div>
  );
};

export default KanbanBoard;
