//! Why hypersig: the problem it addresses, how the HTML carries the page's
//! behavior, and what that costs.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

/// The body of the page that `Snippet::WhyCounter` renders, with the HTML
/// entities decoded. The snippet asserts the exact HTML.
const COUNTER_HTML: &str = r#"<body data-hs-signals='{"count":0}'>
  <div>
    <button data-hs-on-click='["set","count",["add",["sig","count"],["lit",1]]]'>+1</button>
    <span data-hs-text='["str",["sig","count"]]'>0</span>
    <button data-hs-on-click='["call","POST","/reset",null,null,null]'>Reset</button>
    <a href="/history/0"
       data-hs-attr='[["href",["concat",["lit","/history/"],["uri",["sig","count"]]]]]'>History</a>
  </div>
</body>"#;

pub fn why(site: Site) -> Part {
    parts(
        intro(site)
            .into_iter()
            .chain(halves(site))
            .chain(html(site))
            .chain(hydration(site))
            .chain(typed(site))
            .chain(state(site))
            .chain(costs(site)),
    )
}

fn intro(site: Site) -> [Part; 2] {
    [
        title(Doc::Learn(Learn::Why).title()),
        lead(
            site,
            "hypersig renders the page on the server and writes its behavior into the HTML. Every string the browser acts on comes from a Rust type, so the page needs no browser build, no hydration, and no string typed by hand.",
        ),
    ]
}

fn halves(site: Site) -> [Part; 4] {
    [
        heading("Two halves of one page"),
        para(
            site,
            "A page served by a Rust server has two halves. The server half builds the HTML and answers requests. The browser half reacts to clicks and typing, and the browser runs only JavaScript or wasm. Two designs are common today, and each gives up something.",
        ),
        bullets(
            site,
            &[
                "Compile the browser half from Rust to wasm, as Leptos does. The page and the server share types, so a mismatch between them is a compile error. The browser downloads the wasm and hydrates: it runs the page's components again to attach listeners and to rebuild the state the server already had, and the build needs `cargo-leptos`.",
                "Send HTML and describe the behavior in attributes, as htmx does. Nothing is built for the browser and nothing hydrates. The URLs, the target selectors, and the expressions in the attributes are strings, so a typo in one shows up when someone clicks.",
            ],
        ),
        para(
            site,
            "hypersig takes the shape of the second design and the checks of the first. The page is HTML with attributes, and Rust writes every attribute from a typed value.",
        ),
    ]
}

fn html(site: Site) -> [Part; 6] {
    [
        heading("The HTML carries the behavior"),
        para(
            site,
            "Here is the counter from the [Quick start](learn/), with a link added. `HISTORY` is a page whose path holds the count.",
        ),
        show(Snippet::WhyCounter),
        para(
            site,
            "`Page::render` writes this body, shown here with the HTML entities decoded:",
        ),
        code::block(Lang::Html, None, COUNTER_HTML),
        para(
            site,
            "Each attribute comes from one value in the Rust code. `data-hs-signals` is the `Counter` struct, serialized. `data-hs-on-click` is the statement `count.set(count + 1)` as JSON, and `count` in it is the path `Counter::count()` names. `/reset` is the path of `RESET`, and the `href` comes from `HISTORY`. A misspelled field, a value of the wrong type, or a link with a parameter its route does not take fails to compile before any of this is written.",
        ),
    ]
}

fn hydration(site: Site) -> [Part; 3] {
    [
        heading("No hydration"),
        para(
            site,
            "The page's code is Rust and stays on the server. When the page loads, the runtime reads the state from `data-hs-signals` into a store, adds one listener per event type on `<body>`, and reads the `data-hs-*` attributes. It downloads no component code and runs no component again, because the attributes already hold every expression and statement the browser runs. Like Qwik's resumability, the state and the listeners travel in the HTML.",
        ),
        para(
            site,
            "`Page::render` also evaluates each binding with the initial values, so the HTML already shows `0` and the link already points to `/history/0`. The page reads correctly before the runtime arrives, and the runtime is the same file for every page: about 42 KB, or about 14 KB gzipped, embedded in the crate.",
        ),
    ]
}

fn typed(site: Site) -> [Part; 4] {
    [
        heading("Typed from page to server"),
        para(
            site,
            "The rule that keeps the attributes correct is that the browser never receives a string written by hand. Each kind of string has a type that produces it:",
        ),
        bullets(
            site,
            &[
                "Signal names come from `#[derive(Signals)]`, which adds one typed accessor per field.",
                "Expressions and statements are `Expr` and `Stmt` values, whose operators check the types of both sides.",
                "Action URLs come from `Action<S, A>`, which also fixes the page an action belongs to and the type of its arguments.",
                "Page links come from `Route<P, Q>`, declared in `routes!` with the handler that serves it, so a link carries the right path parameters and query.",
                "Patch targets come from `Fragment::ID`, which the page and the patch read from the same constant.",
                "Element names for DOM calls come from `NodeRef`.",
            ],
        ),
        show(Snippet::WrongType),
    ]
}

fn state(site: Site) -> [Part; 2] {
    [
        heading("The server owns the state"),
        para(
            site,
            "The browser keeps signals for what changes on the page, and the server keeps the data. An action answers with patches over Server-Sent Events: new signal values, or a fragment of HTML to morph into the page. A statement runs in the browser at once, and an action call sent with it reaches the server after, so the page changes before the server answers without extra code. [Talking to the server](learn/server/) covers actions, patches, and streams.",
        ),
    ]
}

fn costs(site: Site) -> [Part; 3] {
    [
        heading("What it costs"),
        bullets(
            site,
            &[
                "The browser runs only what `Expr` and `Stmt` express: arithmetic, comparisons, text, lists, conditions, and calls to the server and the DOM. A computation outside that set runs on the server, or in JavaScript through an [escape hatch](learn/escape-hatches/).",
                "Signal values cross as JSON, so a signal's type implements serde's traits, and an `i64` past 2^53 loses precision in the browser.",
                "Following a link loads the next page from the server. There is no client-side navigation that keeps the current page on screen.",
                "Pages are served through axum, and hypersig is not on crates.io yet.",
            ],
        ),
        para(
            site,
            "Next, the [Quick start](learn/) builds the counter and runs it.",
        ),
    ]
}
