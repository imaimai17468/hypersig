# hypersig

[日本語](README.ja.md) | [Documentation](https://imaimai17468.github.io/hypersig/)

A web framework on axum where the page, its behavior in the browser, and the server handlers are all written in Rust. Every string the browser acts on (signal names, expressions, action URLs, links) is produced from a Rust type, so a mistake in one is a compile error.

- `cargo build` builds it, with no Node and no wasm.
- A mismatch between the page and the server is a compile error, and so is a link that does not fit a route ([Why hypersig](https://imaimai17468.github.io/hypersig/learn/why/)).
- The first HTML already holds the page's state. An action answers with only the values that changed, and the browser rewrites only what shows them ([Talking to the server](https://imaimai17468.github.io/hypersig/learn/server/)).
- Changing the page before the server answers (an optimistic update) takes no extra code.
- Form rules are written once, and the browser and the server check them with the same rules ([Forms](https://imaimai17468.github.io/hypersig/learn/forms/)).
- CSS properties and values are type-checked too ([Styling](https://imaimai17468.github.io/hypersig/style/)).
- A component set that looks like shadcn/ui, and a translation crate that makes a missing translation a compile error ([Components](https://imaimai17468.github.io/hypersig/components/), [Translations](https://imaimai17468.github.io/hypersig/i18n/)).

## Framework comparison

Adding a page to a Rust server leaves two choices today. Leptos type-checks what passes between the page and the server, and needs wasm and its build. htmx needs no build, but the URLs and expressions written in its attributes are not type-checked.

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| Page written in | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript and Vue templates | HTML attributes |
| Server written in | Rust | Rust | Any | TypeScript | TypeScript | TypeScript | Any |
| No browser build | Yes | No (wasm and cargo-leptos) | No (Vite) | No (Node bundler) | No (Vite) | No (Vite) | Yes |
| Request types checked at compile time | Yes | Yes | No | Yes | Yes | No | No |
| Links checked against routes at compile time | Yes | No (`href` takes a string) | No (no router) | Opt-in (`typedRoutes`) | Yes | Experimental (`typedPages`) | No |
| State kept in the browser | Yes | Yes | Yes | Yes | Yes | Yes | No |

## Benchmarks

A 2000-item TodoMVC that stores every add, toggle, and delete on the server. Medians of 3 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request.

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 19.2 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| First Contentful Paint | 186 ms | 203 ms | 196 ms | 198 ms | 193 ms | 177 ms | 236 ms |
| Total Blocking Time | 0 ms | 26 ms | 5 ms | 10 ms | 0 ms | 29 ms | 268 ms |
| Interaction to Next Paint | 24 ms | 24 ms | 24 ms | 24 ms | 24 ms | 16 ms | 16 ms |

The apps and the harness are in [`bench/compare`](bench/compare). Results of [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark), which compares list operations in the browser alone, are in [`bench/js-framework-benchmark/results.md`](bench/js-framework-benchmark/results.md).

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
use axum::Router;
use hypersig::html::{button, div, span};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

const RESET: Action<Counter> = Action::post("/reset");

async fn index((): (), (): ()) -> Page<Counter> {
    let count = Counter::count();
    Page::new("Counter", Counter { count: 0 }).child(
        div()
            .child(button().on_click(count.set(count + 1)).text("+1"))
            .child(span().bind_text(count.to_text()))
            .child(button().on_click(RESET.call()).text("Reset on the server")),
    )
}

async fn reset((): (), (): ()) -> Patches<Counter> {
    Counter::count().patch(0).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(RESET, reset);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
```

Run `cargo run` and open <http://127.0.0.1:3000>. The `+1` button counts in the browser without a request. The reset button calls `/reset`, and the handler's patch sets `count` back to 0. Misspelling `count`, or adding a string to it, stops `cargo build`. The [documentation](https://imaimai17468.github.io/hypersig/learn/) continues from here.

## What it does not suit

The browser does on its own only the operations and statements hypersig provides. Anything else goes through the server, so it does not suit these applications:

- An application that runs a JavaScript library in the browser, such as a map or a rich text editor
- An application that works without a connection to the server

## Development

[`AGENTS.md`](AGENTS.md) describes the repository layout and the checks a change runs.

## License

MIT
