// The server's copy of the list, as SPEC.md describes it. Nitro bundles the
// whole server into one module graph, so a module-level variable is shared by
// every route.

export type Todo = { id: number; title: string; done: boolean };

function seed(): Todo[] {
  return Array.from({ length: 2000 }, (_, index) => {
    const id = index + 1;
    return { id, title: `todo ${id}`, done: id % 3 === 0 };
  });
}

let todos: Todo[] = seed();

export function listTodos(): Todo[] {
  return todos;
}

export function resetTodos(): void {
  todos = seed();
}

export function addTodo(todo: Todo): void {
  todos = [...todos, todo];
}

export function setTodoDone(id: number, done: boolean): void {
  todos = todos.map((todo) => (todo.id === id ? { ...todo, done } : todo));
}

export function removeTodo(id: number): void {
  todos = todos.filter((todo) => todo.id !== id);
}
