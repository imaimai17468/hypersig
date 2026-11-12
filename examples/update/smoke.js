// The script CI's desktop job runs in the update example's window on Linux,
// with HYPERSIG_DESKTOP_SMOKE_SCRIPT naming this file, while 127.0.0.1
// serves a newer signed version. It clicks "Check for updates" and passes
// once the page says the new version was installed.
(() => {
  const deadline = Date.now() + 15_000;
  // The page has loaded, but the runtime may not have started on it yet.
  setTimeout(() => document.querySelector("button").click(), 1000);
  const check = () => {
    const status = document.querySelector("p").textContent;
    if (status.startsWith("Version ")) window.hypersigSmoke = "pass";
    else if (status !== "") window.hypersigSmoke = status;
    else if (Date.now() > deadline) window.hypersigSmoke = "the update did not finish";
    else setTimeout(check, 100);
  };
  check();
})();
