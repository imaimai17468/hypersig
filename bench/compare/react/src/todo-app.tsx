// TodoMVC-2000 as a single-page app in React 19 and Vite 8.
//
// Pattern: the HTML is an empty shell. On mount this component fetches the
// list from `GET /api/todos` and holds it in `useState`. Items are
// `React.memo` components keyed by id, and the callbacks they receive are
// stable (`useCallback` with functional state updates), so toggling one item
// re-renders only the list and that item. Each add, toggle, and delete updates
// the state at once and sends the change to the JSON API with `fetch`, without
// awaiting it. Filtering stays in this component's state. The count is derived
// from the list.
//
// Choices a reader might dispute:
// - Plain `useEffect` + `fetch` for the initial load, no data library (TanStack
//   Query, SWR) and no state library: this is the plain React baseline.
// - Writes are fire-and-forget: a failed request is not rolled back or retried.
// - The client picks the id of a new todo (the largest id seen plus one) and
//   sends it to the server, so the optimistic item and the stored one agree.
// - A toggle sends the new `done` value rather than "flip", so it is idempotent.
// - Filtered-out items are left out of the rendered list, the usual React way,
//   rather than kept with `display: none`. Under "All" every item is in the DOM;
//   there is no virtualization.
// - No React Compiler: memoization is written by hand.

import { memo, useCallback, useEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";

type Todo = { id: number; title: string; done: boolean };
type Filter = "all" | "active" | "completed";

const api = {
  add: (todo: Todo) =>
    fetch("/api/todos", { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(todo) }),
  setDone: (id: number, done: boolean) =>
    fetch(`/api/todos/${id}`, { method: "PATCH", headers: { "content-type": "application/json" }, body: JSON.stringify({ done }) }),
  remove: (id: number) => fetch(`/api/todos/${id}`, { method: "DELETE" }),
};

type ItemProps = {
  todo: Todo;
  onToggle: (id: number, done: boolean) => void;
  onDestroy: (id: number) => void;
};

const Item = memo(function Item({ todo, onToggle, onDestroy }: ItemProps) {
  return (
    <li className={todo.done ? "completed" : undefined}>
      <input
        className="toggle"
        type="checkbox"
        checked={todo.done}
        onChange={() => onToggle(todo.id, !todo.done)}
      />
      <label>{todo.title}</label>
      <button className="destroy" onClick={() => onDestroy(todo.id)} />
    </li>
  );
});

const FILTERS: { value: Filter; text: string }[] = [
  { value: "all", text: "All" },
  { value: "active", text: "Active" },
  { value: "completed", text: "Completed" },
];

export default function TodoApp() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [filter, setFilter] = useState<Filter>("all");
  const [draft, setDraft] = useState("");
  const nextId = useRef(1);

  useEffect(() => {
    let ignore = false;
    fetch("/api/todos")
      .then((response) => response.json() as Promise<Todo[]>)
      .then((list) => {
        if (ignore) return;
        nextId.current = list.reduce((max, todo) => Math.max(max, todo.id), 0) + 1;
        setTodos(list);
      });
    return () => {
      ignore = true;
    };
  }, []);

  const onToggle = useCallback((id: number, done: boolean) => {
    setTodos((list) => list.map((todo) => (todo.id === id ? { ...todo, done } : todo)));
    void api.setDone(id, done);
  }, []);

  const onDestroy = useCallback((id: number) => {
    setTodos((list) => list.filter((todo) => todo.id !== id));
    void api.remove(id);
  }, []);

  const onKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== "Enter") return;
    const title = draft.trim();
    if (title === "") return;
    const todo = { id: nextId.current++, title, done: false };
    setTodos((list) => [...list, todo]);
    setDraft("");
    void api.add(todo);
  };

  const left = useMemo(() => todos.reduce((n, todo) => (todo.done ? n : n + 1), 0), [todos]);
  const shown = useMemo(
    () => (filter === "all" ? todos : todos.filter((todo) => todo.done === (filter === "completed"))),
    [todos, filter],
  );

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
