<script setup lang="ts">
// TodoMVC-2000 in Nuxt 4 and Vue 3.5.
//
// Pattern: `useFetch('/api/todos')` loads the list during server rendering, so
// the first HTML holds all 2000 items and the client hydrates from the payload.
// The list then lives in a `shallowRef` initialized from that data. Each item
// is a `TodoItem` component keyed by id. Each add, toggle, and delete replaces
// the array (and, for a toggle, that one todo object) at once and then calls a
// server API route with `$fetch`, without awaiting it. Filtering and the count
// are computeds in the browser.
//
// Choices a reader might dispute:
// - `shallowRef` rather than `ref`: with `ref`, Vue would wrap all 2000 todos
//   in reactive proxies and track each field. The Vue docs recommend
//   `shallowRef` for large lists updated by replacement. The cost is that every
//   change replaces the array, so the list re-renders its 2000 item vnodes and
//   Vue's prop check skips all but the changed item.
// - No `refresh()` after a change: the local list is already right, and a
//   refresh would re-send the 2000-item list.
// - The client picks the id of a new todo (the largest id seen plus one) and
//   sends it to the server, so the local item and the stored one agree.
// - A toggle sends the new `done` value rather than "flip", so it is idempotent.
// - The change routes answer `204` with no body rather than the stored todo,
//   because the client never reads the answer.
// - Filtered-out items are left out of the rendered list (a `v-for` over a
//   computed), the usual Vue way, rather than kept with `v-show`. Under "All"
//   every item is in the DOM; there is no virtualization.
// - There is no `pages/` directory, so Nuxt does not bundle Vue Router: the app
//   has one page.
// - The shared stylesheet is put into the HTML by a Nitro `render:html` hook
//   (`server/plugins/todomvc-css.ts`) rather than `useHead` or `app.head`,
//   either of which would also bundle the CSS text into the client JavaScript.
import type { Todo } from "~/types";

type Filter = "all" | "active" | "completed";

const FILTERS: { value: Filter; text: string }[] = [
  { value: "all", text: "All" },
  { value: "active", text: "Active" },
  { value: "completed", text: "Completed" },
];

const { data } = await useFetch<Todo[]>("/api/todos");

const todos = shallowRef<Todo[]>(data.value ?? []);
const filter = ref<Filter>("all");
const draft = ref("");
let nextId = todos.value.reduce((max, todo) => Math.max(max, todo.id), 0) + 1;

const left = computed(() => todos.value.reduce((n, todo) => (todo.done ? n : n + 1), 0));
const shown = computed(() =>
  filter.value === "all" ? todos.value : todos.value.filter((todo) => todo.done === (filter.value === "completed")),
);

function add() {
  const title = draft.value.trim();
  if (title === "") return;
  const id = nextId++;
  todos.value = [...todos.value, { id, title, done: false }];
  draft.value = "";
  void $fetch("/api/todos", { method: "POST", body: { id, title } });
}

function toggle(id: number, done: boolean) {
  todos.value = todos.value.map((todo) => (todo.id === id ? { ...todo, done } : todo));
  void $fetch(`/api/todos/${id}`, { method: "PATCH", body: { done } });
}

function destroy(id: number) {
  todos.value = todos.value.filter((todo) => todo.id !== id);
  void $fetch(`/api/todos/${id}`, { method: "DELETE" });
}
</script>

<template>
  <section class="todoapp">
    <header>
      <h1>todos</h1>
      <input
        v-model="draft"
        class="new-todo"
        placeholder="What needs to be done?"
        @keydown.enter="add"
      >
    </header>
    <section class="main">
      <ul class="todo-list">
        <TodoItem v-for="todo in shown" :key="todo.id" :todo="todo" @toggle="toggle" @destroy="destroy" />
      </ul>
    </section>
    <footer class="footer">
      <span class="todo-count"><strong>{{ left }}</strong> {{ left === 1 ? "item" : "items" }} left</span>
      <ul class="filters">
        <li v-for="f in FILTERS" :key="f.value">
          <a href="#" :class="{ selected: filter === f.value }" @click.prevent="filter = f.value">{{ f.text }}</a>
        </li>
      </ul>
    </footer>
  </section>
</template>
