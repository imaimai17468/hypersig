# Styling TodoMVC-2000

Measured at 2026-10-20T01:00:11.544Z, commit 47598a5, in Chrome 155.0.8059.85 with the CPU slowed 4x. Browser values are medians of 5 runs, and the render time is the median of 50. @tailwindcss/cli 4.3.3, @pandacss/dev 2.1.1, @stylexjs/stylex 0.19.1, @vanilla-extract/css 1.21.2, esbuild 0.28.2.

| | hypersig | tailwind | panda | stylex | vanilla-extract | plain |
|---|---|---|---|---|---|---|
| CSS (gzip) | 1.0 KB | 1.6 KB | 1.0 KB | 1.0 KB | 0.6 KB | 0.8 KB |
| HTML without CSS (gzip) | 30.2 KB | 29.1 KB | 26.9 KB | 26.8 KB | 27.6 KB | 27.3 KB |
| Sent in all (gzip) | 31.5 KB | 30.7 KB | 27.8 KB | 27.8 KB | 28.2 KB | 28.1 KB |
| Build of the CSS | None | 268 ms | 574 ms | 182 ms | 74 ms | 3.1 ms |
| Server render | 1.7 ms | 1.5 ms | 1.4 ms | 1.4 ms | 0.8 ms | 0.8 ms |
| First Contentful Paint | 136 ms | 140 ms | 136 ms | 144 ms | 140 ms | 140 ms |
| Style and layout on load | 76 ms | 74 ms | 73 ms | 81 ms | 67 ms | 68 ms |
| Marking 2000 items done | 27 ms | 37 ms | 26 ms | 27 ms | 24 ms | 24 ms |
