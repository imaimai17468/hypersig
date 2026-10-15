# TodoMVC-2000 results

2026-10-15T16:49:29.015Z, commit ad939b5, Chrome 155.0.8059.41, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 8.9 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 180 ms | 199 ms | 192 ms | 205 ms | 182 ms | 171 ms | 240 ms |
| TBT | 0 ms | 29 ms | 1 ms | 7 ms | 0 ms | 25 ms | 246 ms |
| INP | 16 ms | 24 ms | 16 ms | 24 ms | 24 ms | 24 ms | 16 ms |
