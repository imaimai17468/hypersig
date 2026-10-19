# TodoMVC-2000 results

Chrome 155.0.8059.80, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 3 runs. topcoat and datastar were measured at 2026-10-19T15:03:09.124Z, commit 9548520, and the other apps at 2026-10-19T03:54:48.597Z, commit 72e62b8, under the same conditions.

| | hypersig | topcoat | react | next | tanstack-start | nuxt | datastar |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 19.2 KB | 14.8 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 13.1 KB |
| FCP | 186 ms | 165 ms | 196 ms | 198 ms | 193 ms | 177 ms | 182 ms |
| TBT | 0 ms | 313 ms | 5 ms | 10 ms | 0 ms | 29 ms | 16 ms |
| INP | 24 ms | 24 ms | 24 ms | 24 ms | 24 ms | 16 ms | 16 ms |
