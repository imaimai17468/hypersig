import { expect, it } from "vitest";
import { $, $$, tick } from "../dom.ts";
import { spyReads } from "../runtime/mount.ts";
import { mount } from "./fixture.ts";

const twoTodos = [
  { id: 1, title: "a", done: false },
  { id: 2, title: "b", done: false },
];

it("does not recheck list keys when a field outside the key changes", async () => {
  const runtime = mount("todos");
  runtime.store.set("todos", twoTodos);
  await tick();
  const reads = spyReads(runtime);
  $$("#list li input[type=checkbox]")[1]!.click();
  await tick();
  expect(runtime.store.get("todos.1.done")).toBe(true);
  expect(reads.filter((path) => path.endsWith(".id"))).toEqual([]);
  expect($("#left").textContent).toBe("1");

  reads.length = 0;
  runtime.store.set("todos.0.id", 7);
  await tick();
  expect(reads).toEqual(expect.arrayContaining(["todos.0.id", "todos.1.id"]));
  $$("#list li .remove")[0]!.click();
  await tick();
  expect($$("#list li label").map((label) => label.textContent)).toEqual(["b"]);
});

it("does not rerun the bindings of the other items when an item is pushed", async () => {
  const runtime = mount("todos");
  runtime.store.set("todos", twoTodos);
  await tick();
  const get = runtime.store.get.bind(runtime.store);
  const reads = spyReads(runtime);
  runtime.store.set("todos", [...(get("todos") as unknown[]), { id: 3, title: "c", done: false }]);
  await tick();
  expect($$("#list li label").map((label) => label.textContent)).toEqual(["a", "b", "c"]);
  expect(reads.filter((path) => /^todos\.[01]\.(id|title)$/u.test(path))).toEqual([]);
  expect(reads.filter((path) => path.endsWith(".title"))).toEqual(["todos.2.title"]);

  runtime.store.set("todos.0.id", 9);
  await tick();
  runtime.store.set("todos", (get("todos") as { id: number }[]).slice(1));
  await tick();
  expect($$("#list li label").map((label) => label.textContent)).toEqual(["b", "c"]);
});

it("reruns only the bindings that read a changed signal", async () => {
  const runtime = mount("todos");
  runtime.store.set("todos", twoTodos);
  await tick();
  const labels = $$("#list label");
  const written = new Set<Node>();
  const observer = new MutationObserver((records) => {
    for (const record of records) written.add(record.target);
  });
  for (const label of labels)
    observer.observe(label, { childList: true, characterData: true, subtree: true });
  runtime.store.set("draft", "typing");
  await tick();
  expect(written.size).toBe(0);
  runtime.store.set("todos.1.title", "B");
  await tick();
  expect(labels.map((label) => label.textContent)).toEqual(["a", "B"]);
  expect([...written]).toEqual([labels[1]]);
  observer.disconnect();
});
