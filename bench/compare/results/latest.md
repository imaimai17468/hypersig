# TodoMVC-2000 results

2026-10-16T10:10:12.721Z, commit e741bff, Chrome 155.0.8059.41, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs.

| | hypersig | leptos | react | next | tanstack-start | nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 13.0 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 185 ms | 196 ms | 192 ms | 194 ms | 188 ms | 184 ms | 236 ms |
| TBT | 0 ms | 24 ms | 0 ms | 4 ms | 0 ms | 20 ms | 229 ms |
| INP | 24 ms | 16 ms | 24 ms | 24 ms | 16 ms | 16 ms | 16 ms |
