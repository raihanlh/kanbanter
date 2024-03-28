'use client'

import { NextPage } from "next"
import React, { useEffect, useState } from "react";
import { DragDropContext, Droppable, Draggable, DraggableProvided, DraggableStateSnapshot, DroppableProvided, DroppableStateSnapshot, OnDragEndResponder, DropResult } from "@hello-pangea/dnd";
import { invoke } from "@tauri-apps/api";

const getItems = (count: number) =>
    Array.from({ length: count }, (v, k) => k).map(k => ({
        id: `item-${k}`,
        content: `item ${k}`
    }));

const grid = 8;

const reorder = (list: any[], startIndex: number, endIndex: number): any[] => {
    const result = Array.from(list);
    const [removed] = result.splice(startIndex, 1);
    result.splice(endIndex, 0, removed);

    return result;
};

const getListStyle = (isDraggingOver: boolean) => ({
    background: isDraggingOver ? "lightblue" : "lightgrey",
    padding: grid,
    width: 250
});

const getItemStyle = (isDragging: any, draggableStyle: any) => ({
    // some basic styles to make the items look a bit nicer
    userSelect: "none",
    padding: grid * 2,
    margin: `0 0 ${grid}px 0`,

    // change background colour if dragging
    background: isDragging ? "lightgreen" : "grey",

    // styles we need to apply on draggables
    ...draggableStyle
});

interface ResData {
    id: number;
    content: string;
}

const DNDPage: NextPage = () => {
    const [items, setItems] = useState<any[]>(getItems(10))
    const [res, setRes] = useState<ResData[]>([])

    useEffect(() => {
        invoke<ResData[]>('get_all_data', { })
              .then(result => {
                setRes(result);
                console.log(result); // Log the updated value of greeting
              })
              .catch(console.error)
    }, [])

    const onDragEnd: OnDragEndResponder = (result: DropResult) => {
        // dropped outside the list
        if (!result.destination) {
            return;
        }

        const newItems = reorder(
            items,
            result.source.index,
            result.destination.index
        );

        setItems(
            newItems
        );
    }

    return <>
        <h1>DND PAGE</h1>
        <DragDropContext onDragEnd={onDragEnd}>
            <Droppable droppableId="droppable">
                {(provided: DroppableProvided, snapshot: DroppableStateSnapshot) => (
                    <div
                        {...provided.droppableProps}
                        ref={provided.innerRef}
                        style={getListStyle(snapshot.isDraggingOver)}
                    >
                        {res.map((item, index) => (
                            <Draggable key={`${item.id}`} draggableId={`${item.id}`} index={index}>
                                {(provided: DraggableProvided, snapshot: DraggableStateSnapshot) => (
                                    <div
                                        ref={provided.innerRef}
                                        {...provided.draggableProps}
                                        {...provided.dragHandleProps}
                                        style={getItemStyle(
                                            snapshot.isDragging,
                                            provided.draggableProps.style
                                        )}
                                    >
                                        {item.content}
                                    </div>
                                )}
                            </Draggable>
                        ))}
                        {provided.placeholder}
                    </div>
                )}
            </Droppable>
        </DragDropContext>
    </>
}

export default DNDPage;