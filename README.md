# hypersig

[日本語](README.ja.md) | [Documentation](https://imaimai17468.github.io/hypersig/)

A web framework on axum where the page, its behavior in the browser, and the server handlers are all written in Rust.

- `cargo build` builds it, with no Node and no wasm.
- A mismatch between the page and the server is a compile error, and so is a link whose page, path parameters, or query do not fit a route.
- Changing the page before the server answers (an optimistic update) takes no extra code.

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

Run `cargo run` and open <http://127.0.0.1:3000>. `routes!` declares the page at `/` with its handler `index`, and the function `pages` it writes serves it. The `+1` button counts in the browser without a request. The reset button calls `/reset`, and the handler's patch sets `count` back to 0. `hypersig_runtime()` serves the browser runtime, and no Node or JavaScript build is involved.

## Pages and links

A page is a `Route`, declared in `routes!` together with the handler that serves it, so a route without a handler cannot be declared. `El::href` links to it from the same value. A parameter is a `{name}` segment, and the route's type gives each one a type, so a link whose parameters do not fit the route, or a path with more or fewer `{name}` segments than the type has values, is a compile error. A path segment that does not parse into its type gets a 404 before the handler runs.

```rust
use hypersig::html::{a, each, li, ul};
use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Member {
    id: i32,
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Team {
    members: Vec<Member>,
}

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<i32> = "/members/{id}" => member;
}

async fn member((): (), id: i32) -> Option<Page<Team>> {
    (id > 0).then(|| Page::new("Member", Team { members: Vec::new() }))
}

let list = ul::<Team>().child(each(Team::members(), |member| {
    li().child(
        a().href(MEMBER.link_with(member.at(Member::id())))
            .bind_text(member.at(Member::name())),
    )
}));
assert!(list.render().contains("/members/"));
```

The second type of `Route<P, Q>` types the query string: `Q` is a struct with `#[derive(Signals, Search)]`, whose fields are each a string, a number, a `bool`, or an `Option` of one, and the links and the handler take `(P, Q)`. Following a link loads the next page from the server. A link to another site still goes through `attr("href", ...)`, unchecked.

## Styling

Styles are Rust values too. `Style` has one method per standard CSS property, generated from MDN's CSS data, and each declaration becomes one class named after the declaration, as in the atomic CSS of Panda CSS and Tailwind CSS. A misspelled property or keyword, or a value of the wrong type such as a color for `width`, is a compile error. The page writes the rules its elements use into `<head>`, and a patch that brings new elements brings their rules, placed so that a breakpoint still overrides the base value.

```rust
use hypersig::html::button;
use hypersig::style::value::{Color, Cursor, rem};
use hypersig::style::{Media, PseudoClass, Style, Theme, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    busy: bool,
}

let accent = Token::new("accent", Color::hex(0xb83f45));
let page = Page::new("Cart", Cart { busy: false })
    .theme(Theme::new().on(Media::Dark, Theme::new().set(&accent, Color::hex(0xff6b6b))))
    .child(
        button()
            .css(
                Style::new()
                    .padding(rem(0.5))
                    .color(accent.var())
                    .cursor(Cursor::Pointer)
                    .on(PseudoClass::Hover, Style::new().color(Color::CRIMSON))
                    .on(Media::Md, Style::new().padding(rem(1))),
            )
            .css_when(Style::new().opacity(0.5), Cart::busy())
            .text("Buy"),
    );
assert!(page.render().contains("hover:color_crimson"));
```

`css_when` switches a style with a signal, and `css_match` picks one per value of a signal, the variants of a Panda CSS recipe. Conditions cover pseudo-classes and pseudo-elements, media features, container sizes, and the state of a marked ancestor or sibling (`group` and `peer`). `hypersig::style` also has design tokens with a page theme, layout patterns such as `stack` and `grid`, rules for elements by tag with a small reset, `@font-face`, and `@keyframes`. The [`style`](examples/style) example puts each of them on one page. The [`todomvc`](examples/todomvc) example is written with them alone, and a Chrome screenshot of it matched, byte for byte, one taken with the stylesheet it had before.

A value with several parts, such as `margin: 0 auto`, a shadow, or a `calc()`, goes through `raw(...)`, which is passed to the browser unchecked, and so is a value outside a property's range, such as a negative width. A page that loads an existing stylesheet keeps it: `Stylesheet::class("name")` checks at compile time that the stylesheet has a selector for the class.

## Components

The `hypersig_ui` crate is a set of components that look like shadcn/ui and take their behavior from the browser's native elements and the WAI-ARIA Authoring Practices: buttons, text fields, checkboxes, switches, radio groups, selects, a slider, a listbox, a combobox, tabs, an accordion, a data table, bar, stacked bar, horizontal bar, line, and area charts, dialogs, a menu button, tooltips, toasts, alerts, badges, cards, and a progress bar. Each returns an `El`, so it takes handlers and children like any other element.

A text field's label, description, and error message are tied to the field by ids derived from the signal it is bound to, and the tabs of a tab list are tied to their panels the same way. A dialog, a menu, a tooltip, or a combobox takes its names from a `const` handle that its parts share, and a name that an id cannot hold is a compile error. The browser opens a dialog with `showModal()` through invoker commands, and puts a menu or a tooltip in the top layer as a `popover` placed beside its button with anchor positioning. Tabs, listbox options, and menu items move with the arrow keys, Home, and End through `El::roving_focus`. A combobox keeps focus in its text field and moves through the options it shows with `El::active_descendant`, which writes the reached option's id to a signal. Toasts sit in an `aria-live` region and stay until dismissed unless a time is set. A data table sorts, filters, pages, and selects its rows in the browser, and its columns are built from fields of the row type, so a column from another struct or a sort key that is neither a number nor a string is a compile error. A server table sends its filter text, page, and sort to an action and shows the page of rows the server answers with, and its sort keys are an enum that the server reads back. A chart draws an `<svg>` from the same kind of list signal, on a scale the browser fits to the values, negative ones included, and a screen reader reads a hidden table of its rows.

The colors follow shadcn/ui's zinc theme. Three values differ, to meet WCAG 2.2 AA: a control's border has 3:1 contrast or more, secondary text keeps 4.5:1 on the muted fill, and focus is a solid outline, which forced-colors mode still draws. The [`ui`](examples/ui) example puts every component on one page, and its end-to-end test drives them with the keyboard and checks their roles and accessible names.

## Translations

The `hypersig_i18n` crate keeps each text with its translations in one value. An application lists its locales in an enum with `#[derive(Segment)]` and implements `Locale` for it, naming a struct with one field per locale, such as `ByLang { en: "Hello", ja: "こんにちは" }`. A text without one of its translations fails to compile, and so does a text once a locale is added to the enum. The locale is the first segment of each page's route, as in `"/{lang}/members/{id}"`, so the link to the same page in another language is the same route with another locale. `Page::lang` writes the locale's tag into `<html lang>`, and `redirect_to_locale` answers `/` with a redirect to the language the browser's `Accept-Language` asks for. The [i18n docs](https://imaimai17468.github.io/hypersig/i18n/) cover it in full.

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
cd e2e && pnpm install && pnpm check && pnpm test   # starts the six examples and drives them in Chrome
```

The browser runtime is TypeScript under `runtime/`, and `pnpm build` writes the prebuilt `crates/hypersig/assets/hypersig.js` (about 41 KB, about 14 KB gzipped) that the crate embeds. `AGENTS.md` describes the repository layout and the checks a change runs. The lint set and coding conventions follow Microsoft's [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/).

## License

MIT
