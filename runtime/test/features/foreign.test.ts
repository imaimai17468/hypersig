import { expect, it } from "vitest";
import { $, tick } from "../dom.ts";
import { events, mount } from "./fixture.ts";

it("keeps what a script added to a foreign element through a patch, and keeps its bindings", async () => {
  const runtime = mount("widgets");
  await tick();
  const map = $("[data-hs-ref=map]");
  expect(map.dataset.stars).toBe("0");
  map.classList.add("leaflet-container");
  map.style.height = "200px";
  map.append(document.createElement("canvas"));

  const [patch] = events("widgets_patch");
  runtime.apply(patch!);
  await tick();
  expect($("#atlas p").textContent).toBe("Kyoto");
  expect($("[data-hs-ref=map]")).toBe(map);
  expect(map.className).toBe("leaflet-container");
  expect(map.style.height).toBe("200px");
  expect(map.querySelector("canvas")).not.toBeNull();

  runtime.store.set("stars", 4);
  await tick();
  expect(map.dataset.stars).toBe("4");
});
