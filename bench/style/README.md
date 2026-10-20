# Styling TodoMVC-2000 six ways

The page of `bench/compare/hypersig`, 2000 todos, styled with `hypersig::style`, Tailwind CSS, Panda CSS, StyleX, vanilla-extract, and the hand-written `bench/compare/shared/todomvc.css`. Every styling sets the values of that file, and the six pages render to the same pixels. `results/latest.md` holds the latest numbers.

## How it works

Each tool reads its own source in the directory named after it and writes `out/<name>/classes.json`, the class names of each part of the page, and `out/<name>/style.css`. Panda CSS, StyleX, and vanilla-extract are run as their documentation describes for a production build, with Panda's `@pandacss/preset-base` and no preflight. Tailwind loads its theme and utilities without preflight. The StyleX and hand-written CSS are minified with esbuild. Every styling sets the same rules on `*` and `body`.

`src/main.rs` then renders the page once per styling with hypersig. The markup is the same for all of them, and only the `class` attributes and the CSS differ. The `hypersig` page puts its rules into `<style>` elements in the HTML, and every other page links its `style.css`.

## Measurements

- **CSS (gzip)**: the `<style>` elements of the page and its `style.css`, gzipped.
- **HTML without CSS (gzip)**: the page with the text of its `<style>` elements removed, gzipped.
- **Sent in all (gzip)**: the page as sent and its `style.css`, each gzipped.
- **Build of the CSS**: the time the tool took to write `classes.json` and `style.css`. The `hypersig` styles compile with the page, so there is no step.
- **Server render**: the median time of 50 calls to `Page::render` in a release build.
- **First Contentful Paint** and **Style and layout on load**: from Chrome with the CPU slowed 4x, the page served from localhost. The second is the sum of Chrome's `RecalcStyleDuration` and `LayoutDuration` at the `load` event.
- **Marking 2000 items done**: the time to add to every item and title the classes a done item has, and to read the layout once, measured in the page.

Browser values are medians of 5 runs.

## Run

```sh
pnpm install
node measure.mjs   # writes results/latest.json and results/latest.md
```

It needs Node 22.18 or later, which runs the TypeScript sources directly, and the Chrome installed on the machine.
