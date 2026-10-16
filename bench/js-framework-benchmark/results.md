# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, 10 runs each, medians. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 28.9 | 21.1 | 23.4 | 24.4 | 22.3 | 25.3 |
| replace all rows | 30.9 | 22.1 | 29.4 | 26.4 | 23.5 | 28.9 |
| partial update (4x slowdown) | 20.1 | 9.9 | 14.6 | 13.2 | 10.5 | 11.7 |
| select row (4x slowdown) | 3.5 | 4.9 | 6.6 | 4.5 | 5.8 | 4.5 |
| swap rows (4x slowdown) | 14.6 | 12.4 | 89.8 | 14.6 | 12.9 | 12.9 |
| remove row (2x slowdown) | 15.4 | 10.7 | 14.5 | 14.1 | 12.7 | 12.6 |
| create many rows | 319.1 | 213.7 | 387.4 | 253.1 | 227.9 | 267.7 |
| append rows to large table | 31.5 | 23.1 | 27.8 | 26.4 | 24.6 | 27.8 |
| clear rows (4x slowdown) | 15.6 | 8.8 | 16.1 | 11.9 | 11.4 | 16.4 |
| ready memory (MB) | 0.65 | 0.56 | 1.17 | 0.85 | 0.59 | 1.71 |
| run memory (MB) | 5.37 | 1.87 | 4.38 | 3.91 | 2.68 | 5.44 |
| create/clear 5 times (MB) | 1.47 | 0.62 | 1.97 | 1.17 | 0.77 | 4.58 |
| size, compressed (KB) | 10.2 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 62.5 | 78.0 | 277.8 | 168.7 | 99.9 | 296.5 |
