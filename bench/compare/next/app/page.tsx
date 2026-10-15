import { connection } from "next/server";
import TodoApp from "./todo-app";
import { list } from "./todos";

export default async function Page() {
  // The list changes between requests, so render at request time, not at build.
  await connection();
  return <TodoApp initialTodos={list()} />;
}
