# Working on this project

This is a web application built with [hypersig](https://imaimai17468.github.io/hypersig/) {{hypersig-version}}, a server-driven hypermedia library for Rust. The server owns the state, renders HTML, and sends patches over Server-Sent Events, and a small runtime in the browser applies them. Every string the browser acts on (signal names, action URLs, page links, patch targets) is produced from a Rust type, so a mistake in one is a compile error. Keep it that way: write the typed form, and let the compiler find the mistake.

## Check each change

- `cargo check` after each edit. A hypersig mistake is a compile error that names the type it expected, so read the whole message before changing anything.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` before you report the work done.
- `cargo hypersig dev` runs the server and restarts it after each change, and the open page reloads itself. A Rust test runs no JavaScript, so a binding or a statement that changes the page in the browser is checked only by opening the page.

## Where to read

- [llms.txt](https://imaimai17468.github.io/hypersig/llms.txt) lists every page of the guide with one line on what it covers. Fetch the page you need from there.
- The guide follows the main branch of the repository, which can be ahead of the release this project uses. Before you use an API the guide names, confirm it in the source of the version in `Cargo.lock`, under `~/.cargo/registry/src/*/hypersig-<version>/`, or on `https://docs.rs/hypersig/<version>`.
- [`examples/`](https://github.com/imaimai17468/hypersig/tree/main/examples) holds complete applications: `todo` (a list every tab sees, which a stream re-renders as a `Fragment` after each change on a `broadcast` channel), `database` (the same list kept in SQLite), `showcase` (list patches by key), `signup` (a form), `auth` (sign-in with `tower-sessions`), `postgres` and `redis` (more than one process), `ui` (the components).

## Rust tools and guidelines

- **rust-analyzer** gives you the compiler's errors, the types, and the definitions after each edit, without a `cargo check`. Claude Code reaches it through the `rust-analyzer-lsp` plugin, which needs the `rust-analyzer` binary on `PATH`. Where `which rust-analyzer` finds nothing, ask the developer to run `rustup component add rust-analyzer` and `/plugin install rust-analyzer-lsp@claude-plugins-official`, since installing tools on their machine is theirs to decide.
- **Clippy's pedantic lints** are on in `Cargo.toml`. Fix what a warning points at. Where a lint does not fit one place, write `#[expect(clippy::name, reason = "...")]` there, which warns again once the code no longer needs it, and turn a lint off for the whole project in `[lints.clippy]` only with the developer's agreement.
- **[Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/agents/all.txt)** is Microsoft's guide to idiomatic Rust, written as one file for agents (about 136 KB, so search it rather than reading it whole). Each guideline has a heading such as `## Use the proper type family (M-STRONG-TYPES)`. Most of them are for libraries. For an application, read M-APP-ERROR on errors, M-PANIC-ON-BUG and M-PANIC-IS-STOP on panics, and M-STRONG-TYPES on types.
- **Crate documentation:** where a documentation server such as Context7 is connected, ask it for the version in `Cargo.lock`, rather than writing a crate's API from memory.

## How hypersig code is written

- **A page's state is one struct** with `#[derive(Signals, Serialize, Deserialize)]`. The derive gives each field a function, such as `Counter::count()`, that returns a typed `Sig`. Refer to a signal only through that function.
- **A page is a route** declared in `hypersig::routes!`, which pairs each `Route` constant with its handler. Link to a page with `ROUTE.link()`, `ROUTE.link_with(args)` for a route with parameters, or `ROUTE.url(args)` on the server, never with a path written by hand.
- **An action is a constant**, `Action::post("/path")` or `Action::get("/path")`, and that constant is the only place its path is written. The page calls it with `ACTION.call()` or `ACTION.call_with(signal)`, and the router serves it with `.action(ACTION, handler)`. A stream is a `StreamAction`, served with `.stream(..)`, that the page opens with `on_load(STREAM.call().reconnect())`.
- **A handler is `async fn(state, args)` or `async fn(state, extractor, args)`.** It takes one extractor at most, and a tuple of extractors where it needs more. It answers with `Patches` (`Counter::count().patch(value).into()`), and fails with `Result<_, E>` where `E: IntoResponse`. The page shows the failure's text in the signal `.error(signal)` names.
- **A list** is `each_keyed`. A handler changes the items it names by key with `patch_append`, `patch_upsert`, or `patch_remove`, in place of sending the whole list again.
- **A stream that sends a list and then each change** reads the list and subscribes to the changes under one lock, or answers a change with `patch_upsert`, so that a change made between the two is neither lost nor shown twice.
- **A form** is a `Form<T>` field with `impl Validate for T`. The browser checks the rules as the visitor types, and the handler checks the same rules with `Form::validate` before it trusts the values.
- **The components** of `hypersig_ui` (`TextField`, `submit_button`, `alert`, `card`), a separate crate released at the same version, carry their own attributes and ARIA roles, so prefer them to `.attr(..)` with a name written by hand.
- **No JavaScript of your own and no `data-hs-*` attribute.** A statement such as `count.set(count + 1)` runs in the browser and is written in Rust. A JavaScript library goes in a custom element, as the guide's JavaScript page shows.
- **Signed-in visitors:** keep the session with `tower-sessions` 0.14, whose session stores still depend on its 0.14 core, and read the member with your own extractor. A page answers a signed-out visitor with a redirect, and an action with a 401, because the runtime's `fetch` follows a redirect and cannot apply the HTML it gets. `RequestKind::of(parts)` tells the two apart. The page tells the 401 from other failures with `.status(signal)` on the call, compared with `StatusCode::UNAUTHORIZED`, not by matching the error text.

## Tests

Call a handler as a function and read what it returned: `Page::value_of(Sig)` for a signal's first value, `Patches::value_of(Sig)` for the value a batch sets, `Form::value()` for the values of a form it holds, and `Patches::navigates_to()` for a navigation. The function `#[derive(Signals)]` writes for a field has the field's visibility. A `mod tests` inside the module that declares the struct reaches a private field's function, and a test anywhere else needs the field to be `pub(crate)`. Send a request through the router with `tower::ServiceExt::oneshot` to test what the router adds, such as an extractor's refusal: `ACTION.request(&args)` and `STREAM.request(&args)` build the request the runtime sends, with its headers, so a test names the action, not its path.

## This project's host

{{host-notes}}
