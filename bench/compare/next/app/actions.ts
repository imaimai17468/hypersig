"use server";

import { add, remove, setDone } from "./todos";

export async function addTodo(id: number, title: string): Promise<void> {
  const trimmed = title.trim();
  if (trimmed !== "") add({ id, title: trimmed, done: false });
}

export async function setTodoDone(id: number, done: boolean): Promise<void> {
  setDone(id, done);
}

export async function deleteTodo(id: number): Promise<void> {
  remove(id);
}
