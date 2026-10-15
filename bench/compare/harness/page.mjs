// Runs in the page before any app script, through `addInitScript`.
//
// `__bench.arm(check)` returns a promise of two times, in milliseconds from the
// next input event: `dom`, when `check` first holds, and `frame`, the first
// frame painted after that. The input event
// is caught on `window` in the capture phase, before any app listener runs.

export function instrument() {
  const count = () => document.querySelector(".todo-count strong")?.textContent?.trim() ?? "";
  const items = () => document.querySelectorAll("ul.todo-list > li");
  const byTitle = (title) => {
    for (const li of items()) {
      if (li.querySelector("label")?.textContent === title) return li;
    }
    return null;
  };
  const visible = () => {
    let n = 0;
    for (const li of items()) if (li.offsetParent !== null) n++;
    return n;
  };
  let target = null;
  const find = (title) => {
    if (target === null || !target.isConnected) target = byTitle(title);
    return target;
  };
  const holds = (c) => {
    switch (c.kind) {
      case "toggle": {
        const li = find(c.title);
        return li !== null && li.classList.contains("completed") === c.done && count() === c.count;
      }
      case "add":
        return count() === c.count && byTitle(c.title) !== null;
      case "delete":
        return count() === c.count && byTitle(c.title) === null;
      case "filter":
        return visible() === c.visible;
      default:
        return false;
    }
  };

  let pending = null;
  const begin = (event) => {
    if (pending !== null && pending.t0 === null && pending.events.includes(event.type)) {
      pending.t0 = performance.now();
    }
  };
  window.addEventListener("pointerdown", begin, { capture: true });
  window.addEventListener("keydown", begin, { capture: true });

  window.__bench = {
    count,
    visible,
    arm(check) {
      target = null;
      return new Promise((resolve) => {
        const p = { t0: null, events: check.kind === "type" || check.kind === "add" ? ["keydown"] : ["pointerdown"], done: false };
        pending = p;
        const finish = () => {
          if (p.done) return;
          p.done = true;
          const dom = performance.now() - p.t0;
          observer.disconnect();
          window.removeEventListener("input", onInput);
          requestAnimationFrame(() => {
            const channel = new MessageChannel();
            channel.port1.onmessage = () => {
              pending = null;
              resolve({ dom, frame: performance.now() - p.t0 });
            };
            channel.port2.postMessage(0);
          });
        };
        const test = () => {
          if (p.t0 !== null && check.kind !== "type" && holds(check)) finish();
        };
        const observer = new MutationObserver(test);
        observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, characterData: true });
        // Bubbling to `window` comes after every app listener has handled the input.
        const onInput = () => {
          if (p.t0 !== null && check.kind === "type") finish();
        };
        window.addEventListener("input", onInput);
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
