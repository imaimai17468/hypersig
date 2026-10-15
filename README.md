# hypersig

[日本語](README.ja.md)

A web framework that runs on axum. The page, its behavior in the browser, and the server handlers live in one Rust crate that `cargo build` builds on its own, with no Node and no wasm.

The page's state is one Rust struct, the signal struct, and both the expressions in the page and the server handlers read and write values through its fields. Renaming a field or changing its type makes every place that uses it, in the page or in a handler, a compile error.

The server embeds the signal struct's values as JSON in the first HTML, and the runtime in the browser loads them and keeps them. When a click or an input changes a value, the browser at once rewrites only the parts of the page that use it, the same reactive updates Solid and Vue make. Saving the new value to the server sends the request after that rewrite, so the page updates without waiting for the server's answer and without any optimistic-update code.

## Compared with Leptos, Next.js, React, TanStack Start, Nuxt, and htmx

`bench/compare/` builds the same 2000-item TodoMVC in each of them and checks that they behave the same.

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| You write | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript and Vue templates | HTML attributes, any server language |
| Build step for the browser | None | wasm (cargo-leptos) | Vite | Node bundler | Vite | Vite | None |
| First HTML has the data | Yes | Yes | No, fetched after load | Yes | Yes | Yes | Yes |
| A click updates the page before the server answers | Yes | Yes | Yes | Yes | Yes | Yes | No |
| The browser gets server state as | The signal struct | Server functions | A JSON API | Server actions and props | Server functions into a query cache | API routes | HTML fragments |

Three things set hypersig apart from that list:

- **No JavaScript toolchain and no wasm.** The whole app is one Rust crate built by `cargo build`. The browser runtime is prebuilt and embedded in the crate.
- **One struct is the page's state on both sides.** The page's signals are a Rust struct that the browser keeps a copy of, and server handlers answer with patches to it. An action's arguments are typed where the page calls it and where the handler receives them. Renaming a field breaks every expression, call, and patch that names it at compile time, so nothing falls out of step unnoticed.
- **The strings the browser runs never appear in your source.** Signal names, expressions, action URLs, and patch targets are Rust values, so a misspelled signal or a number passed where a `bool` belongs is a compile error. The repository keeps 19 such mistakes under `crates/hypersig/tests/ui/fail/`.

## Benchmarks

### js-framework-benchmark

[js-framework-benchmark](https://github.com/krausest/js-framework-benchmark), keyed, with hypersig's entry in `bench/js-framework-benchmark/`. Headless Chrome 155 on an Apple M4 Max, medians of 10 runs, with the benchmark's own CPU slowdowns.

| | hypersig | Leptos | React | Vue | Solid | vanillajs |
|---|---|---|---|---|---|---|
| create rows (ms) | 30.4 | 25.3 | 23.0 | 24.5 | 21.6 | 21.0 |
| replace all rows (ms) | 35.1 | 29.1 | 29.1 | 26.5 | 23.7 | 22.1 |
| partial update (ms) | 20.1 | 11.6 | 14.6 | 12.7 | 10.8 | 10.1 |
| select row (ms) | 8.7 | 6.3 | 6.5 | 4.0 | 5.2 | 3.2 |
| swap rows (ms) | 87.8 | 13.0 | 90.7 | 13.5 | 13.4 | 12.0 |
| remove row (ms) | 52.9 | 12.9 | 14.7 | 13.6 | 12.3 | 10.7 |
| create many rows (ms) | 338.4 | 265.2 | 388.3 | 247.6 | 232.7 | 208.7 |
| append rows to large table (ms) | 34.1 | 27.9 | 28.3 | 26.0 | 24.3 | 23.1 |
| clear rows (ms) | 25.1 | 14.7 | 16.7 | 12.1 | 11.0 | 10.1 |
| memory after create/clear ×5 (MB) | 5.11 | 4.58 | 1.93 | 1.12 | 0.77 | 0.64 |
| size, compressed (KB) | 7.1 | 48.8 | 51.4 | 23.3 | 4.5 | 2.5 |
| first paint (ms) | 42 | 279.9 | 281.9 | 113.9 | 116.5 | 67.7 |

### TodoMVC with a server

The same 2000-item TodoMVC in each framework, in `bench/compare/`. FCP and TBT from Lighthouse, INP from the Event Timing API over 30 interactions. CPU slowed 4x, 50 ms added to every request, medians of 3 runs.

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS (gzip) | 6.5 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| FCP | 178 ms | 196 ms | 178 ms | 191 ms | 179 ms | 167 ms | 230 ms |
| TBT | 71 ms | 28 ms | 4 ms | 8 ms | 0 ms | 26 ms | 248 ms |
| INP | 16 ms | 24 ms | 16 ms | 24 ms | 24 ms | 24 ms | 16 ms |

## Updating after a change

Toggling a todo and saving it to the server, with TanStack Query's documented optimistic update:

```tsx
const queryClient = useQueryClient();
const toggle = useMutation({
  mutationFn: (change: { id: number; done: boolean }) => setTodoDone({ data: change }),
  onMutate: async ({ id, done }) => {
    await queryClient.cancelQueries({ queryKey: ["todos"] });
    const previous = queryClient.getQueryData<Todo[]>(["todos"]);
    queryClient.setQueryData<Todo[]>(["todos"], (list) =>
      list?.map((todo) => (todo.id === id ? { ...todo, done } : todo)),
    );
    return { previous };
  },
  onError: (_error, _change, context) => queryClient.setQueryData(["todos"], context?.previous),
});

<input type="checkbox" checked={todo.done}
  onChange={(event) => toggle.mutate({ id: todo.id, done: event.target.checked })} />
```

The same in hypersig:

```rust
use hypersig::html::{El, each_keyed, input, li, ul};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    todos: Vec<Todo>,
}

const SET_DONE: Action<Todos, (i32, bool)> = Action::post("/done");

fn list() -> El<Todos> {
    ul().child(each_keyed(Todos::todos(), |todo| todo.at(Todo::id()), |todo| {
        let done = todo.at(Todo::done());
        li().child(
            input()
                .attr("type", "checkbox")
                .bind(done)
                .on_change(SET_DONE.call_with((todo.at(Todo::id()), done))),
        )
    }))
}

async fn set_done((): (), (id, done): (i32, bool)) -> Patches<Todos> {
    store(id, done);
    Patches::new()
}

fn store(_id: i32, _done: bool) {}
```

The checkbox writes `done` in the browser, and only the bindings that read that item's `done` run again. `on_change` then sends the id and the new value to `set_done`, typed as `(i32, bool)` on both ends. There is no query key to choose, no cache to snapshot, and no list to refetch.

The cost is on failure. When the server answers with an error, the change stays in the browser: `error(message)` can show the failure, but nothing rolls the value back.

## Getting started

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

## Limits

- Handlers and expressions can use only the operations the library provides. Arbitrary JavaScript, such as `localStorage` or a call into another library, cannot be written.
- A component's state needs a place prepared for it in the page's signal struct. To add components at run time, make them items of a `Vec`.
- Arithmetic is limited to `i32` and `f64`, and division to `f64`. A type whose results JavaScript numbers cannot reproduce cannot appear in an expression.
- `equals` compares the JSON form. It agrees with a derived `PartialEq` and can disagree with a hand-written one.
- `Page::render` is the only place the server evaluates expressions. A fragment sent with `Patches::element` carries no signal values, so the browser fills in its bindings.
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

The browser runtime is TypeScript under `runtime/`, and `pnpm build` writes the prebuilt `crates/hypersig/assets/hypersig.js` (about 14 KB, about 5 KB gzipped) that the crate embeds. `AGENTS.md` describes the repository layout and the checks a change runs. The lint set and coding conventions follow Microsoft's [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/).

## License

MIT
