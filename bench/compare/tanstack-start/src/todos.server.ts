// The server's copy of the list, as SPEC.md describes it.
//
// The `.server.ts` suffix puts this module under Start's import protection, so
// it can never end up in the client bundle. It is imported only from server
// function handlers and the `/bench-reset` server route, which share one
// server bundle, so a module-level variable is the single copy.

export type Todo = { id: number; title: string; done: boolean };

function seed(): Todo[] {
  return Array.from({ length: 2000 }, (_, index) => {
    const id = index + 1;
    return { id, title: `todo ${id}`, done: id % 3 === 0 };
  });
}

let todos = seed();

export function list(): Todo[] {
  return todos.slice();
}

export function reset(): void {
  todos = seed();
}

export function add(todo: Todo): Todo {
  todos.push(todo);
  return todo;
}

export function setDone(id: number, done: boolean): Todo | undefined {
  let changed: Todo | undefined;
  todos = todos.map((todo) => (todo.id === id ? (changed = { ...todo, done }) : todo));
  return changed;
}

export function remove(id: number): void {
  todos = todos.filter((todo) => todo.id !== id);
}
