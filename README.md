# hypersig

[日本語](README.ja.md)

A web framework on axum where the page, its behavior in the browser, and the server handlers are all written in Rust.

- `cargo build` builds it, with no Node and no wasm.
- A mismatch between the page and the server is a compile error.
- Changing the page before the server answers (an optimistic update) takes no extra code.

## Framework comparison

Adding a page to a Rust server leaves two choices today. Leptos type-checks what passes between the page and the server, and needs wasm and its build. htmx needs no build, but the URLs and expressions written in its attributes are not type-checked.

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| Page written in | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript and Vue templates | HTML attributes |
| Server written in | Rust | Rust | Any | TypeScript | TypeScript | TypeScript | Any |
| No browser build | Yes | No (wasm and cargo-leptos) | No (Vite) | No (Node bundler) | No (Vite) | No (Vite) | Yes |
| Request types checked at compile time | Yes | Yes | No | Yes | Yes | No | No |
| State kept in the browser | Yes | Yes | Yes | Yes | Yes | Yes | No |

## Benchmarks

### js-framework-benchmark

[js-framework-benchmark](https://github.com/krausest/js-framework-benchmark), keyed. Medians of 10 runs in headless Chrome 155 on an Apple M4 Max, with the benchmark's own CPU slowdowns.

| | hypersig | Leptos | React | Vue | Solid | vanillajs |
|---|---|---|---|---|---|---|
| create rows (ms) | 24.7 | 25.1 | 23.2 | 24.3 | 21.9 | 20.8 |
| replace all rows (ms) | 27.8 | 29.1 | 29.0 | 27.0 | 23.6 | 21.8 |
| partial update (ms) | 15.1 | 11.9 | 14.5 | 12.7 | 11.2 | 9.8 |
| select row (ms) | 3.6 | 5.2 | 6.3 | 4.2 | 7.6 | 3.1 |
| swap rows (ms) | 13.2 | 12.6 | 90.2 | 13.6 | 12.9 | 11.6 |
| remove row (ms) | 14.1 | 13.1 | 14.6 | 13.2 | 12.1 | 10.4 |
| create many rows (ms) | 264.8 | 261.8 | 388.2 | 254.9 | 229.4 | 210.6 |
| append rows to large table (ms) | 29.2 | 28.0 | 27.8 | 26.1 | 24.4 | 23.2 |
| clear rows (ms) | 13.1 | 14.7 | 16.5 | 11.6 | 11.8 | 9.8 |
| memory after create/clear ×5 (MB) | 1.50 | 4.58 | 1.98 | 1.19 | 0.77 | 0.64 |
| size, compressed (KB) | 12.7 | 48.8 | 51.4 | 23.3 | 4.5 | 2.5 |
| first paint (ms) | 66.1 | 250.9 | 248.4 | 98.9 | 55.2 | 45.1 |

### Full-stack TodoMVC

A 2000-item TodoMVC that stores every add, toggle, and delete on the server. Medians of 3 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request.

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 13.0 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 185 ms | 196 ms | 192 ms | 194 ms | 188 ms | 184 ms | 236 ms |
| TBT | 0 ms | 24 ms | 0 ms | 4 ms | 0 ms | 20 ms | 229 ms |
| INP | 24 ms | 16 ms | 24 ms | 24 ms | 16 ms | 16 ms | 16 ms |

## State management and rendering

The page's state is one Rust struct, the signal struct. Expressions in the page and server handlers can read and write values only through its fields, so renaming a field and forgetting an expression that used it, passing a number where a condition needs a `bool`, or calling another page's action is a compile error before anything runs in the browser.

What the browser runs on load and on an update, compared with other frameworks. An effect is the code that reads signal values and rewrites the DOM. hypersig has one for each display attribute such as `data-hs-text` or `data-hs-show`.

| | hypersig | React | Vue | Solid | Qwik |
|---|---|---|---|---|---|
| App code sent to the browser | None (expressions travel as data in HTML attributes) | Every component | Every component | Every component | Only the handlers used, when they are used |
| What the browser runs on load | Every effect, once each | The whole component tree | The whole component tree | Each component once, to build its reactive dependencies | Nothing (state and handler locations are in the HTML) |
| What reruns on an update | Only the effects that read the value | The component holding the state and its descendants | The component's render function, diffed through a virtual DOM | Only the effects that read the value | Only the effects that read the value |

The arguments are typed as `(i32, bool)` both in the page code that calls the action and in the handler that receives them. The query key, the cache snapshot, and the refetch that a TanStack Query optimistic update needs are not written.

The cost is on failure. When the server answers with an error, the change stays in the browser: `error(message)` can show the failure, but nothing rolls the value back.

## The unit of fetching

In RSC and Next.js, data fetching is written with `await` inside server components, so each component starts its fetch separately and the results appear in arrival order, one Suspense boundary at a time. Depending on where the boundaries sit, the page fills in piece by piece (popcorn UI), and code in which a child component's fetch cannot start until its parent's `await` finishes puts the fetches in series and delays the page. After a change, `revalidatePath` or `router.refresh` renders the route's server components again.

In Remix and TanStack Start, each route's loader runs in parallel before the page is shown, so fetches rarely run in series and the page rarely fills in piece by piece. The loaders run again after a change, though: Remix reruns the loaders of every route on screen after an action, TanStack Router reruns them on `router.invalidate()`, and TanStack Query reruns the queries you invalidate, so toggling one checkbox fetches the page's or the list's data again.

In hypersig, the first data is already in the page's HTML as the signal struct's JSON, so the browser fetches nothing to show the page. After a change, the action's handler returns only the values of the signals that changed, as in `Counter::count().patch(0)`, and the browser rewrites only what shows those values, so toggling one checkbox fetches neither the page nor the list again.

## Quick start

Add the dependencies to `Cargo.toml`. hypersig is not on crates.io yet, so it comes from Git.

```toml
[dependencies]
hypersig = { git = "https://github.com/imaimai17468/hypersig" }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }
```

Write `src/main.rs`:

```rust,no_run
use axum::{Router, response::Html, routing::get};
use hypersig::html::{button, div, span};
use hypersig::{Action, Page, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const RESET: Action<Counter> = Action::post("/reset");

async fn index() -> Html<String> {
    let count = Counter::count();
    let page = Page::new("Counter", Counter { count: 0 }).child(
        div()
            .child(button().on_click(count.set(count + 1)).text("+1"))
            .child(span().bind_text(count.to_text()))
            .child(button().on_click(RESET.call()).text("Reset on the server")),
    );
    Html(page.render())
}

async fn reset((): (), (): ()) -> Patches<Counter> {
    Counter::count().patch(0).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .hypersig_runtime()
        .action(RESET, reset);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
```

Run `cargo run` and open <http://127.0.0.1:3000>. The `+1` button counts in the browser without a request. The reset button calls `/reset`, and the handler's patch sets `count` back to 0. `hypersig_runtime()` serves the browser runtime, and no Node or JavaScript build is involved.

## Limitations

- Handlers and expressions can use only the operations the library provides. Arbitrary JavaScript, such as `localStorage` or a call into another library, cannot be written.
- A component's state needs a place prepared for it in the page's signal struct. To add components at run time, make them items of a `Vec`.
- Arithmetic is limited to `i32` and `f64`, and division to `f64`. A type whose results JavaScript numbers cannot reproduce cannot appear in an expression.
- `equals` compares the JSON form. It agrees with a derived `PartialEq` and can disagree with a hand-written one.
- `Page::render` is the only place the server evaluates expressions. A fragment sent with `Patches::element` carries no signal values, so its effects fill in what it shows once it reaches the browser.
- The server and the browser can disagree on rounding an exact half in `to_fixed`, on formatting numbers below `1e-6` or at or above `1e21`, and on ordering characters outside the Basic Multilingual Plane. After loading, the browser's values replace the server's.
- Replacing a fragment that contains an `each` with a server patch rebuilds the list items inside it. Their values live in signals, so their contents are kept.
- Signals cannot use attributes such as `#[serde(rename)]`. The derive rejects them.

## Development

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p hypersig --bench render
cd runtime && pnpm install && pnpm check && pnpm test && pnpm build
cd e2e && pnpm install && pnpm check && pnpm test   # starts the four examples and drives them in Chrome
```

The browser runtime is TypeScript under `runtime/`, and `pnpm build` writes the prebuilt `crates/hypersig/assets/hypersig.js` (about 37 KB, about 13 KB gzipped) that the crate embeds. `AGENTS.md` describes the repository layout and the checks a change runs. The lint set and coding conventions follow Microsoft's [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/).

## License

MIT
