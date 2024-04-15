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

export interface CreateBoardDialogProps {
  title?: string;
  boardName?: string;
  boardContent?: string;
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  onSubmit: (name: string, description: string) => void;
}

export const CreateBoardDialog: FC<CreateBoardDialogProps> = ({
  title = "Add new swimlane",
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
                setName(editor?.getHTML().toString());
              }}
              content={name}
            />
          </div>
          <TextEditor
            onUpdate={({ editor, transaction }) => {
              setDescription(editor?.getHTML().toString());
            }}
            content={description}
            editorProps={{
              attributes: {
                class: "mt-3",
              },
            }}
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

export default CreateBoardDialog;
