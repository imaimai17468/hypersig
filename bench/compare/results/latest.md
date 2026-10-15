# TodoMVC-2000 results

2026-10-15T05:34:12.774Z, commit ddf3f9a, Chrome 155.0.8059.41, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 6.5 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 178 ms | 196 ms | 178 ms | 191 ms | 179 ms | 167 ms | 230 ms |
| TBT | 71 ms | 28 ms | 4 ms | 8 ms | 0 ms | 26 ms | 248 ms |
| INP | 16 ms | 24 ms | 16 ms | 24 ms | 24 ms | 24 ms | 16 ms |
