"use client";

import { getAllTasks } from "@/invoker/getAllTask";
import { Task } from "@/model/task";
import { NextPage } from "next";
import { useEffect, useState } from "react";

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
            <th className="border-b border-blue-gray-100 bg-blue-gray-50 p-4">Task</th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr key={task.task_id}>
              <td className="p-4 border-b border-blue-gray-50">{task.title}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default ArchivePage;
