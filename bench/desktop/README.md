# Desktop start-up

How long after its process starts each desktop build of the same todo list shows its first page, and how large each build is.

| Build | What it runs |
|---|---|
| hypersig | `examples/todo` with `--features desktop`: the page rendered by the server in the process, shown in the system's web view by `hypersig_desktop` |
| tauri | `tauri/`, Tauri 2.12.5, showing `web/index.html`, the same list written in plain JavaScript |
| electron | `electron/main.mjs`, Electron 38.2.0, showing `web/index.html` |

Each build writes a line with `first page shown` when its window shows the first page and then quits: hypersig in a `HYPERSIG_DESKTOP_SMOKE` run, Tauri when the page has loaded, and Electron on `ready-to-show`. `measure.mjs` starts each build five times to warm up, since macOS checks a newly built executable on its first starts, then 20 times, and takes the time from spawning the process to that line. Size counts the executable, or for Electron the Electron distribution with the app's files.

## Running

```sh
cargo build --release -p example-todo --features desktop
(cd bench/desktop/tauri && cargo build --release)
cd bench/desktop && pnpm install && node measure.mjs
```

The results go to `results/<date>-<system>-<arch>.{json,md}`.

## Results

[results/2026-11-10-darwin-arm64.md](results/2026-11-10-darwin-arm64.md), on an Apple M4 Max:

| Build | Median (ms) | p95 (ms) | Size (MiB) |
|---|---|---|---|
| hypersig | 482 | 534 | 4.7 |
| tauri | 455 | 639 | 9.8 |
| electron | 489 | 679 | 269.9 |

The three show their first page within about 35 ms of each other at the median; Tauri's is the soonest. hypersig's build is half the size of Tauri's.

## Not measured

- A cold start, after the system's caches are emptied, which needs `sudo purge` or a restart between runs.
- Memory. On macOS, WKWebView's processes are started by the system rather than by the application, so they cannot be counted as the application's from its process tree.
- Tauri with a React front end, which needs a bundler build of its own.
- Windows and Linux.
