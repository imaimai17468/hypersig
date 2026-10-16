# TodoMVC-2000 results

2026-10-16T02:17:45.612Z, commit d7955b7, Chrome 155.0.8059.41, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 8.9 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 178 ms | 203 ms | 196 ms | 197 ms | 179 ms | 174 ms | 221 ms |
| TBT | 0 ms | 30 ms | 2 ms | 5 ms | 0 ms | 28 ms | 252 ms |
| INP | 24 ms | 16 ms | 24 ms | 24 ms | 24 ms | 16 ms | 16 ms |
