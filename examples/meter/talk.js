// The script CI's desktop job runs in the meter example's window on Linux,
// with HYPERSIG_DESKTOP_SMOKE_SCRIPT naming this file, while xdotool presses
// and releases Control+Shift+Space on the X server. The run passes once the
// page has said "Talking" and then stopped saying it, which takes the
// press and the release of the global shortcut, each sent over the page's
// stream.
(() => {
  const deadline = Date.now() + 15_000;
  let held = false;
  const check = () => {
    const talking = document.body.innerText.includes("Talking");
    if (talking) held = true;
    if (held && !talking) window.hypersigSmoke = "pass";
    else if (Date.now() > deadline) {
      window.hypersigSmoke = held ? "the release of the key did not reach the page" : "the press of the key did not reach the page";
    } else setTimeout(check, 50);
  };
  check();
})();
