// TodoMVC-2000 in TanStack Start 1.169 with TanStack Query 5.
//
// Pattern: the route loader calls `queryClient.ensureQueryData(todosQuery)`,
// whose `queryFn` is a `createServerFn` server function. During server
// rendering that fills the query cache, `@tanstack/react-router-ssr-query`
// dehydrates it into the HTML, and the component reads it with
// `useSuspenseQuery`, so the first HTML has all 2000 items and hydration does
// not fetch again. Each add, toggle, and delete is a `useMutation` whose
// `mutationFn` is a server function, with Query's optimistic update through
// the cache: `onMutate` cancels in-flight list queries, snapshots the list,
// and writes the new list with `setQueryData`; `onError` puts the snapshot
// back. Items are `React.memo` components keyed by id. The updaters keep every
// unchanged item object (`map` returns the same object, `filter` and the
// spread keep them), and `setQueryData`'s structural sharing keeps them too,
// so with the stable `mutate` functions as handlers a toggle re-renders only
// the list and that one item. Filtering stays in component state, and the
// count is derived from the list.
//
// Choices a reader might dispute:
// - No `invalidateQueries` after a mutation (the usual `onSettled` step of the
//   documented pattern). The cache already holds what the server stored, and
//   invalidating would refetch all 2000 items on every click. The add and
//   toggle server functions return the changed item, but writing it back is
//   skipped as well: it equals the optimistic one, and writing a late
//   response could undo a newer optimistic change to the same item.
// - `staleTime: Infinity` on the list query (see `todos.ts`), for the same
//   reason: no refetch after hydration or on window focus.
// - `onError` restores the snapshot taken before that mutation, as the docs
//   show. With several mutations in flight, a failure also rolls back the ones
//   made after it; a reload shows the server's list.
// - The client picks the id of a new todo (the largest id seen plus one) and
//   sends it, so the optimistic item and the stored one agree and its key never
//   changes. A temporary id swapped on success would remount the item.
// - A toggle sends the new `done` value rather than "flip", so it is idempotent.
// - The HTML carries the list twice, as rendered items and as the dehydrated
//   query cache the client hydrates from. That is how the SSR-Query
//   integration works; the cache is what lets the client skip a fetch.
// - Filtered-out items are left out of the rendered list rather than kept with
//   `display: none`. Under "All" every item is in the DOM; no virtualization.

import { useMutation, useQueryClient, useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { memo, useMemo, useRef, useState, type KeyboardEvent } from "react";
import { addTodo, deleteTodo, setTodoDone, todosQuery, type Todo } from "../todos";

export const Route = createFileRoute("/")({
  loader: ({ context }) => context.queryClient.ensureQueryData(todosQuery),
  component: TodoApp,
});

type Filter = "all" | "active" | "completed";

const FILTERS: { value: Filter; text: string }[] = [
  { value: "all", text: "All" },
  { value: "active", text: "Active" },
  { value: "completed", text: "Completed" },
];

type ItemProps = {
  todo: Todo;
  onToggle: (change: { id: number; done: boolean }) => void;
  onDestroy: (id: number) => void;
};

const Item = memo(function Item({ todo, onToggle, onDestroy }: ItemProps) {
  return (
    <li className={todo.done ? "completed" : undefined}>
      <input
        className="toggle"
        type="checkbox"
        checked={todo.done}
        onChange={() => onToggle({ id: todo.id, done: !todo.done })}
      />
      <label>{todo.title}</label>
      <button className="destroy" onClick={() => onDestroy(todo.id)} />
    </li>
  );
});

/** The optimistic-update callbacks for a mutation that changes the list. */
function useOptimisticList<TVariables>(update: (list: Todo[], variables: TVariables) => Todo[]) {
  const queryClient = useQueryClient();
  return {
    onMutate: async (variables: TVariables) => {
      await queryClient.cancelQueries({ queryKey: todosQuery.queryKey });
      const previous = queryClient.getQueryData(todosQuery.queryKey);
      queryClient.setQueryData(todosQuery.queryKey, (list) => (list ? update(list, variables) : list));
      return { previous };
    },
    onError: (_error: unknown, _variables: TVariables, context: { previous?: Todo[] } | undefined) => {
      if (context?.previous) queryClient.setQueryData(todosQuery.queryKey, context.previous);
    },
  };
}

function TodoApp() {
  const { data: todos } = useSuspenseQuery(todosQuery);
  const [filter, setFilter] = useState<Filter>("all");
  const [draft, setDraft] = useState("");
  const nextId = useRef(0);
  if (nextId.current === 0) {
    nextId.current = todos.reduce((max, todo) => Math.max(max, todo.id), 0) + 1;
  }

  const add = useMutation({
    mutationFn: (todo: { id: number; title: string }) => addTodo({ data: todo }),
    ...useOptimisticList((list, todo: { id: number; title: string }) => [...list, { ...todo, done: false }]),
  });

  const toggle = useMutation({
    mutationFn: (change: { id: number; done: boolean }) => setTodoDone({ data: change }),
    ...useOptimisticList((list, { id, done }: { id: number; done: boolean }) =>
      list.map((todo) => (todo.id === id ? { ...todo, done } : todo)),
    ),
  });

  const destroy = useMutation({
    mutationFn: (id: number) => deleteTodo({ data: id }),
    ...useOptimisticList((list, id: number) => list.filter((todo) => todo.id !== id)),
  });

  const onKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== "Enter") return;
    const title = draft.trim();
    if (title === "") return;
    add.mutate({ id: nextId.current++, title });
    setDraft("");
  };

  const left = useMemo(() => todos.reduce((n, todo) => (todo.done ? n : n + 1), 0), [todos]);
  const shown = useMemo(
    () => (filter === "all" ? todos : todos.filter((todo) => todo.done === (filter === "completed"))),
    [todos, filter],
  );

  // `mutate` from `useMutation` keeps its identity across renders, so the
  // memoized items see the same handlers every time.
  const onToggle = toggle.mutate;
  const onDestroy = destroy.mutate;

  return (
    <section className="todoapp">
      <header>
        <h1>todos</h1>
        <input
          className="new-todo"
          placeholder="What needs to be done?"
          value={draft}
          onChange={(event) => setDraft(event.target.value)}
          onKeyDown={onKeyDown}
        />
      </header>
      <section className="main">
        <ul className="todo-list">
          {shown.map((todo) => (
            <Item key={todo.id} todo={todo} onToggle={onToggle} onDestroy={onDestroy} />
          ))}
        </ul>
      </section>
      <footer className="footer">
        <span className="todo-count">
          <strong>{left}</strong> {left === 1 ? "item" : "items"} left
        </span>
        <ul className="filters">
          {FILTERS.map(({ value, text }) => (
            <li key={value}>
              <a
                href="#"
                className={filter === value ? "selected" : undefined}
                onClick={(event) => {
                  event.preventDefault();
                  setFilter(value);
                }}
              >
                {text}
              </a>
            </li>
          ))}
        </ul>
      </footer>
    </section>
  );
}
