import { expect, it } from "vitest";
import { DepIndex, type Effect } from "../src/deps.ts";
import { affects, type Reads } from "../src/store.ts";

const segments = ["a", "b", "0", "1", "c"];

function randomPath(next: () => number): string {
  const depth = 1 + Math.floor(next() * 3);
  return Array.from({ length: depth }, () => segments[Math.floor(next() * segments.length)]!).join(".");
}

it("finds exactly the bindings that `affects` says a change reaches", () => {
  let seed = 7;
  const next = () => ((seed = (seed * 1103515245 + 12345) % 2 ** 31) / 2 ** 31);
  const index = new DepIndex();
  const effects: Effect[] = [];
  for (let id = 0; id < 200; id++) {
    const reads: Reads = new Map();
    for (let k = 0; k < 3; k++) reads.set(randomPath(next), next() < 0.5);
    const effect: Effect = { id, element: document.body, run: () => {}, deps: null, nodes: new Map() };
    index.set(effect, reads);
    effects.push(effect);
  }
  for (let round = 0; round < 300; round++) {
    const moved = effects[Math.floor(next() * effects.length)]!;
    const reads: Reads = new Map(moved.deps!);
    reads.delete([...reads.keys()][0]!);
    reads.set(randomPath(next), next() < 0.5);
    index.set(moved, reads);
    const changes = new Map([
      [randomPath(next), next() < 0.5],
      [randomPath(next), next() < 0.5],
    ]);
    const expected = effects.filter((effect) => affects(effect.deps!, changes));
    expect([...index.affected(changes)].sort((a, b) => a.id - b.id)).toEqual(expected);
  }
});

it("forgets a binding's old reads when it is registered again", () => {
  const index = new DepIndex();
  const effect: Effect = { id: 0, element: document.body, run: () => {}, deps: null, nodes: new Map() };
  index.set(effect, new Map([["a", true]]));
  index.set(effect, new Map([["b", true]]));
  expect(index.affected(new Map([["a", true]])).size).toBe(0);
  expect(index.affected(new Map([["b", true]])).size).toBe(1);
  index.remove(effect);
  expect(index.affected(new Map([["b", true]])).size).toBe(0);
});
