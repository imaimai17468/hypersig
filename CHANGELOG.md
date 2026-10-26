# Changelog

The release notes on GitHub list the titles of the merged pull requests. This file groups the changes a user of the crates sees, one section per release, and marks the ones that need a change to existing code under **Breaking**. Every crate of the workspace (`hypersig`, `hypersig_proc`, `hypersig_proc_impl`, `hypersig_style`, `hypersig_i18n`, `hypersig_ui`, and from 0.3.0 `hypersig_cli`) is released at the same version.

## [Unreleased]

### Breaking

- `hypersig_ui::Chart<S, T, R>` is now `Chart<S, T>`. Its constructors still take the rows of a plain field or of a computed one, and code that named the third parameter drops it.

### Added

- `Sig::read()` on a page-rooted signal returns a `Sig<S, T, Read>`, a read-only view with the same path. The view reads like the signal it came from, because `Read` implements `FromPage`, and it has no `set`, list statement, `bind`, or `patch`. `Page::computed` refuses it.
- `hypersig::serve(router, port)` serves an axum `Router`. It listens on the address in `ADDR` where that is set, otherwise on every interface at the port in `PORT`, otherwise on `127.0.0.1:port`. On Ctrl+C or SIGTERM it waits for the requests in progress, for at most 5 seconds or until a second signal. It needs the `axum` feature and is not built for `wasm32-unknown-unknown`.

### Changed

- Writing a signal that cannot be written reports ``a signal marked `Computed` cannot be written``, with `hypersig::Read` in place of `Computed` for a view, where it used to report `a computed signal cannot be written`.
- `cargo hypersig new <path>` without `--host` writes the `server` template, where it used to stop with a list of hosts.
- The server template depends on `hypersig_ui`, uses its `button`, and starts with `hypersig::serve`. The examples start with `hypersig::serve` too.
- `cargo hypersig add server` leaves `Cargo.toml` as it is, because the step it prints calls `hypersig::serve` and needs no tokio `signal` feature.

## [0.8.2] - 2026-10-26

No public signature changed, but several mistakes that used to misbehave quietly now panic while the page is built.

### Changed

- These now panic at build time: a second `show`, `bind_text`, `id`, `node_ref`, `foreign`, or binding on one element; a static attribute set twice, through `attr` or a method such as `role`; a binding on an element the runtime does not bind, such as `bind` on a `<div>`; `to_fixed` above 100 digits; `on_interval` of 0; and an `on_interval`, `on_timeout`, or `on_debounced` longer than 2^31 - 1 ms.
- The server trims the same characters as JavaScript's `trim` and orders strings by UTF-16 code unit, so `Form::validate` and the first HTML agree with the browser.
- The runtime cancels the browser's own submission of every form, including one without `on_submit`.
- `Id::with_key` escapes ASCII whitespace and `%` in a key, so `a b` and `a_b` give different ids. An id built from a key holding whitespace or `%` changes.

### Fixed

- A query value of `NaN` or `inf`, or one that is not UTF-8, gets 400, and a query key that is not UTF-8 is ignored. A path segment that is not UTF-8 gets the router's 404 page in place of axum's bare 400.
- `hypersig_ui::pagination` shows one page for a count of 0, and table paging no longer overflows on a large page number.

## [0.8.1] - 2026-10-26

No public Rust API changed. The fixes are in the browser runtime.

### Fixed

- A stream opened by an element's statement closes once the element leaves the page, so a replaced element no longer leaves two streams applying the same events.
- An event whose data does not parse fails alone and no longer ends the stream or leaves the connection open.
- A stream that answers 2xx and closes before its first event is reopened with backoff, where it used to be reopened every second.
- A computed signal that throws keeps its value and logs the error, and the rest of the statement runs.
- Text that is not JSON leaves a JSON-bound control's signal as it was.
- Intervals, timeouts, custom event listeners, and on-visible observers follow the attributes a patch writes.
- Moving keyed list items keeps the focus and caret of a field inside a moved item.

## [0.8.0] - 2026-10-26

### Breaking

- `Page::framed_by(origin)` is now `Page::framing(Framing::Origins(vec![origin]))`, and `Page::framed_anywhere()` is now `Page::framing(Framing::Anywhere)`. One call sets the whole policy, where `framed_by` after `framed_anywhere` used to be ignored.
- `attr::SRCSET` and `attr!("srcset")` are replaced by `El::srcset([(url, Candidate::Width(480)), ...])`, `attr::POSTER` by `El::poster(url)`, and `attr::STYLE` by `El::css` or `El::bind_style`.
- `attr::IMAGESRCSET`, `PING`, `BACKGROUND`, `MANIFEST`, `CODEBASE`, `ARCHIVE`, `CLASSID`, and `LONGDESC` are removed, and `attr!` refuses those names.
- `el(tag!("link"))` fails to compile. `Page::stylesheet` and `Page::icon` write a page's `<link>` elements.

### Changed

- The runtime applies the rule of `SafeUrl` when a binding writes `href`, `src`, or `poster`, and removes the attribute where the URL breaks it. A URL the browser builds, such as one in a `CustomEvent<SafeUrl>`, was written unchecked before.
- `El::src` on an `<iframe>`, `<embed>`, or `<frame>` panics when the URL reads a signal.
- An integer outside -2^53 to 2^53 gets 400 in an action's arguments or a query value and 404 in an `i64` or `u64` route segment. Such a value used to reach the signals and make the render panic.

### Fixed

- `Page::render` panics when a field marked `#[signals(computed)]` is named by no `Page::computed` call. `#[derive(Signals)]` writes `Signals::COMPUTED` for this check.
- A list item used inside another list at the same depth is caught when the page is built.

## [0.7.1] - 2026-10-26

### Changed

- `hypersig_ui::Chart` is `Chart<S, T, R = Root>` and accepts a computed list, so rows filtered with `Page::computed` can go into a chart. Code that writes `Chart<S, T>` still compiles.
- Using an `Item`, or a signal reached from it, after its closure has returned panics while the page is built. It used to compile and fail in the browser with `no list item at depth N`.

## [0.7.0] - 2026-10-26

### Breaking

- A field marked `#[signals(computed)]` is a `Sig<S, T, Computed>`, and `set`, `toggle`, the list statements, `bind`, and `patch` on it fail to compile. `Page::computed` accepts only a marked field. The wire format is unchanged.

## [0.6.0] - 2026-10-26

### Breaking

- Actions, streams, and uploads are declared as lines of `routes!`, each with its handler: `action ADD: Action<Todos, String> = post "/todos" => add;`, `stream UPDATES: StreamAction<Todos> = "/updates" => updates;`, and `upload AVATAR: Upload<Profile> = "/avatar" max_bytes 1024 * 1024 => avatar;`. `Action::post`, `Action::get`, `StreamAction::new`, `Upload::post`, `Upload::max_bytes`, and each `under` are gone from the public API, and `.action(ADD, add)` on the router is hidden. An action without a handler no longer compiles.
- `NodeRef` and `DialogRef` are declared with `hypersig::node_ref!(const FIELD: NodeRef<Search>);` and `dialog_ref!`, which build the name from the module path and the constant's name. `NodeRef::new` and `DialogRef::new` are gone. `hypersig_ui`'s `Tooltip::new`, `Menu::new`, `Dialog::new`, and `Combobox::new` take a `NodeRef` or `DialogRef` in place of a name.
- `Expr` methods such as `gt`, `equals`, `to_text`, and `concat` take `&self`, so `remaining.clone().gt(0)` becomes `remaining.gt(0)`. Code that passes a method as a function, such as `Expr::and`, needs a closure.

### Added

- After `cargo hypersig dev` restarts the server, the reloaded tab gets back the signals the visitor changed, where the route and the struct's fields match. `#[derive(Signals)]` writes `Signals::SHAPE` for this. The development code moved to `/_hypersig/dev.js`, which only a debug build serves.
- `DialogRef::id()` gives the element id of a dialog.

## [0.5.0] - 2026-10-25

### Breaking

- `El::attr`, `bind_attr`, and `attr_when` take an `Attr` in place of a `&'static str`: a constant from `html::attr`, or `hypersig::attr!("data-lat")` for a name the list lacks. The role is set with `El::role(Role)`. A refused attribute is a compile error, where it used to panic while rendering.
- `El::id` takes an `Id`: `Id::new` for an id written in code, `Id::with_key` for repeated elements, and `Id::from_text` for an id made from text at run time. Attributes that point at an id, such as `for` and `aria-describedby`, are set with methods that take the same `Id` (`label_for`, `aria_describedby`, `aria_controls`, and others), and their `attr` constants are removed. `Fragment::ID` is an `Id`.
- `el` takes a `Tag` from `hypersig::tag!("sl-rating")`, which refuses names holding whitespace, quotes, or `=`, and refuses `script`, `style`, and `base`. `el("img onerror=...")` used to slip an `on*` attribute past the checks.

### Changed

- `El::active_descendant` binds `aria-activedescendant` to the same signal.
- The code `cargo hypersig elements` writes uses `Attr` constants and `tag!`.

## [0.4.2] - 2026-10-25

### Added

- `Patches::items_of(list)` and `Patches::removed_keys_of(list, key)` let a test read the list patches a handler returns.

### Changed

- Clearer compile errors: a child element from another page's signals, a stream handler written as `async fn`, and an action handler whose return type is not accepted each get a message that names the mistake. `El::child`, `El::children`, and `Page::child` take `impl IntoNode<S>`, which every `Into<Node<S>>` implements.
- `#[derive(Signals)]` and `#[derive(Segment)]` still write the field functions after refusing a `#[serde(...)]` attribute, so no unrelated E0599 follows.
- `cargo hypersig new` without `--host` lists the hosts and an example command. `cargo hypersig dev` prints the files it watches and a line at each start, and reloads faster after a save (a median of 1.88 s down to 1.70 s on the server template).
- The server and Vercel templates carry tests of the counter, and the template's `AGENTS.md` shows how to write `#[expect]` for the two pedantic lints hypersig's shapes trigger.

### Fixed

- Toasts added by the browser and by the server no longer share a key.
- `cargo hypersig dev` no longer misses a file saved during a build.

## [0.4.1] - 2026-10-25

### Added

- `Expr::text("...")` builds a string constant, in place of `Expr::lit(&"...".to_owned())`.

### Fixed

- `Route::link_with` has no failure path left and no longer falls back to a default value.

## [0.4.0] - 2026-10-24

### Added

- `cargo hypersig` (crate `hypersig_cli`) carries the deploy templates. `cargo hypersig new <path> --host server|cloudflare|vercel` creates a project, and `cargo hypersig add <host>` moves an existing one, editing `Cargo.toml` and printing the Rust change to make by hand.
- `cargo hypersig dev` rebuilds and restarts the server after each change, and a page served by a debug build reloads itself once the new server is up.
- `cargo hypersig elements <custom-elements.json>` writes a Rust module of a library's custom elements, attributes, and events.
- `cargo hypersig new` writes `AGENTS.md` and `CLAUDE.md`, and the templates turn on Clippy's pedantic group.
- `Form::value()` reads the values a form holds. `Action::request` and `StreamAction::request` build the request the runtime sends, for tests through the router.
- `Call::status` and `StreamCall::status` put the HTTP status of a failed answer in an `i32` signal.
- Examples that keep data in SQLite, PostgreSQL, Redis, and another service's API, and an `auth` example built on `tower-sessions`. Learn pages cover storing data, calling an API, signing in, and testing.

### Changed

- An integer outside -2^53 to 2^53 panics before it is written to JSON, in a page, a patch, or a literal, where the browser used to round it. Keep larger ids and amounts in a `String`.

## [0.3.0] - 2026-10-22

### Added

- A Vercel template, in which `vercel_runtime`'s `VercelLayer` hands each request to the axum `Router`.
- `Page::framed_by(origin)` and `Page::framed_anywhere()` let other sites show a page in a frame.

### Changed

- A page answers with `X-Frame-Options: SAMEORIGIN` and `Content-Security-Policy: frame-ancestors 'self'` by default, so another site can no longer frame it and click its buttons. A page meant to be embedded needs one of the two methods above.
- `Origin` refuses characters that cannot appear in a host and port.
- The server sends `hs-end` when a stream's handler finishes. A call with `reconnect()` treats a stream that closes before `hs-end`, such as one Vercel ends at its time limit, as broken off and opens it again.

### Fixed

- The runtime no longer applies a last SSE event that was cut off midway.

## [0.2.1] - 2026-10-22

### Fixed

- `Form::validate` refuses `inf`, `-inf`, and `NaN`. They became `null` in JSON, so a rule with a fallback value, such as `price.some().unwrap_or(0.0).le(1000.0)`, let them through.

## [0.2.0] - 2026-10-22

### Breaking

- `El::href` and `El::src` take a `Link` or a `SafeUrl`. `attr` and `bind_attr` panic on attribute names that can run script, such as `href`, `src`, `on*`, `srcdoc`, `action`, `formaction`, and `http-equiv`.
- A GET action answers 403 to a request without the `Accept` header the runtime sends.

### Changed

- `El::id` takes `impl Into<Cow<'static, str>>`, and two `String` ids that are the same no longer stop the render.
- A fragment rendered for another part answers 500 in place of panicking.
- `Form::validate` treats a rule the server cannot evaluate as failed.
- The server template's `Dockerfile` names no package, builds with `--locked`, and runs as `nobody`. The Workers template needs no `package.json`.

## [0.1.1] - 2026-10-19

### Added

- A statement can handle a custom event a script dispatches, with its `detail` typed.
- `El::foreign` keeps an element a script owns out of the server's patches.
- `Page::script` loads a script that defines custom elements.

## [0.1.0] - 2026-10-19

The first release, with six crates:

- `hypersig`: page state as a struct with `#[derive(Signals)]` and typed `Sig` paths, browser expressions and statements as `Expr` and `Stmt`, the HTML builder, server patches over Server-Sent Events, typed routes, the axum integration, and the browser runtime, served with Brotli where the browser accepts it.
- `hypersig_proc` and `hypersig_proc_impl`: the derives.
- `hypersig_style`: typed atomic CSS generated from MDN's data.
- `hypersig_i18n`: typed translations and `Accept-Language` negotiation.
- `hypersig_ui`: components styled after shadcn/ui.
