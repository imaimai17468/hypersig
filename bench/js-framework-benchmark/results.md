# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, 10 runs each, medians. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 24.7 | 20.8 | 23.2 | 24.3 | 21.9 | 25.1 |
| replace all rows | 27.8 | 21.8 | 29.0 | 27.0 | 23.6 | 29.1 |
| partial update (4x slowdown) | 15.1 | 9.8 | 14.5 | 12.7 | 11.2 | 11.9 |
| select row (4x slowdown) | 3.6 | 3.1 | 6.3 | 4.2 | 7.6 | 5.2 |
| swap rows (4x slowdown) | 13.2 | 11.6 | 90.2 | 13.6 | 12.9 | 12.6 |
| remove row (2x slowdown) | 14.1 | 10.4 | 14.6 | 13.2 | 12.1 | 13.1 |
| create many rows | 264.8 | 210.6 | 388.2 | 254.9 | 229.4 | 261.8 |
| append rows to large table | 29.2 | 23.2 | 27.8 | 26.1 | 24.4 | 28.0 |
| clear rows (4x slowdown) | 13.1 | 9.8 | 16.5 | 11.6 | 11.8 | 14.7 |
| ready memory (MB) | 0.72 | 0.56 | 1.18 | 0.86 | 0.59 | 1.75 |
| run memory (MB) | 3.16 | 1.87 | 4.41 | 3.87 | 2.68 | 5.43 |
| create/clear 5 times (MB) | 1.50 | 0.64 | 1.98 | 1.19 | 0.77 | 4.58 |
| size, compressed (KB) | 12.7 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 66.1 | 45.1 | 248.4 | 98.9 | 55.2 | 250.9 |
