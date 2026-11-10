// Shows web/index.html in a window the size of the hypersig todo example's,
// hidden until its first paint, then writes how long after the process
// started that was and quits, as a hypersig_desktop smoke run does.
import { app, BrowserWindow } from "electron";

app.whenReady().then(() => {
  const window = new BrowserWindow({ width: 640, height: 720, show: false });
  window.once("ready-to-show", () => {
    window.show();
    console.log(`electron: first page shown ${Math.round(performance.now())} ms after start`);
    app.quit();
  });
  window.loadFile(new URL("../web/index.html", import.meta.url).pathname);
});
