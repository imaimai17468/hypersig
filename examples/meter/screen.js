// The script CI's desktop job runs in the meter example's window on Linux,
// with HYPERSIG_DESKTOP_SMOKE_SCRIPT naming this file, where a smoke build
// gives WebKitGTK mock capture devices. It clicks "Share the screen" and
// passes once the page shares it, which needs the web view to grant it.
(() => {
  const deadline = Date.now() + 15_000;
  // The page has loaded, but the runtime may not have started on it yet.
  setTimeout(() => document.querySelector("screen-share button").click(), 1000);
  const check = () => {
    const state = document.querySelector("screen-share")?.dataset.state;
    if (state === "sharing") window.hypersigSmoke = "pass";
    else if (state !== undefined) window.hypersigSmoke = `the screen was refused: ${state}`;
    else if (Date.now() > deadline) window.hypersigSmoke = "the screen was not shared";
    else setTimeout(check, 100);
  };
  check();
})();
