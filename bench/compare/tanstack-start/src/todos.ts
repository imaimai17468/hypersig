// Server functions and the query for the list. The handlers run on the server
// only; Start's compiler replaces them with RPC stubs in the client bundle.

import { queryOptions } from "@tanstack/react-query";
import { createServerFn } from "@tanstack/react-start";
import { add, list, remove, setDone, type Todo } from "./todos.server";

export type { Todo };

export const listTodos = createServerFn({ method: "GET" }).handler(async () => list());

export const addTodo = createServerFn({ method: "POST" })
  .validator((data: { id: number; title: string }) => data)
  .handler(async ({ data }) => {
    const title = data.title.trim();
    return title === "" ? null : add({ id: data.id, title, done: false });
  });

export const setTodoDone = createServerFn({ method: "POST" })
  .validator((data: { id: number; done: boolean }) => data)
  .handler(async ({ data }) => setDone(data.id, data.done) ?? null);

export const deleteTodo = createServerFn({ method: "POST" })
  .validator((id: number) => id)
  .handler(async ({ data }) => {
    remove(data);
  });

// `staleTime: Infinity`: the list is fetched once, by the route loader during
// server rendering, and from then on the mutations keep the cache in step with
// the server. A finite stale time would refetch all 2000 items after
// hydration and on every window focus.
export const todosQuery = queryOptions({
  queryKey: ["todos"],
  queryFn: () => listTodos(),
  staleTime: Infinity,
});
