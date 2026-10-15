// The server's copy of the list, as SPEC.md describes it.
//
// Next.js builds the page, the server actions, and the route handler as
// separate server entries, so a plain module-level variable could exist once
// per entry. The list lives on `globalThis` so every entry sees the same one.

import "server-only";

export type Todo = { id: number; title: string; done: boolean };

function seed(): Todo[] {
  return Array.from({ length: 2000 }, (_, index) => {
    const id = index + 1;
    return { id, title: `todo ${id}`, done: id % 3 === 0 };
  });
}

const store = globalThis as typeof globalThis & { __benchTodos?: Todo[] };

function todos(): Todo[] {
  return (store.__benchTodos ??= seed());
}

export function list(): Todo[] {
  return todos().slice();
}

export function reset(): void {
  store.__benchTodos = seed();
}

export function add(todo: Todo): void {
  todos().push(todo);
}

export function setDone(id: number, done: boolean): void {
  store.__benchTodos = todos().map((todo) => (todo.id === id ? { ...todo, done } : todo));
}

export function remove(id: number): void {
  store.__benchTodos = todos().filter((todo) => todo.id !== id);
}
