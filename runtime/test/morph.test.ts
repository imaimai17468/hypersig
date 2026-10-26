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
  const b = document.querySelector("#b");
  morph(
    document.querySelector("#list")!,
    element('<ul id="list" class="x"><li id="b">B2</li><li id="c">C</li></ul>'),
  );
  expect(document.body.innerHTML).toBe(
    '<ul id="list" class="x"><li id="b">B2</li><li id="c">C</li></ul>',
  );
  expect(document.querySelector("#b")).toBe(b);
});

it("replaces an element whose tag changed", () => {
  document.body.innerHTML = '<div id="root"><p>old</p></div>';
  morph(document.querySelector("#root")!, element('<div id="root"><span>new</span></div>'));
  expect(document.body.innerHTML).toBe('<div id="root"><span>new</span></div>');
});

it("keeps the elements drawn after a template it keeps, and drops those after one it replaces", () => {
  document.body.innerHTML =
    '<div id="root"><template>a</template><li>1</li><li>2</li><template>b</template><li>3</li></div>';
  const [kept, one, two, replaced, three] = [...document.querySelector("#root")!.children];
  const drawn = new Map<Element, Set<Element>>([
    [kept!, new Set([one!, two!])],
    [replaced!, new Set([three!])],
  ]);
  const removed: Element[] = [];
  morph(
    document.querySelector("#root")!,
    element('<div id="root"><template>a</template><template>c</template></div>'),
    removed,
    (template) => drawn.get(template),
  );
  expect(document.body.innerHTML).toBe(
    '<div id="root"><template>a</template><li>1</li><li>2</li><template>c</template></div>',
  );
  expect(removed).toEqual([replaced, three]);
});

it("collects the elements it takes out of the document", () => {
  document.body.innerHTML = '<div id="root"><span id="a">A</span><i>x</i><p>y</p></div>';
  const [a, i, p] = [...document.querySelector("#root")!.children];
  const removed: Element[] = [];
  morph(
    document.querySelector("#root")!,
    element('<div id="root"><b>z</b><span id="a">A</span></div>'),
    removed,
  );
  expect(document.body.innerHTML).toBe('<div id="root"><b>z</b><span id="a">A</span></div>');
  // The span is replaced at the first position, then moved back by its id.
  expect(removed.length).toBe(3);
  [a, p, i].forEach((child, index) => expect(removed[index]).toBe(child));
  expect(a!.isConnected).toBe(true);
});

it("keeps a foreign element's attributes and children, and finds it by its name", () => {
  document.body.innerHTML =
    '<div id="root"><p>1</p><div data-hs-foreign="map" data-hs-ref="map"></div></div>';
  const map = document.querySelector("[data-hs-foreign]")!;
  map.classList.add("leaflet-container");
  map.append(document.createElement("canvas"));
  morph(
    document.querySelector("#root")!,
    element(
      '<div id="root"><p>0</p><p>1</p><div data-hs-foreign="map" data-hs-ref="map"></div></div>',
    ),
  );
  expect(document.body.innerHTML).toBe(
    '<div id="root"><p>0</p><p>1</p><div data-hs-foreign="map" data-hs-ref="map" class="leaflet-container"><canvas></canvas></div></div>',
  );
  expect(document.querySelector("[data-hs-foreign]")).toBe(map);
});

it("keeps a foreign element's id, so a script finds the same element by it after a patch", () => {
  document.body.innerHTML =
    '<div id="root"><p>1</p><div data-hs-foreign="office::MAP" data-hs-ref="office::MAP" id="map"></div></div>';
  const map = document.querySelector("#map")!;
  map.classList.add("leaflet-container");
  morph(
    document.querySelector("#root")!,
    element(
      '<div id="root"><p>0</p><p>1</p><div data-hs-foreign="office::MAP" data-hs-ref="office::MAP" id="map"></div></div>',
    ),
  );
  expect(document.querySelector("#map")).toBe(map);
  expect(map.classList.contains("leaflet-container")).toBe(true);
});

it("replaces a foreign element with one of another name", () => {
  document.body.innerHTML =
    '<div id="root"><div data-hs-foreign="map"><canvas></canvas></div></div>';
  morph(
    document.querySelector("#root")!,
    element('<div id="root"><div data-hs-foreign="chart"></div></div>'),
  );
  expect(document.body.innerHTML).toBe('<div id="root"><div data-hs-foreign="chart"></div></div>');
});

it("keeps a details element open or closed as the visitor left it", () => {
  document.body.innerHTML =
    '<div id="root"><details id="a"><summary>A</summary></details><details id="b" open><summary>B</summary></details></div>';
  const [a, b] = [...document.querySelectorAll("details")];
  a!.open = true;
  b!.open = false;
  morph(
    document.querySelector("#root")!,
    element(
      '<div id="root"><details id="a" class="x"><summary>A2</summary></details><details id="b" open><summary>B</summary></details></div>',
    ),
  );
  expect(a!.open).toBe(true);
  expect(a!.className).toBe("x");
  expect(a!.textContent).toBe("A2");
  expect(b!.open).toBe(false);
});

it("keeps a modal dialog open", () => {
  document.body.innerHTML = '<div id="root"><dialog id="d"><p>1</p></dialog></div>';
  const dialog = document.querySelector("dialog")!;
  dialog.showModal();
  morph(
    document.querySelector("#root")!,
    element('<div id="root"><dialog id="d"><p>2</p></dialog></div>'),
  );
  expect(dialog.open).toBe(true);
  expect(dialog.textContent).toBe("2");
});
