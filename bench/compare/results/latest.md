# TodoMVC-2000 results

2026-10-16T06:27:43.266Z, commit 01d8a91, Chrome 155.0.8059.41, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 10.4 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 173 ms | 197 ms | 192 ms | 181 ms | 192 ms | 176 ms | 238 ms |
| TBT | 0 ms | 28 ms | 3 ms | 5 ms | 0 ms | 28 ms | 239 ms |
| INP | 16 ms | 24 ms | 24 ms | 24 ms | 24 ms | 16 ms | 16 ms |
