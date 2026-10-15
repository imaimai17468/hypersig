# js-framework-benchmark results

Keyed, headless Chrome 155 on an Apple M4 Max, 10 runs each, medians. Times are in milliseconds, from the click until the browser has painted, with the CPU slowdown the benchmark sets for some operations.

| | hypersig | vanillajs | React 19.2 | Vue 3.5 | Solid 1.9 | Leptos 0.7 |
|---|---|---|---|---|---|---|
| create rows | 30.4 | 21.0 | 23.0 | 24.5 | 21.6 | 25.3 |
| replace all rows | 35.1 | 22.1 | 29.1 | 26.5 | 23.7 | 29.1 |
| partial update (4x slowdown) | 20.1 | 10.1 | 14.6 | 12.7 | 10.8 | 11.6 |
| select row (4x slowdown) | 8.7 | 3.2 | 6.5 | 4.0 | 5.2 | 6.3 |
| swap rows (4x slowdown) | 87.8 | 12.0 | 90.7 | 13.5 | 13.4 | 13.0 |
| remove row (2x slowdown) | 52.9 | 10.7 | 14.7 | 13.6 | 12.3 | 12.9 |
| create many rows | 338.4 | 208.7 | 388.3 | 247.6 | 232.7 | 265.2 |
| append rows to large table | 34.1 | 23.1 | 28.3 | 26.0 | 24.3 | 27.9 |
| clear rows (4x slowdown) | 25.1 | 10.1 | 16.7 | 12.1 | 11.0 | 14.7 |
| ready memory (MB) | 0.62 | 0.56 | 1.15 | 0.86 | 0.59 | 1.74 |
| run memory (MB) | 4.91 | 1.86 | 4.37 | 3.89 | 2.66 | 5.41 |
| create/clear 5 times (MB) | 5.11 | 0.64 | 1.93 | 1.12 | 0.77 | 4.58 |
| size, compressed (KB) | 7.1 | 2.5 | 51.4 | 23.3 | 4.5 | 48.8 |
| first paint | 42 | 67.7 | 281.9 | 113.9 | 116.5 | 279.9 |
