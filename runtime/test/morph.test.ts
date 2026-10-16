import { expect, it } from "vitest";
import { morph } from "../src/morph.ts";

function element(html: string): Element {
  const template = document.createElement("template");
  template.innerHTML = html;
  const first = template.content.firstElementChild;
  if (first === null) throw new Error("no element");
  return first;
}

it("keeps elements matched by id and updates their content", () => {
  document.body.innerHTML = '<ul id="list"><li id="a">A</li><li id="b">B</li></ul>';
  const b = document.getElementById("b");
  morph(document.getElementById("list")!, element('<ul id="list" class="x"><li id="b">B2</li><li id="c">C</li></ul>'));
  expect(document.body.innerHTML).toBe('<ul id="list" class="x"><li id="b">B2</li><li id="c">C</li></ul>');
  expect(document.getElementById("b")).toBe(b);
});

it("replaces an element whose tag changed", () => {
  document.body.innerHTML = '<div id="root"><p>old</p></div>';
  morph(document.getElementById("root")!, element('<div id="root"><span>new</span></div>'));
  expect(document.body.innerHTML).toBe('<div id="root"><span>new</span></div>');
});

it("collects the elements it takes out of the document", () => {
  document.body.innerHTML = '<div id="root"><span id="a">A</span><i>x</i><p>y</p></div>';
  const [a, i, p] = [...document.getElementById("root")!.children];
  const removed: Element[] = [];
  morph(document.getElementById("root")!, element('<div id="root"><b>z</b><span id="a">A</span></div>'), removed);
  expect(document.body.innerHTML).toBe('<div id="root"><b>z</b><span id="a">A</span></div>');
  // The span is replaced at the first position, then moved back by its id.
  expect(removed.length).toBe(3);
  [a, p, i].forEach((child, index) => expect(removed[index]).toBe(child));
  expect(a!.isConnected).toBe(true);
});
