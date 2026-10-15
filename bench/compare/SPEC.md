# TodoMVC-2000

The same TodoMVC written in hypersig, Datastar, htmx, Next.js, and Leptos, measured in one harness. This file is the contract every implementation follows.

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
| `hypersig/` | 4100 | this repository | The list is a signal updated in the browser. Each change calls an action with its arguments |
| `datastar/` | 4101 | Datastar 1.0.4, `datastar` crate 0.4.1, axum | The server answers each action with element patches for the changed item and the count |
| `htmx/` | 4102 | htmx 2.0.11, axum | The server answers each request with the changed item and an out-of-band count |
| `next/` | 4103 | Next.js 16.4.1, React 19.3.1 | A server component renders the list into a client component that holds it in state, with memoized items. Each change calls a server action |
| `leptos/` | 4104 | Leptos 0.8.22, leptos_axum 0.8.10 | Server rendering with hydration, a signal per item, and a server function per change |

## Measurements

`harness/measure.mjs` drives Chrome through Playwright and measures inside the page, so its own round trips do not count.

- **Load**: bytes transferred for HTML and for scripts, First Contentful Paint, and the total length of long tasks until the `load` event.
- **Interaction latency**: two times from the input event (captured on `window` before the app sees it). The first ends when the DOM shows the result, and the second at the first frame painted after that, which is what the user sees and what Chrome's Interaction to Next Paint measures. The second is rounded up to the display's frame interval, so the first shows the framework's own work more finely. For a toggle, the result is the item's `completed` class and the count. For an add, the new item and the count. For a delete, the item gone and the count. For typing, the input's value.
- **Bytes per interaction**: the bytes sent and received over the network during one toggle.

Each interaction runs at two network conditions: localhost, and 50 ms of added latency per request through Chrome DevTools Protocol network emulation. Each value is the median of 15 runs on different items, after 3 warm-up runs.

`harness/parity.mjs` checks the behavior above on every app before measuring.
