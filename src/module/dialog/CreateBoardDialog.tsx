import { TextEditor } from "@/components/textEditor/TextEditor";
import dynamic from "next/dynamic";
import { Dispatch, FC, SetStateAction, useState } from "react";

export interface CreateBoardDialogProps {
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  // onSubmit: MouseEventHandler<HTMLButtonElement>;
  onSubmit: (name: string, description: string) => void;
}

const Dialog = dynamic(() => import("@/components/dialog/Dialog"), {
  ssr: false,
});

export const CreateBoardDialog: FC<CreateBoardDialogProps> = ({
  open,
  setOpen,
  onSubmit,
}) => {
  const [name, setName] = useState<string>("");
  const [description, setDescription] = useState<string>("");

  return (
    // <></>
    <Dialog
      open={open}
      setOpen={setOpen}
      title={"Add new board"}
      content={
        <>
          <TextEditor
            enableMenuBar={false}
            onUpdate={({ editor, transaction }) => {
              setName(editor.getText.toString);
            }}
            content="Title"
          />
          <TextEditor
            onUpdate={({ editor, transaction }) => {
              setDescription(editor.getText.toString);
            }}
          />
        </>
      }
      buttonText="Submit"
      onSubmit={async (e) => {
        e.preventDefault();
        onSubmit(name, description);
      }}
    />
  );
};

export default CreateBoardDialog;
