# TodoMVC-2000 results

2026-10-16T13:23:24.926Z, commit c2c9b9c, Chrome 155.0.8059.41, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 13.4 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 179 ms | 190 ms | 184 ms | 198 ms | 188 ms | 172 ms | 235 ms |
| TBT | 0 ms | 27 ms | 2 ms | 5 ms | 0 ms | 25 ms | 233 ms |
| INP | 24 ms | 16 ms | 24 ms | 16 ms | 24 ms | 16 ms | 16 ms |
