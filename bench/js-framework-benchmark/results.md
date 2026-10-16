# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, 10 runs each, medians. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 28.9 | 20.8 | 25.1 | 24.6 | 21.8 | 25.7 |
| replace all rows | 30.6 | 21.9 | 29.4 | 26.6 | 23.8 | 29.1 |
| partial update (4x slowdown) | 22.8 | 10.7 | 15.0 | 13.6 | 11.9 | 13.1 |
| select row (4x slowdown) | 3.8 | 6.9 | 6.8 | 4.1 | 5.3 | 4.6 |
| swap rows (4x slowdown) | 14.7 | 11.9 | 90.9 | 15.9 | 16.5 | 13.8 |
| remove row (2x slowdown) | 17.6 | 10.3 | 14.4 | 15.9 | 12.2 | 12.9 |
| create many rows | 304.1 | 210.3 | 381.8 | 250.8 | 224.4 | 264.5 |
| append rows to large table | 32.5 | 24.3 | 30.9 | 26.6 | 24.7 | 28.1 |
| clear rows (4x slowdown) | 19.1 | 8.8 | 17.4 | 12.6 | 11.1 | 16.0 |
| ready memory (MB) | 0.64 | 0.56 | 1.13 | 0.86 | 0.59 | 1.71 |
| run memory (MB) | 4.98 | 1.83 | 4.40 | 3.84 | 2.68 | 5.42 |
| create/clear 5 times (MB) | 1.42 | 0.64 | 1.95 | 1.19 | 0.77 | 4.57 |
| size, compressed (KB) | 9.8 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 91.6 | 81.5 | 384.6 | 144.0 | 97.1 | 294.6 |
