<img src="https://raw.githubusercontent.com/imaimai17468/hypersig/main/assets/hypersig.svg" alt="" width="96" height="96">

# hypersig

[日本語](README.ja.md) | [Documentation](https://imaimai17468.github.io/hypersig/)

**Server-owned state, type-checked strings.**

hypersig is a [hypermedia](https://imaimai17468.github.io/hypersig/learn/why/#hypermedia) web framework on axum in which the page, its behavior in the browser, and the server handlers are all written in Rust. The server holds the state and sends the browser patches. Every string the browser acts on (signal names, expressions, action URLs, links) is produced from a Rust type, so a mistake in one is a compile error.

- `cargo build` builds the whole application, with no Node or wasm step.
- The first HTML already holds the page's state, so the page needs no hydration, and an action answers with only the values that changed ([Why hypersig](https://imaimai17468.github.io/hypersig/learn/why/)).
- A form's rules, CSS, and translations are checked by the compiler as well, and a component set styled after shadcn/ui comes with it ([Forms](https://imaimai17468.github.io/hypersig/learn/forms/), [Styling](https://imaimai17468.github.io/hypersig/style/), [Translations](https://imaimai17468.github.io/hypersig/i18n/), [Components](https://imaimai17468.github.io/hypersig/components/)).

## Quick start

Add the dependencies to `Cargo.toml`:

```toml
[dependencies]
hypersig = "0.8"
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
    action RESET: Action<Counter> = post "/reset" => reset;
}

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
    let app = pages(Router::new()).hypersig_runtime();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
```

Run `cargo run` and open <http://127.0.0.1:3000>. The `+1` button counts in the browser without a request, and the reset button calls `/reset`, whose patch sets `count` back to 0. Misspelling `count`, or adding a string to it, stops `cargo build`. With the CLI (`cargo install --locked hypersig_cli`), `cargo hypersig new my-app` starts a project and `cargo hypersig dev` rebuilds and reloads it after each change. The [documentation](https://imaimai17468.github.io/hypersig/learn/) continues from here.

## Framework comparison

[Why hypersig](https://imaimai17468.github.io/hypersig/learn/why/) compares hypersig with Topcoat and Datastar, which come closest, in more detail.

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar | Leptos | Dioxus |
|---|---|---|---|---|---|---|---|---|---|
| Page written in | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript and Vue templates | HTML attributes | Rust | Rust |
| Server written in | Rust | Rust | Any | TypeScript | TypeScript | TypeScript | Any | Rust | Rust |
| No browser build | Yes | Yes | No (Vite) | No (Node bundler) | No (Vite) | No (Vite) | Yes | No (`cargo leptos`, to Wasm) | No (`dx`, to Wasm) |
| Request types checked at compile time | Yes | Yes | No | Yes | Yes | No | No | Yes | Yes |
| Links checked against routes at compile time | Yes | Handler only (parameters at run time) | No (no router) | Opt-in (`typedRoutes`) | Yes | Experimental (`typedPages`) | No | No (`href` is a string) | Yes (a `Routable` enum value) |
| State kept in the browser | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |

## Benchmarks

The test app is a 2000-item TodoMVC that stores every add, toggle, and delete on the server. Each number is the median of 10 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request, and the fastest and slowest of the 10 runs are in parentheses.

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar | Leptos | Dioxus |
|---|---|---|---|---|---|---|---|---|---|
| JS (gzip) | 19.7 KB | 14.8 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 13.1 KB | 5.5 KB | 15.7 KB |
| Wasm (gzip) | - | - | - | - | - | - | - | 145.1 KB | 441.4 KB |
| First Contentful Paint | 239 ms (231–309) | 221 ms (206–241) | 209 ms (198–266) | 240 ms (229–261) | 237 ms (217–251) | 236 ms (223–263) | 217 ms (204–249) | 232 ms (222–240) | 213 ms (205–244) |
| Total Blocking Time | 3 ms (0–37) | 318 ms (311–351) | 16 ms (11–23) | 14 ms (9–16) | 0 ms (0–0) | 36 ms (29–42) | 20 ms (17–23) | 47 ms (42–57) | 122 ms (114–126) |
| Interaction to Next Paint | 24 ms (24–24) | 24 ms (24–24) | 24 ms (16–24) | 24 ms (16–24) | 24 ms (16–24) | 24 ms (24–24) | 16 ms (16–16) | 24 ms (16–24) | 72 ms (72–80) |

The apps and the harness are in [`bench/compare`](bench/compare), measured at commit 7d031d0. Leptos and Dioxus load most of their code as WebAssembly, which the Wasm row counts. [`bench/js-framework-benchmark`](bench/js-framework-benchmark/results.md) has the results of js-framework-benchmark, and the [`hypersig_style` README](crates/hypersig_style/README.md#benchmarks) compares the styling crates. On the same machine, [`examples/todomvc`](examples/todomvc) (485 lines) builds from an empty target directory in 13.4 s and rebuilds after a one-line change in 1.3 s (`cargo build`, Rust 1.99.0).

## Deploying and security

`cargo hypersig new my-app --host server|cloudflare|vercel` writes a project for a container host, Cloudflare Workers, or a Vercel Function, and `cargo hypersig add <host>` adds one to an existing project ([Deploying](https://imaimai17468.github.io/hypersig/learn/deploying/)). [Security](https://imaimai17468.github.io/hypersig/learn/security/) lists what hypersig checks on each request and what each application checks itself, starting with who may call each action.

## What the compiler does not check

The compiler checks that names, types, and links match. These mistakes still compile, and show up only when the program runs. The [documentation](https://imaimai17468.github.io/hypersig/learn/why/#what-the-compiler-does-not-check) has the same list.

| Mistake | What happens |
|---|---|
| The function `routes!` generates is never passed to the router | Its pages and actions answer 404 |
| A page under a `Base` calls an action of a `routes!` block without it | The call answers 404 |
| One `node_ref!`, `dialog_ref!`, `Id`, or `FileInput` name goes on two elements, or `El::id` is used inside a list template | Rendering the page panics. Two ids from `Id::from_text` alike are not caught, and a reference to that id reaches the first |
| `label_for` or a similar reference names an `Id` no element carries | The control has no label |
| A script outside hypersig finds an `El::foreign` element by its value, and the constant is renamed | The script finds no element. Look it up by `Id` instead |
| A `#[signals(computed)]` field is never passed to `Page::computed` | Rendering the page panics, for a field of a nested struct only where the page reads it |
| A list item is used outside the closure it was passed to | Building or rendering the page panics. An expression built from the item inside the closure and used inside another list at the same depth is not caught, and reads that list's item |
| A signal holds an integer beyond ±2^53 | Rendering the page or building the patch panics |
| A `String` segment of a route, or a part of a `Rest`, is empty, `.`, or `..` | The link answers 404, or the browser resolves the dots and asks for another page |

## What it does not suit

Without the server, the browser runs only the operations hypersig provides and the scripts a page loads. A JavaScript library such as a map or an editor runs inside a custom element, and `cargo hypersig elements` writes the names from its `custom-elements.json` as Rust items ([JavaScript libraries](https://imaimai17468.github.io/hypersig/learn/javascript/)). hypersig is a poor fit for:

- An application whose main work runs inside a JavaScript library, such as a drawing tool or a spreadsheet
- An application that works without a connection to the server
- A page whose forms must work with JavaScript turned off, since a form calls an action only through the runtime

## Development

[`CONTRIBUTING.md`](CONTRIBUTING.md) covers bug reports and pull requests, and [`AGENTS.md`](AGENTS.md) describes the repository layout and the checks a change runs.

## License

MIT
