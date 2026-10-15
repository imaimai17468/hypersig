"use client";

// TodoMVC-2000 in Next.js 16 (App Router) and React 19.
//
// Pattern: the server component in `page.tsx` reads the list and renders this
// client component, which holds the list in `useState`. Items are `React.memo`
// components keyed by id, and the callbacks they receive are stable
// (`useCallback` with functional state updates), so toggling one item
// re-renders only the list and that item. Each add, toggle, and delete updates
// the state at once and calls a server action (`actions.ts`) without awaiting
// it. Filtering stays in this component's state. The count is derived from the
// list.
//
// Choices a reader might dispute:
// - No `revalidatePath` or `router.refresh()` after an action: the client state
//   is already correct, and a refresh would re-send the 2000-item list.
// - The client picks the id of a new todo (the largest id seen plus one) and
//   sends it to the server, so the optimistic item and the stored one agree.
// - A toggle sends the new `done` value rather than "flip", so it is idempotent.
// - Filtered-out items are left out of the rendered list, the usual React way,
//   rather than kept with `display: none`. Under "All" every item is in the DOM;
//   there is no virtualization.
// - The React Compiler is off, because Next.js 16.4.1 enables it only through
//   `reactCompiler: true`. Memoization is written by hand.

import { memo, useCallback, useMemo, useRef, useState, type KeyboardEvent } from "react";
import { addTodo, deleteTodo, setTodoDone } from "./actions";

type Todo = { id: number; title: string; done: boolean };
type Filter = "all" | "active" | "completed";

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

export default function TodoApp({ initialTodos }: { initialTodos: Todo[] }) {
  const [todos, setTodos] = useState(initialTodos);
  const [filter, setFilter] = useState<Filter>("all");
  const [draft, setDraft] = useState("");
  const nextId = useRef(0);
  if (nextId.current === 0) {
    nextId.current = initialTodos.reduce((max, todo) => Math.max(max, todo.id), 0) + 1;
  }

  const onToggle = useCallback((id: number, done: boolean) => {
    setTodos((list) => list.map((todo) => (todo.id === id ? { ...todo, done } : todo)));
    void setTodoDone(id, done);
  }, []);

  const onDestroy = useCallback((id: number) => {
    setTodos((list) => list.filter((todo) => todo.id !== id));
    void deleteTodo(id);
  }, []);

  const onKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== "Enter") return;
    const title = draft.trim();
    if (title === "") return;
    const id = nextId.current++;
    setTodos((list) => [...list, { id, title, done: false }]);
    setDraft("");
    void addTodo(id, title);
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
