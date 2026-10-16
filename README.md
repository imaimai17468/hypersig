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

A 2000-item TodoMVC that stores every add, toggle, and delete on the server. Medians of 3 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request.

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 13.4 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| First Contentful Paint | 179 ms | 190 ms | 184 ms | 198 ms | 188 ms | 172 ms | 235 ms |
| Total Blocking Time | 0 ms | 27 ms | 2 ms | 5 ms | 0 ms | 25 ms | 233 ms |
| Interaction to Next Paint | 24 ms | 16 ms | 24 ms | 16 ms | 24 ms | 16 ms | 16 ms |

The apps and the harness are in [`bench/compare`](bench/compare). Results of [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark), which compares list operations in the browser alone, are in [`bench/js-framework-benchmark/results.md`](bench/js-framework-benchmark/results.md).

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

In hypersig the server holds the state. What the browser can do on its own is the set of operations and statements hypersig provides, such as arithmetic on `i32` and `f64`, string and list operations, and focus. Anything else goes through a server action, so it responds a network round trip later.

It does not suit these applications:

- An application that runs a JavaScript library in the browser, such as a map, a chart, or a rich text editor. These have to run inside the browser, and the server cannot stand in for them.
- An application that works without a connection to the server, such as one used offline.

## Development

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p hypersig --bench render
cd runtime && pnpm install && pnpm check && pnpm test && pnpm build
cd e2e && pnpm install && pnpm check && pnpm test   # starts the four examples and drives them in Chrome
```

The browser runtime is TypeScript under `runtime/`, and `pnpm build` writes the prebuilt `crates/hypersig/assets/hypersig.js` (about 38 KB, about 13 KB gzipped) that the crate embeds. `AGENTS.md` describes the repository layout and the checks a change runs. The lint set and coding conventions follow Microsoft's [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/).

## License

MIT
