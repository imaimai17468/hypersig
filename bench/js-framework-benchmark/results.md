# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, 10 runs each, medians. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations. Size and first paint are measured once per framework, because the runner loads each page for them a single time, so first paint can move by 10 ms or more between runs with nothing changed.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 24.3 | 20.7 | 23.6 | 24.3 | 21.7 | 25.1 |
| replace all rows | 26.9 | 21.9 | 29.2 | 26.3 | 23.6 | 28.9 |
| partial update (4x slowdown) | 15.0 | 22.9 | 13.7 | 12.9 | 11.1 | 11.3 |
| select row (4x slowdown) | 3.3 | 4.8 | 5.8 | 5.1 | 4.0 | 4.5 |
| swap rows (4x slowdown) | 13.0 | 13.0 | 88.1 | 13.4 | 13.1 | 12.4 |
| remove row (2x slowdown) | 13.4 | 9.9 | 13.8 | 12.7 | 12.8 | 12.4 |
| create many rows | 252.9 | 208.3 | 383.3 | 247.9 | 220.4 | 259.5 |
| append rows to large table | 27.8 | 23.0 | 27.4 | 26.3 | 23.8 | 27.8 |
| clear rows (4x slowdown) | 14.2 | 9.1 | 16.4 | 12.2 | 10.4 | 16.4 |
| ready memory (MB) | 0.73 | 0.56 | 1.18 | 0.85 | 0.59 | 1.71 |
| run memory (MB) | 3.06 | 1.87 | 4.39 | 3.84 | 2.68 | 5.42 |
| create/clear 5 times (MB) | 1.37 | 0.64 | 1.93 | 1.11 | 0.76 | 4.58 |
| size, compressed (KB) | 13.1 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 81.1 | 53.8 | 244.9 | 113.7 | 63.4 | 268.3 |
