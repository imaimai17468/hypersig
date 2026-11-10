// The script CI's desktop job runs in each window of the notes example,
// started with one file, with HYPERSIG_DESKTOP_SMOKE_SCRIPT naming this file.
// The note's window, which the application opens for the file, edits the
// text and saves it; the list window passes once the save has reached it
// over its stream.
(() => {
  if (location.pathname.startsWith("/notes/")) {
    // The page has loaded, but the runtime may not have started on it yet.
    setTimeout(() => {
      const text = document.querySelector("textarea");
      text.value = "Saved by the smoke run.\n";
      text.dispatchEvent(new Event("input", { bubbles: true }));
      [...document.querySelectorAll("button")].find((button) => button.textContent === "Save").click();
    }, 1000);
    return;
  }
  const deadline = Date.now() + 15_000;
  const check = () => {
    if (document.body.textContent.includes("1 saves")) window.hypersigSmoke = "pass";
    else if (Date.now() > deadline) window.hypersigSmoke = "the save in the note's window did not reach the list";
    else setTimeout(check, 100);
  };
  check();
})();
