# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, medians. hypersig and vanillajs were measured at commit 72e62b8 with 5 runs each. The other four columns are from an earlier run of the same day with 10 runs each, which also measured vanillajs; its times then were within 0.5 ms of these except for select row. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations. Size and first paint are measured once per framework, because the runner loads each page for them a single time, so first paint can move by 100 ms or more between runs with nothing changed: vanillajs gave 66.9 ms in the earlier run and 196.7 ms in this one.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 23.9 | 20.1 | 24.3 | 24.3 | 21.6 | 25.2 |
| replace all rows | 26.5 | 21.9 | 29.6 | 27.2 | 23.8 | 29.1 |
| partial update (4x slowdown) | 14.9 | 11.2 | 14.9 | 16.0 | 11.9 | 13.2 |
| select row (4x slowdown) | 7.1 | 3.0 | 6.2 | 7.5 | 5.8 | 4.2 |
| swap rows (4x slowdown) | 14.5 | 13.5 | 93.1 | 14.6 | 14.2 | 13.4 |
| remove row (2x slowdown) | 14.6 | 10.9 | 13.9 | 13.2 | 12.1 | 12.5 |
| create many rows | 248.6 | 211.6 | 405.6 | 251.3 | 225.6 | 267.8 |
| append rows to large table | 27.4 | 22.8 | 28.2 | 26.7 | 24.1 | 28.1 |
| clear rows (4x slowdown) | 12.0 | 9.5 | 17.1 | 12.7 | 10.9 | 16.0 |
| ready memory (MB) | 0.84 | 0.56 | 1.19 | 0.86 | 0.59 | 1.76 |
| run memory (MB) | 3.09 | 1.87 | 4.43 | 3.91 | 2.68 | 5.43 |
| create/clear 5 times (MB) | 1.32 | 0.64 | 1.98 | 1.19 | 0.77 | 4.57 |
| size, compressed (KB) | 18.3 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 96.5 | 196.7 | 260.9 | 116.4 | 66.7 | 261.0 |
