# TodoMVC-2000 results

2026-10-25T17:49:19.822Z, commit 7d031d0, Chrome 155.0.8059.85, Apple M4 Max, darwin 25.6.0. CPU slowed 4x and 50 ms added to every request. Medians of 10 runs, with the smallest and largest run in parentheses.

| | hypersig | topcoat | react | next | tanstack-start | nuxt | datastar | leptos | dioxus |
|---|---|---|---|---|---|---|---|---|---|
| JS (gzip) | 19.7 KB | 14.8 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 13.1 KB | 5.5 KB | 15.7 KB |
| Wasm (gzip) | - | - | - | - | - | - | - | 145.1 KB | 441.4 KB |
| FCP | 239 ms (231–309) | 221 ms (206–241) | 209 ms (198–266) | 240 ms (229–261) | 237 ms (217–251) | 236 ms (223–263) | 217 ms (204–249) | 232 ms (222–240) | 213 ms (205–244) |
| TBT | 3 ms (0–37) | 318 ms (311–351) | 16 ms (11–23) | 14 ms (9–16) | 0 ms (0–0) | 36 ms (29–42) | 20 ms (17–23) | 47 ms (42–57) | 122 ms (114–126) |
| INP | 24 ms (24–24) | 24 ms (24–24) | 24 ms (16–24) | 24 ms (16–24) | 24 ms (16–24) | 24 ms (24–24) | 16 ms (16–16) | 24 ms (16–24) | 72 ms (72–80) |
