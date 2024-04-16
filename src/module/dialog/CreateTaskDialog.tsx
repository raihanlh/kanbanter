"use client";

import { TextEditorProps } from "@/components/textEditor/TextEditor";
import { defaultBoardContent, defaultBoardTitle } from "@/constants/constants";
import dynamic from "next/dynamic";
import { Dispatch, FC, SetStateAction, useState } from "react";

const Dialog = dynamic(() => import("@/components/dialog/Dialog"), {
  ssr: false,
});
const TextEditor = dynamic<TextEditorProps>(
  () => import("@/components/textEditor/TextEditor"),
  {
    ssr: false,
  }
);

export interface CreateTaskDialogProps {
  title?: string;
  boardName?: string;
  boardContent?: string;
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  // onSubmit: MouseEventHandler<HTMLButtonElement>;
  onSubmit: (name: string, description: string) => void;
}

export const CreateTaskDialog: FC<CreateTaskDialogProps> = ({
  title = "Add new task",
  boardName,
  boardContent,
  open,
  setOpen,
  onSubmit,
}) => {
  const [name, setName] = useState<string>(boardName ?? defaultBoardTitle);
  const [description, setDescription] = useState<string>(
    boardContent ?? defaultBoardContent
  );

  return (
    <Dialog
      open={open}
      setOpen={setOpen}
      title={title}
      content={
        <div className="space-y-3">
          <div className="my-12">
            <TextEditor
              enableMenuBar={false}
              onUpdate={({ editor, transaction }) => {
                setName(editor?.getText().toString());
              }}
              content={name}
              className="border border-sky-500 p-2"
            />
          </div>
          <TextEditor
            onUpdate={({ editor, transaction }) => {
              setDescription(editor?.getText().toString());
            }}
            content={defaultBoardContent}
            editorProps={{
              attributes: {
                class: "mt-3",
              },
            }}
            className="border border-sky-500 p-2"
          />
        </div>
      }
      buttonText="Submit"
      onSubmit={async (e) => {
        e.preventDefault();
        onSubmit(name, description);
        setOpen(false);
      }}
    />
  );
};

export default CreateTaskDialog;
