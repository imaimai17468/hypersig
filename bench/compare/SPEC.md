# TodoMVC-2000

The same TodoMVC written in hypersig, Topcoat, Next.js, React, TanStack Start, Nuxt, Datastar, Leptos, and Dioxus, measured in one harness. This file is the contract every implementation follows.

## Data

- The server seeds 2000 todos at startup: id `i` from 1 to 2000, title `todo i`, done when `i % 3 == 0`. That leaves 1334 active.
- The server owns the list. A reload shows the list as the server last stored it.
- `POST /bench-reset` replaces the list with the seed and answers `204`. The harness calls it before every measured page load.

## Behavior

- **Add**: typing in `input.new-todo` and pressing Enter appends a todo with the trimmed text and clears the input. Empty text adds nothing.
- **Toggle**: clicking an item's `input.toggle` flips its done state.
- **Delete**: clicking an item's `button.destroy` removes it.
- **Count**: `.todo-count strong` shows the number of active todos.
- **Filter**: `.filters a` with the texts `All`, `Active`, and `Completed` show all, active, or completed items. A hidden item is either absent from the DOM or has `display: none`.

Every add, toggle, and delete reaches the server, which stores the new list. An implementation may update the page before the server answers where that is how its framework works. Filtering changes no data and may stay in the browser.

## Markup

The harness and the parity test find elements by these selectors only:

| Selector | Element |
|---|---|
| `input.new-todo` | The new todo input |
| `ul.todo-list > li` | One item, with class `completed` when done |
| `li input.toggle` | The item's checkbox |
| `li label` | The item's title |
| `li button.destroy` | The item's delete button |
| `.todo-count strong` | The active count |
| `.filters a` | The filter links |

## Implementations

Each app is written the way an experienced user of that framework would write it for a 2000-item list, following the framework's documented patterns. The comment at the top of each app names the pattern it follows, so a reader can check it. All of them run as production builds, and every script they load is served by the same local server.

| App | Port | Version | Pattern |
|---|---|---|---|
| `hypersig/` | 4100 | this repository | The list is a signal updated in the browser. Each add, toggle, and delete also calls an action with that change's typed arguments |
| `datastar/` | 4102 | Datastar 1.0.4, datastar SDK 0.4.1, axum | The server answers each action with server-sent events that patch the changed item and the count |
| `next/` | 4103 | Next.js 16.4.1, React 19.3.1 | A server component renders the list into a client component that holds it in state, with memoized items. Each change calls a server action |
| `topcoat/` | 4104 | Topcoat 0.10.0 | The list is a shard rendered on the server. Each add, toggle, and delete awaits a procedure with that change's typed arguments, then changes a signal the shard reads, so the server renders the list again |
| `react/` | 4105 | React 19.3.1, Vite 8.3.4 | A single-page app. The HTML is empty, the app fetches the list from a JSON API, holds it in state with memoized items, and sends each change to the API |
| `tanstack-start/` | 4106 | TanStack Start 1.169.2, TanStack Query 5.104.2 | Server rendering with a route loader that fills the query cache. Each change is a `useMutation` that updates the cache before the server function answers |
| `nuxt/` | 4107 | Nuxt 4.6.1, Vue 3.5.44 | Server rendering with `useFetch`, the list in a reactive ref with a component per item, and a server API route per change |
| `leptos/` | 4108 | Leptos 0.8.22, leptos_axum 0.8.10, cargo-leptos 0.3.11 | Server rendering and hydration of a `Resource` inside `<Suspense>`. The list is a signal of items with a signal each, rendered by a keyed `<For>`. Each change dispatches a `ServerAction` for a `#[server]` function without awaiting it |
| `dioxus/` | 4109 | Dioxus 0.7.10 fullstack, dioxus-cli 0.7.10 | Server rendering and hydration of `use_server_future`. The list is a `Store` with a component per item. Each change spawns a call to a `#[server]` function without awaiting it |

## Measurements

`harness/measure.mjs` reports the metrics Lighthouse and Web Vitals define, under Lighthouse's mobile CPU slowdown (4x) with 50 ms added to every request. Bandwidth is not capped, because the Rust servers here send uncompressed responses while Next.js compresses its own, and a bandwidth cap would measure that setting instead of the framework.

- **JS (gzip)**: every script the page loads, gzipped by the harness.
- **Wasm (gzip)**: every WebAssembly file the page loads, gzipped by the harness. Only the Leptos and Dioxus apps load one.
- **FCP** and **TBT** (First Contentful Paint, Total Blocking Time): from Lighthouse 13.5.0 with the settings above. Largest Contentful Paint is left out, because in every app it equals First Contentful Paint.
- **INP** (Interaction to Next Paint): from the Event Timing API, over one page session of 15 checkbox clicks and 15 additions with the Enter key after 3 warm-up clicks. With fewer than 50 interactions, INP is the slowest one. Chrome reports only interactions of 16 ms or more, so a faster page shows as "< 16 ms".

Each value is the median of 3 runs. `results/latest.json` also records, for each app, the median time from a checkbox click until the DOM shows the item done and the new count, which shows whether the app waits for the server.

`harness/parity.mjs` checks the behavior above on every app before measuring.
