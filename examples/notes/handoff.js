// The script CI's desktop job runs in each window of the notes example,
// started with no file, while a second start of notes names one: the
// second start hands the file to this copy, whose list passes once the
// file's note is in it.
(() => {
  if (location.pathname.startsWith("/notes/")) return;
  const deadline = Date.now() + 15_000;
  const check = () => {
    if (document.body.textContent.includes("handoff.md")) window.hypersigSmoke = "pass";
    else if (Date.now() > deadline) window.hypersigSmoke = "the file a second start named did not reach the list";
    else setTimeout(check, 100);
  };
  check();
})();
