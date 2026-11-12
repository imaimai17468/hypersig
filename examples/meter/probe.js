(async () => {
  const kind = typeof navigator.mediaDevices?.getDisplayMedia;
  setTimeout(() => { window.hypersigSmoke ??= `typeof=${kind}; still pending after 10 s`; }, 10_000);
  let result = "not called";
  if (kind === "function") {
    try {
      const stream = await navigator.mediaDevices.getDisplayMedia({ video: true });
      result = `shared ${stream.getVideoTracks()[0]?.label}`;
    } catch (error) {
      result = `${error.name}: ${error.message}`;
    }
  }
  window.hypersigSmoke ??= `typeof=${kind}; ${result}`;
})();
