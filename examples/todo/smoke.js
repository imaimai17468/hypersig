// The script CI's desktop job runs in each window of the todo example, with
// HYPERSIG_DESKTOP_SMOKE_SCRIPT naming this file. The first window adds a
// todo, which the server sends back over the page's stream, then opens a
// second window at `/#second`, which adds another; the first passes once
// that one reaches it too.
(() => {
  const count = () => document.querySelectorAll("li").length;
  const add = (title) => {
    const input = document.querySelector("form input");
    input.value = title;
    input.dispatchEvent(new Event("input", { bubbles: true }));
    document.querySelector("form").requestSubmit();
  };
  const wait = (done, failure, then) => {
    const deadline = Date.now() + 10_000;
    const check = () => {
      if (done()) then();
      else if (Date.now() > deadline) window.hypersigSmoke = failure;
      else setTimeout(check, 100);
    };
    check();
  };
  // The page has loaded, but the runtime may not have started on it yet.
  setTimeout(() => {
    if (location.hash === "#second") {
      add("added in the second window");
      return;
    }
    add("added in the first window");
    wait(() => count() === 1, "the todo did not come back over the stream", () => {
      window.open("/#second");
      wait(() => count() === 2, "the second window's todo did not reach the first", () => {
        window.hypersigSmoke = "pass";
      });
    });
  }, 1000);
})();
