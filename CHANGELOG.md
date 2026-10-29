# Changelog

The release notes on GitHub list the titles of the merged pull requests. This file groups the changes a user of the crates sees, one section per release, and marks the ones that need a change to existing code under **Breaking**. Every crate of the workspace (`hypersig`, `hypersig_proc`, `hypersig_proc_impl`, `hypersig_style`, `hypersig_i18n`, `hypersig_ui`, and from 0.3.0 `hypersig_cli`) is released at the same version.

## [0.14.0] - 2026-10-29

### Breaking

- `Style::line_height` takes a multiplier as an `f64` and no longer converts from an integer. `line_height(16)` wrote 16 times the font size; write `line_height(2.0)` for a multiplier or `line_height(px(16))` for a length.
- Two `Keyframes` with one name and different steps panic where their styles meet, including in `Page::render` and `Patches::element`. Only the first was written, so the other animation played the wrong steps. Identical keyframes are still written once.
- `Color::hex` panics on a value above `0xffffff`. It dropped the top byte of an eight-digit value; `Color::hex_alpha` writes `#rrggbbaa`.
- `FontFace::weights` panics on a value outside 1 to 1000 or on a range whose minimum is above its maximum, which the browser drops.
- `El::aria_describedby`, `El::aria_activedescendant`, and `El::popover_target` panic when called twice on one element, as the other attributes that name an id do. The browser read only the first.

### Added

- `clamp`, `min`, `max`, and the `+`, `-`, `* f64`, and `/ f64` operators write `calc()` over one quantity type (`Length`, `Percentage`, `Time`, or `Angle`); mixing two is a compile error.
- `Color::hsl`, `Color::oklch`, and `Color::oklab` with typed channels.
- Every generated value enum ends in `Raw`, and `Color` and `Length` convert from `Raw`, for a value the types cannot write yet.
- `ColorScheme::Light`, `Dark`, and `LightDark`, and the other single values MDN's grammar allows alone, such as `Display::ListItem` and `ScrollbarGutter::Stable`. The grid-line properties still refuse an integer.
- `FontFamily::fallback` lists fallback families, also after `FontFace::family()`.
- `Global::write_css` writes a `Global` into a page's stylesheet with the page's keyframes.

### Fixed

- A `Global` rule whose style carries `Style::keyframes` writes its `@keyframes` rule, and a page checks those keyframes with its elements' keyframes.
- `all` is written before the properties it resets, so `accent-color` in the same style is no longer reset by `all: unset`.
- `Length::ZERO` keeps its unit inside `calc()` and equals `px(0)`.
- The docs of `PseudoClass` name the pseudo-classes it leaves out and why, and the docs of `Segment`, `Base`, and `routes!` describe the implementors and the 500 of a nested router as they are.

## [0.13.3] - 2026-10-28

### Added

- `Page::persist` keeps a page signal in the browser's `localStorage`, and the runtime starts the next page that persists the same path from the stored value. A script in `<head>` hides the page until the runtime has rendered a stored value that differs from the initial one, so the first frame is not drawn from the initial value, and shows it after one second if the runtime fails to start.
- `Stmt::view_transition` runs a statement inside `document.startViewTransition`, so the browser animates what it changes on the page. A browser without view transitions runs the statement with no animation.
- `Page::css_when` and `Page::css_match` put a style on `<body>`, so a token a signal picks reaches every element of the page and the background of `<body>` itself.

## [0.13.2] - 2026-10-28

### Fixed

- On macOS, `cargo hypersig dev` builds a project from `cargo hypersig new` with blaze. blaze refused such a project unless its dev profile set `split-debuginfo`, so `dev` built every change with Cargo after blaze failed: a median of 13.7 s from a save to the new page, where it is now 1.1 s.

## [0.13.1] - 2026-10-28

### Fixed

- `cargo hypersig dev` runs `blaze` itself, so it starts with the blaze its installer gives today, which puts no `cargo-blaze` on the path.
- `cargo hypersig dev` builds its first server with Cargo while blaze builds beside it, and each change with Cargo until that build ends. On a machine whose blaze store was empty, the first page of an example came 23.7 s after the start, where it came after 64.5 s.
- `cargo hypersig dev` reads `cargo metadata` once, which took 0.17 s of each rebuild.

## [0.13.0] - 2026-10-28

### Breaking

- `Style::scrollbar_color` takes a `(thumb, track)` pair of colors, and `ScrollbarColor::Color` is gone. One color wrote `scrollbar-color:red`, which the browser drops.
- `El::child` and `El::children` on a `p` panic on an element at which the HTML parser ends the paragraph, such as a `div`, a `ul`, or another `p`, anywhere inside the child outside a `button` or a template. The parser moved such an element after the paragraph, so a condition in it was drawn twice and the first copy stayed.
- A style number that is NaN or infinite panics in the call that sets it, as a length, a duration, and an angle already did. `opacity(Opacity::Number(f64::NAN))` and `flex_grow(f64::INFINITY)` wrote `NaN` or `inf`, which the browser drops.

### Fixed

- A server patch that adds or removes an element before a list or a condition in the same parent keeps the list's template and the items the runtime drew after it, with the text typed into them and their focus.
- A server patch that inserts a template before an input without an id keeps that input.
- `Route::url` writes an `i64` or `u64` search value past 2^53 with its exact digits, so the handler receives it exactly. The docs of `Search` say that `-0.0` arrives as `0.0` and that a NaN or an infinity is left out.
- A `print` rule nested in a breakpoint, a color scheme, or reduced motion writes `print` first, which Chrome needs to apply it.
- `fixed` with many digits writes a value exactly halfway as `toFixed` does.
- The docs of `El::bind` say what a bound field shows for a signal that holds a carriage return.

## [0.12.0] - 2026-10-28

### Breaking

- A `Route` or `Base` path refuses a byte the browser percent-encodes: anything outside ASCII letters, digits, `/`, and `-._~!$&'()*+,;=:@`, such as a space, `?`, `#`, `%`, or a non-ASCII character. A route literal such as `"/café"` compiled before and every link to it got 404, and now fails to compile.
- An `action`, `stream`, or `upload` path in `routes!` refuses `{` and `}` and the bytes a route refuses. `"/a/{"` compiled and panicked when the router was built, and `"/notes/{id}"` matched every path under `/notes/`.
- `#[derive(Segment)]` refuses two variants with one segment. `rename_all` changes the case of ASCII letters only, as serde does, so a variant `Über` is the segment `Über` under `lowercase`.
- `hypersig_ui`'s `Dialog::dialog` and `Dialog::sheet` take a `&DialogText` as their last argument, and `Combobox::render` a `&ComboboxText`, as `pagination` takes a `&PaginationText`. Pass `&Default::default()` for the English words.
- `cargo hypersig new` refuses uppercase letters, a Rust keyword, `test`, the names of the directories under `target/debug`, and, for Cloudflare, `_`.

### Added

- `hypersig_ui::DialogText`, `hypersig_ui::ComboboxText`, and `Toaster::dismiss_label`.

### Fixed

- A page with search parameters no longer answers 400 for a value that is not UTF-8 under a key its struct does not have, such as `?utm=%FF`.
- Two items of an `each_keyed` list with one key no longer leave a stale server-rendered row after hydration: the renderer renames a repeated key as the runtime does.
- A bound style value with an unbalanced `[` is left for the runtime to set, so it no longer takes in the other rules of a page's `<style>` under a CSP nonce.
- `<textarea>` and `<pre>` keep a leading newline of their content.
- A server patch that puts an element before an element with an id or a foreign name keeps that element in the document, so an iframe inside it does not load again and a scrolled box keeps its place. Moves by id use `moveBefore` where the browser has it.
- The warning about a repeated id or name after an element patch reads the page once per id and per attribute, where it read the page once per element of the patch.
- A `Menu` whose first action is disabled focuses the first enabled action when it opens.
- A `Combobox` closes its list when focus has left the field and the pointer leaves the list.
- `cargo hypersig elements` leaves out every attribute `attr!` refuses, such as `poster`, and writes a `detail` field named `yield` as `r#yield`.
- `cargo hypersig dev` finds the binary under the target directory when `build.build-dir` sets the build directory apart.
- Every published crate holds `LICENSE`, and the README on crates.io links to the repository's files.

## [0.11.0] - 2026-10-27

### Breaking

- `hypersig_style::value::url` takes a `&'static str` and is a `const fn`. It panics on an empty URL, a space or a control character, or a scheme other than `http` or `https`, which is a compile error in a `const`, as `Asset::new` does. A URL built at run time no longer compiles. `Url` is `Copy` and has `as_str`, and `Asset` panics with the message of `url`.

### Added

- `hypersig_ui::DateField`, an `<input type="date">` bound to a `CalendarDate` or an `Option<CalendarDate>` signal, with a `CalendarDate` for `min` and `max`.

### Changed

- A root path in a stylesheet's `url(..)` is written under the routes' `Base`, in the page and in the patches an action, a stream, or an upload sends, as an `Asset` path is. A batch built outside the handler, such as a broadcast, goes under the base of the stream that sends it.
- `Page::render` panics where an expression reads the item of one list inside another list at the same depth, whose item the browser read in its place.
- In a debug build, `Page::render` and `Page::simulate` panic where `Page::lang` names the language of an `IntlLocale` and the page formats its plurals and dates only with other ones.

## [0.10.0] - 2026-10-27

### Breaking

- `String` is no longer a `Segment`. A route parameter that holds any text is a `PathSegment`, which refuses an empty value, `.`, and `..`: `PathSegment::new` panics on one, which is a compile error in a `const`, and `PathSegment::parse` returns a `PathSegmentError`. A link that read a `String` signal as a segment reads a `PathSegment` field, and a `String` sort key of `hypersig_ui`'s `Table` becomes a `PathSegment` or an enum with `#[derive(Segment)]`.
- `Rest::new` takes a `&'static str` and panics on an empty rest or a `.` or `..` part. `Rest::parse` reads one built at run time.
- `#[derive(Segment)]` refuses a variant renamed to an empty name, `.`, or `..`.

### Added

- `hypersig::PathSegment`, `hypersig::PathSegmentError`, and `Rest::parse`.

### Changed

- A request whose `PathSegment` or `Rest` decodes to an empty value, `.`, or `..`, such as `/orgs/%2E%2E`, answers 404 before the handler runs, and deserializing either type refuses those values.
- `hypersig::style`'s unit constructors, such as `px`, panic on a NaN or infinite number where the value is built. Before, the stylesheet panicked when it was written.

## [0.9.1] - 2026-10-27

### Changed

- `cargo hypersig dev` builds with [blaze](https://github.com/indexable-inc/blaze) and checks with `cargo check`. After each change it runs both at once and restarts the server from blaze's binary once the check passes, so a compile error still leaves the last server running. It needs blaze installed, and stops with the install line where it is missing. Where blaze fails on code that checks, it builds with `cargo build` as before.

### Fixed

- `attr!` and `tag!` refuse a name in `cargo check` and in rust-analyzer. Before, only `cargo build` reported `attr!("onerror")`.

## [0.9.0] - 2026-10-27

### Breaking

- `Page::script`, `stylesheet`, `icon`, and `runtime_src` take an `Asset`, a URL checked when the constant is built. A root path such as `/map.js` is written under the page's `Base`.
- `Page::lang` and the `hreflang` of `Page::alternate` take a `LanguageTag`, which refuses a tag such as `en_US`. `Locale::tag` returns one.
- `Locale::ALL` is gone. `Locale` requires `Variants`, `hypersig_i18n::locales::<L>()` lists the enum's variants in declaration order, and the first is the default.
- `hypersig_ui::Tabs::new` and `Listbox::new` take one closure over every variant of their enum, in place of `.tab(..)` and `.option(..)`, so a missing variant is a compile error. `Accordion::exclusive` takes an `Id`.
- `UploadedFile::file_name` and `StreamedFile::file_name` return `Option<FileName>`, whose `base_name` gives a name safe to use as a file's last segment.
- Every action answer ends with `hs-end`, and the router refuses a GET action or stream without the `hypersig-call` header the runtime sends (403). `Action::request` and `StreamAction::request` add the header.
- These now panic while the page is built: two elements carrying one id from `Id::with_key`; a `Page::computed` signal named twice, reading itself, or reading one declared after it; a form rule that reads `Expr::now` or `random_below`; a list item read outside any list; and a nested computed field the page reads without a `Page::computed`.
- `#[derive(Search)]` writes `impl Search` directly, so the hidden `SearchFields` trait is gone.
- `hypersig_ui::Chart<S, T, R>` is now `Chart<S, T>`. Its constructors still take the rows of a plain field or of a computed one, and code that named the third parameter drops it.

### Added

- `Call::timeout(Duration)` fails a call whose answer takes longer, and `Call::exclusive()` skips a call while another to the same action is in flight.
- `hypersig::Asset`, `hypersig::LanguageTag`, `hypersig::FileName`, and `Page::render_under(base)` for a static site under a prefix.
- `hypersig_ui` components: `Popover`, `toggle`, `icon_toggle`, `ToggleGroup`, `MultiToggleGroup`, `Collapsible`, `Avatar`, `kbd`, and `spinner`.
- `Sig::read()` on a page-rooted signal returns a `Sig<S, T, Read>`, a read-only view with the same path. The view reads like the signal it came from, because `Read` implements `FromPage`, and it has no `set`, list statement, `bind`, or `patch`. `Page::computed` refuses it.
- `hypersig::serve(router, port)` serves an axum `Router`. It listens on the address in `ADDR` where that is set, otherwise on every interface at the port in `PORT`, otherwise on `127.0.0.1:port`. On Ctrl+C or SIGTERM it waits for the requests in progress, for at most 5 seconds or until a second signal. It needs the `axum` feature and is not built for `wasm32-unknown-unknown`.
- `Page::simulate` returns a `hypersig::testing::Simulation`, which clicks, types, and submits on a page in a test without a browser. It runs the statements of the event on the server's evaluator, finds elements by their `NodeRef`, `DialogRef`, `FileInput`, or `Id`, and reads their text, attributes, and visibility. Calls, links, and element statements such as `focus` come back in an `Effects`, typed by the `Action` or `Route` they came from. It panics on a statement that reads a value only the browser has, such as `Expr::now`.

### Changed

- The runtime fails a call (status 0) when its answer closes before `hs-end`, was redirected, or is not an event stream, and the error signal takes the body only when it is `text/plain`.
- The older answer of a POST or upload no longer overwrites the answer of a newer call to the same action. The element that sent a POST or upload carries `aria-busy="true"` while it is in flight.
- Reconnect waits are spread at random and end early when the browser comes back online or the page becomes visible. A navigation to a URL that keeps the page, such as a download, no longer stops the patches that follow.
- An element patch keeps an input without an id when a sibling is inserted before it, restores focus and the selection, and keeps `<details>` and `<dialog>` open. An answer that repeats text the visitor typed into a focused field leaves what they typed since.
- The server keeps its `frame-ancestors` policy when a handler sends its own `Content-Security-Policy`, answers a handler's panic with 500, sends a fixed sentence for each refusal (the parser's message stays in `source`), and parses action arguments without building a `serde_json::Value` first.
- Pages carry `Cache-Control: private, no-cache`, hypersig's responses carry `X-Content-Type-Options: nosniff`, and streams carry `X-Accel-Buffering: no`.
- Route segments and query values take every integer `url()` writes, past 2^53 included.
- Errors from `routes!`, `attr!`, and `tag!` point at the line that is wrong, without the standard library's panic frames.
- `hypersig_ui::DonutChart::new` takes the rows of a computed field as well as a plain one.
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
