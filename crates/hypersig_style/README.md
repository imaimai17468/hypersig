# hypersig_style

[日本語](README.ja.md) | [Documentation](https://imaimai17468.github.io/hypersig/style/)

Typed atomic CSS for [hypersig](https://github.com/imaimai17468/hypersig). Use it through the `hypersig` crate, as `hypersig::style`.

- `Style` has one method per standard CSS property, generated from MDN's CSS data, so a misspelled property or keyword is a compile error.
- Each declaration becomes one class named after it, and the page writes each rule it uses once into `<head>`. There is no build step besides `cargo build`.
- Hover, breakpoints, group and peer states, tokens, themes, and keyframes are Rust values too ([Styling](https://imaimai17468.github.io/hypersig/style/)).

## Benchmarks

The 2000-item TodoMVC page of [hypersig's framework benchmark](https://github.com/imaimai17468/hypersig/tree/main/bench/compare), styled six ways and rendered by hypersig with the same markup, so only the class names and the CSS differ. Measured in Chrome 155 on an Apple M4 Max with the CPU slowed 4x. Browser values are medians of 5 runs, and the server render is the median of 50.

| | hypersig | Tailwind CSS | Panda CSS | StyleX | vanilla-extract | Hand-written CSS |
|---|---|---|---|---|---|---|
| CSS (gzip) | 1.0 KB | 1.6 KB | 1.0 KB | 1.0 KB | 0.6 KB | 0.8 KB |
| HTML without CSS (gzip) | 30.2 KB | 29.1 KB | 26.9 KB | 26.8 KB | 27.6 KB | 27.3 KB |
| Sent in all (gzip) | 31.5 KB | 30.7 KB | 27.8 KB | 27.8 KB | 28.2 KB | 28.1 KB |
| Build of the CSS | None | 268 ms | 574 ms | 182 ms | 74 ms | 3.1 ms |
| Server render | 1.7 ms | 1.5 ms | 1.4 ms | 1.4 ms | 0.8 ms | 0.8 ms |
| First Contentful Paint | 136 ms | 140 ms | 136 ms | 144 ms | 140 ms | 140 ms |
| Style and layout on load | 76 ms | 74 ms | 73 ms | 81 ms | 67 ms | 68 ms |
| Marking 2000 items done | 27 ms | 37 ms | 26 ms | 27 ms | 24 ms | 24 ms |

| Where hypersig falls behind | Why |
|---|---|
| HTML size | Each class name is made from its declaration alone: the conditions, the property's full name, and the value, as in `font-size_24px`. A value with a space or another character outside letters, digits, `.`, `#`, `%`, and `-` becomes a 16-digit hash. A server patch renders new elements without knowing the page's classes, so one declaration has to get the same name every time it is rendered |
| Server render | `Page::render` writes those names into every item. The HTML before gzip is 1.21 MB for hypersig and 0.88 MB for Panda CSS, and the render times grow in the same order |

The pages and the harness are in [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style).
