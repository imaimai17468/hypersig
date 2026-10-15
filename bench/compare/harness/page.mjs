// Runs in the page before any app script, through `addInitScript`.
//
// `__bench.arm(check)` returns a promise of the milliseconds from the next
// input event to the moment `check` first holds in the DOM. The input event is
// caught on `window` in the capture phase, before any app listener runs.

export function instrument() {
  const count = () => document.querySelector(".todo-count strong")?.textContent?.trim() ?? "";
  const byTitle = (title) => {
    for (const li of document.querySelectorAll("ul.todo-list > li")) {
      if (li.querySelector("label")?.textContent === title) return li;
    }
    return null;
  };
  let target = null;
  const holds = (c) => {
    if (count() !== c.count) return false;
    if (c.kind === "add") return byTitle(c.title) !== null;
    if (target === null || !target.isConnected) target = byTitle(c.title);
    return target !== null && target.classList.contains("completed");
  };

  let pending = null;
  const begin = (event) => {
    if (pending !== null && pending.t0 === null && pending.event === event.type) pending.t0 = performance.now();
  };
  window.addEventListener("pointerdown", begin, { capture: true });
  window.addEventListener("keydown", begin, { capture: true });

  window.__bench = {
    count,
    arm(check) {
      target = null;
      return new Promise((resolve) => {
        const p = { t0: null, event: check.kind === "add" ? "keydown" : "pointerdown", done: false };
        pending = p;
        const test = () => {
          if (p.done || p.t0 === null || !holds(check)) return;
          p.done = true;
          observer.disconnect();
          pending = null;
          resolve(performance.now() - p.t0);
        };
        const observer = new MutationObserver(test);
        observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, characterData: true });
        const poll = () => {
          if (p.done) return;
          test();
          requestAnimationFrame(poll);
        };
        requestAnimationFrame(poll);
      });
    },
  };
}
