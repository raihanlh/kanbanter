"use client";

import { DropdownMenu } from "@/components/menu/DropdownMenu";
import { unarchiveTaskById } from "@/invoker/unarchiveTaskById";
import { getAllTasks } from "@/invoker/getAllTask";
import { Task } from "@/model/task";
import { NextPage } from "next";
import { useEffect, useState } from "react";
import { BsThreeDotsVertical } from "react-icons/bs";

const ArchivePage: NextPage = () => {
  const [tasks, setTasks] = useState<Task[]>([]);

  const initTasks = async () => {
    const res = await getAllTasks({ is_archived: true });
    console.log("initTasks");
    console.log(res);
    setTasks(res);
  };

  useEffect(() => {
    initTasks();
  }, []);

  initTasks();
  return (
    <div className="container mx-auto overflow-auto">
      <table className="table-auto min-w-full">
        <thead>
          <tr>
            <th className="border-b border-blue-gray-100 bg-blue-gray-50 p-4">
              Task
            </th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr key={task.task_id}>
              <td className="p-4 border-b border-blue-gray-50 flex justify-between flex-row">
                <div>{task.title}</div>
                <DropdownMenu
                  text={<BsThreeDotsVertical />}
                  dropdownItems={[
                    {
                      text: "Unarchive",
                      onClick: async (e) => {
                        try {
                          e.preventDefault();
                          await unarchiveTaskById(task.task_id);

                          const res = await getAllTasks({ is_archived: true });
                          setTasks(res);
                        } catch (e) {
                          console.log(e);
                        }
                      },
                    },
                  ]}
                />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default ArchivePage;
