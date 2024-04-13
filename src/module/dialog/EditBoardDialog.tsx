"use client";

import { TextEditorProps } from "@/components/textEditor/TextEditor";
import { defaultBoardContent, defaultBoardTitle } from "@/constants/constants";
import dynamic from "next/dynamic";
import { Dispatch, FC, SetStateAction, useState } from "react";

export interface CreateBoardDialogProps {
  title: string;
  open: boolean;
  setOpen?: () => void;
  // onSubmit: MouseEventHandler<HTMLButtonElement>;
  onSubmit: (name: string, description: string) => void;
}

const Dialog = dynamic(() => import("@/components/dialog/Dialog"), {
  ssr: false,
});
const TextEditor = dynamic<TextEditorProps>(
  () => import("@/components/textEditor/TextEditor"),
  {
    ssr: false,
  }
);

export const EditBoardDialog: FC<CreateBoardDialogProps> = ({
  title,
  open,
  setOpen,
  onSubmit,
}) => {
  const [name, setName] = useState<string>(defaultBoardTitle);
  const [description, setDescription] = useState<string>(defaultBoardContent);

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
          />
        </div>
      }
      buttonText="Submit"
      onSubmit={async (e) => {
        e.preventDefault();
        onSubmit(name, description);
      }}
    />
  );
};

export default EditBoardDialog;
