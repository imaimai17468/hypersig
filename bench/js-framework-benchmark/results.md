# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, 10 runs each, medians. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 28.5 | 20.8 | 23.4 | 24.7 | 21.6 | 25.4 |
| replace all rows | 32.4 | 22.0 | 28.9 | 28.3 | 24.1 | 29.3 |
| partial update (4x slowdown) | 23.4 | 10.1 | 14.0 | 12.1 | 10.8 | 11.8 |
| select row (4x slowdown) | 3.3 | 5.0 | 6.1 | 4.7 | 4.5 | 6.3 |
| swap rows (4x slowdown) | 20.1 | 24.9 | 88.9 | 14.2 | 12.9 | 12.4 |
| remove row (2x slowdown) | 19.5 | 10.0 | 13.7 | 12.2 | 12.8 | 12.4 |
| create many rows | 306.9 | 206.9 | 374.4 | 243.7 | 220.6 | 256.6 |
| append rows to large table | 30.6 | 22.9 | 27.4 | 25.9 | 24.3 | 27.6 |
| clear rows (4x slowdown) | 22.8 | 10.5 | 16.6 | 11.6 | 11.3 | 14.1 |
| ready memory (MB) | 0.64 | 0.56 | 1.18 | 0.86 | 0.59 | 1.72 |
| run memory (MB) | 5.40 | 1.87 | 4.35 | 3.90 | 2.68 | 5.43 |
| create/clear 5 times (MB) | 1.32 | 0.64 | 1.90 | 1.19 | 0.77 | 4.57 |
| size, compressed (KB) | 9.0 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 94.3 | 151.2 | 265.2 | 135.1 | 105.2 | 258.9 |
