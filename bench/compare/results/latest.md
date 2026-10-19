# TodoMVC-2000 results

2026-10-19T03:54:48.597Z, commit 72e62b8, Chrome 155.0.8059.80, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 19.2 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 186 ms | 203 ms | 196 ms | 198 ms | 193 ms | 177 ms | 236 ms |
| TBT | 0 ms | 26 ms | 5 ms | 10 ms | 0 ms | 29 ms | 268 ms |
| INP | 24 ms | 24 ms | 24 ms | 24 ms | 24 ms | 16 ms | 16 ms |
