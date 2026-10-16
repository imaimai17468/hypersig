# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, 10 runs each, medians. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 29.3 | 21.1 | 24.2 | 24.6 | 21.9 | 25.1 |
| replace all rows | 30.7 | 22.1 | 29.4 | 26.8 | 23.7 | 29.2 |
| partial update (4x slowdown) | 19.1 | 9.9 | 15.1 | 12.7 | 12.3 | 12.1 |
| select row (4x slowdown) | 4.5 | 7.1 | 5.9 | 3.9 | 5.3 | 5.7 |
| swap rows (4x slowdown) | 13.2 | 12.8 | 88.2 | 13.0 | 12.4 | 12.4 |
| remove row (2x slowdown) | 14.3 | 10.6 | 14.8 | 12.4 | 12.1 | 12.5 |
| create many rows | 313.5 | 208.3 | 409.3 | 247.6 | 224.4 | 261.6 |
| append rows to large table | 31.0 | 23.1 | 27.8 | 26.3 | 24.1 | 27.8 |
| clear rows (4x slowdown) | 13.4 | 10.7 | 16.4 | 11.8 | 10.6 | 14.8 |
| ready memory (MB) | 0.65 | 0.56 | 1.18 | 0.86 | 0.59 | 1.74 |
| run memory (MB) | 5.27 | 1.87 | 4.41 | 3.91 | 2.68 | 5.44 |
| create/clear 5 times (MB) | 1.52 | 0.64 | 1.95 | 1.19 | 0.76 | 4.57 |
| size, compressed (KB) | 10.4 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 115.3 | 111.3 | 274.0 | 150.5 | 100.7 | 297.4 |
