// The script CI's desktop job runs in the meter example's window, with
// HYPERSIG_DESKTOP_SMOKE_SCRIPT naming this file, on a system that gives the
// web view a fake microphone and camera. The meter allows the microphone
// alone, so the run passes once the page has opened the microphone and the
// camera has been refused.
(() => {
  const deadline = Date.now() + 15_000;
  const camera = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: true });
      stream.getTracks().forEach((track) => track.stop());
      window.hypersigSmoke = "the camera was granted, which the meter does not allow";
    } catch (error) {
      window.hypersigSmoke =
        error.name === "NotAllowedError" ? "pass" : `the camera failed with ${error.name}, not NotAllowedError`;
    }
  };
  const check = () => {
    const state = document.querySelector("mic-level")?.dataset.state;
    if (state === "listening") camera();
    else if (state !== undefined) window.hypersigSmoke = `the microphone was refused: ${state}`;
    else if (Date.now() > deadline) window.hypersigSmoke = "the meter did not open the microphone";
    else setTimeout(check, 100);
  };
  check();
})();
