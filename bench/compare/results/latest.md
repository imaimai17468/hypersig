# TodoMVC-2000 results

2026-10-18T16:24:42.767Z, commit 15b02b5, Chrome 155.0.8059.80, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 17.9 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 185 ms | 200 ms | 187 ms | 198 ms | 186 ms | 178 ms | 235 ms |
| TBT | 0 ms | 29 ms | 8 ms | 9 ms | 0 ms | 32 ms | 266 ms |
| INP | 24 ms | 24 ms | 16 ms | 24 ms | 24 ms | 24 ms | 16 ms |
