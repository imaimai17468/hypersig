<img src="https://raw.githubusercontent.com/imaimai17468/hypersig/main/assets/hypersig.svg" alt="" width="96" height="96">

# hypersig

[日本語](README.ja.md) | [Documentation](https://imaimai17468.github.io/hypersig/)

**Server-owned state, type-checked strings.**

hypersig is a [hypermedia](https://imaimai17468.github.io/hypersig/learn/why/#hypermedia) web framework on axum in which the page, its behavior in the browser, and the server handlers are all written in Rust. The server holds the state and sends the browser patches. Every string the browser acts on (signal names, expressions, action URLs, links) is produced from a Rust type, so a mistake in one is a compile error.

- `cargo build` builds the whole application. There is no Node or wasm step.
- A mismatch between the page and the server is a compile error, and so is a link that does not fit a route ([Why hypersig](https://imaimai17468.github.io/hypersig/learn/why/)).
- The first HTML already holds the page's state. An action answers with only the values that changed, and the browser rewrites only what shows them ([Talking to the server](https://imaimai17468.github.io/hypersig/learn/server/)).
- The page needs no hydration. On load, the runtime reads the state the server wrote into the HTML and runs no component again ([Why hypersig](https://imaimai17468.github.io/hypersig/learn/why/)).
- You write a form's rules once, and the browser and the server both check them ([Forms](https://imaimai17468.github.io/hypersig/learn/forms/)).
- CSS properties and values are type-checked too ([Styling](https://imaimai17468.github.io/hypersig/style/)).
- A component set that looks like shadcn/ui, and a translation crate that makes a missing translation a compile error ([Components](https://imaimai17468.github.io/hypersig/components/), [Translations](https://imaimai17468.github.io/hypersig/i18n/)).

## Quick start

Add the dependencies to `Cargo.toml`:

```toml
[dependencies]
hypersig = "0.5"
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

Run `cargo run` and open <http://127.0.0.1:3000>. The `+1` button counts in the browser without a request. The reset button calls `/reset`, and the handler's patch sets `count` back to 0. Misspelling `count`, or adding a string to it, stops `cargo build`. For a faster loop, install the CLI with `cargo install --locked hypersig_cli` and run `cargo hypersig dev`: it rebuilds and restarts the server after each change, and the open tab reloads from the new one. The [documentation](https://imaimai17468.github.io/hypersig/learn/) continues from here.

## Framework comparison

Topcoat and Datastar come closest to hypersig. Both keep signals in the browser and update the page from the server with no browser build, and Topcoat writes the browser's expressions in Rust too.

|  | Topcoat and Datastar | hypersig |
|---|---|---|
| Signal names and URLs (Datastar) | Strings in HTML attributes. A typo shows up when the browser runs it | Written from Rust values. A typo stops `cargo build` |
| Link parameters (Topcoat) | `href!` checks only the handler. A missing or extra path parameter panics when the page renders, and the request answers 500 | `Route` takes its parameters as a typed value. A wrong one is a compile error |
| Adding to a list (Topcoat) | Expressions cannot add an item, so the list changes once the server answers and renders it again | The list is a signal, so the browser shows the change before the server answers |

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar |
|---|---|---|---|---|---|---|---|
| Page written in | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript and Vue templates | HTML attributes |
| Server written in | Rust | Rust | Any | TypeScript | TypeScript | TypeScript | Any |
| No browser build | Yes | Yes | No (Vite) | No (Node bundler) | No (Vite) | No (Vite) | Yes |
| Request types checked at compile time | Yes | Yes | No | Yes | Yes | No | No |
| Links checked against routes at compile time | Yes | Handler only (parameters at run time) | No (no router) | Opt-in (`typedRoutes`) | Yes | Experimental (`typedPages`) | No |
| State kept in the browser | Yes | Yes | Yes | Yes | Yes | Yes | Yes |

## Benchmarks

The test app is a 2000-item TodoMVC that stores every add, toggle, and delete on the server. Each number is the median of 3 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request.

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 19.2 KB | 14.8 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 13.1 KB |
| First Contentful Paint | 186 ms | 165 ms | 196 ms | 198 ms | 193 ms | 177 ms | 182 ms |
| Total Blocking Time | 0 ms | 313 ms | 5 ms | 10 ms | 0 ms | 29 ms | 16 ms |
| Interaction to Next Paint | 24 ms | 24 ms | 24 ms | 24 ms | 24 ms | 16 ms | 16 ms |

The apps and the harness are in [`bench/compare`](bench/compare). The JS row for hypersig is its runtime at commit 72e62b8, where these runs were made, gzipped by the harness. The runtime has grown since, to about 20 KB gzipped. Results of [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark), which compares list operations in the browser alone, are in [`bench/js-framework-benchmark/results.md`](bench/js-framework-benchmark/results.md).

The same page styled with `hypersig::style`, Tailwind CSS, Panda CSS, StyleX, and vanilla-extract is measured in the [`hypersig_style` README](crates/hypersig_style/README.md#benchmarks).

## Deploying

[Deploying](https://imaimai17468.github.io/hypersig/learn/deploying/) goes from a new project to a running application in a few commands, using `cargo hypersig` (install it with `cargo install --locked hypersig_cli`). `cargo hypersig new my-app --host server` creates a project that builds a container image with Docker, for a host such as Fly.io, Cloud Run, or Render. With `--host cloudflare`, the same pages, actions, and streams run on Cloudflare Workers, and the worker hands each request to the axum `Router`. With `--host vercel`, they run as a Vercel Function, and `vercel_runtime`'s `VercelLayer` hands over the requests. The templates in [`crates/hypersig_cli/templates/`](crates/hypersig_cli/templates) are compiled into the command, so every project made with one version gets the same files, including the commands they run at build time. The Workers and Vercel projects also need Node.js, because Cloudflare and Vercel publish their command-line tools (`wrangler` and `vercel`) on npm. For an existing project, `cargo hypersig add <host>` edits `Cargo.toml`, writes the host's files, and prints the change you need to make to the Rust code.

## Security

[Security](https://imaimai17468.github.io/hypersig/learn/security/) lists what hypersig checks: escaping, the attributes and URLs a page may write, the requests an action refuses, and why a request cannot name code to run on the server. It also lists what each application checks itself, starting with who may call each action.

## What it does not suit

Without the server, the browser can run only the operations and statements hypersig provides, plus the scripts a page loads. A JavaScript library such as a map or a rich text editor can run inside a custom element: the page passes it values through attributes and receives typed events from it ([JavaScript libraries](https://imaimai17468.github.io/hypersig/learn/javascript/)). For a library that ships a Custom Elements Manifest (`custom-elements.json`), `cargo hypersig elements` writes its tags, attribute names, and event names as Rust items, so a misspelled one is a compile error. Everything else goes through the server, which makes hypersig a poor fit for these applications:

- An application whose main work runs inside a JavaScript library, such as a drawing tool or a spreadsheet
- An application that works without a connection to the server

## Development

[`CONTRIBUTING.md`](CONTRIBUTING.md) covers bug reports and pull requests, and [`AGENTS.md`](AGENTS.md) describes the repository layout and the checks a change runs.

## License

MIT
