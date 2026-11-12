// The script CI's desktop job runs in the meter example's window on Linux,
// with HYPERSIG_DESKTOP_SMOKE_SCRIPT naming this file, where a smoke build
// gives WebKitGTK mock capture devices. WebKit shares the screen only from a
// person's click, so the job clicks the middle of the window with xdotool,
// and this script stretches "Share the screen" over the whole window for
// that click to land on. It passes once the page shares the screen.
(() => {
  const deadline = Date.now() + 30_000;
  const stretch = () => {
    const button = document.querySelector("screen-share button");
    if (button === null) return setTimeout(stretch, 100);
    button.style.cssText = "position: fixed; inset: 0; z-index: 1";
  };
  stretch();
  const check = () => {
    const state = document.querySelector("screen-share")?.dataset.state;
    if (state === "sharing") window.hypersigSmoke = "pass";
    else if (state !== undefined) window.hypersigSmoke = `the screen was refused: ${state}`;
    else if (Date.now() > deadline) window.hypersigSmoke = "the screen was not shared";
    else setTimeout(check, 100);
  };
  check();
})();
