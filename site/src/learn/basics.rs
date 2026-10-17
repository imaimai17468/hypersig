//! The quick start and the page on describing the page.

use super::snippet::{Snippet, show};
use super::{Program, program};
use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, title};

pub fn quick_start(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::QuickStart).title()),
        lead(
            site,
            "Build a counter that counts in the browser and resets on the server, in one Rust file.",
        ),
        heading("Add the dependencies"),
        para(
            site,
            "hypersig is not on crates.io yet, so it comes from Git. Add it to `Cargo.toml` with axum, serde, and tokio:",
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = { git = "https://github.com/imaimai17468/hypersig" }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }"#,
        ),
        heading("Write the server"),
        para(
            site,
            "Write `src/main.rs`. It holds the page, the handler that answers the reset button, and the router that serves both.",
        ),
        program(Program::QuickStart),
        heading("Run it"),
        code::block(Lang::Shell, None, "cargo run"),
        para(
            site,
            "Open `http://127.0.0.1:3000`. The `+1` button counts in the browser without a request. The reset button calls `/reset`, and the handler's patch sets `count` back to 0. `hypersig_runtime()` serves the browser runtime from the crate, so no Node or JavaScript build is involved.",
        ),
        heading("What each part does"),
        bullets(
            site,
            &[
                "`#[derive(Signals)]` on `Counter` adds `Counter::count()`, a `Sig<Counter, i32>` that names the field. A misspelled field is a compile error.",
                "`count.set(count + 1)` is a statement. `on_click` writes it into a `data-hs-on-click` attribute as JSON, and the runtime runs it on each click.",
                "`span().bind_text(count.to_text())` keeps the text equal to the count. `Page::render` evaluates it with the initial values, so the HTML already shows `0`.",
                "`INDEX` is a `Route`, the page at `/`. `routes!` declares it together with its handler `index`, the function `pages` it writes serves it, and a link to it is `INDEX.link()`. [Pages and links](learn/pages/) covers routes with parameters.",
                "`RESET` is an `Action<Counter>`. The page calls it with `RESET.call()` and the router serves it with `.action(RESET, reset)`, so both sides share its path, its method, its argument type, and the page it belongs to.",
                "The handler returns `Counter::count().patch(0)`. The server sends it as a Server-Sent Event, and the runtime writes the value into the signal, which rewrites the text.",
            ],
        ),
        heading("Try it here"),
        para(
            site,
            "The counter below runs the browser half of the program on this page. It uses buttons from `hypersig_ui`, the component crate in the same repository.",
        ),
        demo::show(site, Demo::Counter),
        note(
            site,
            "This site is a set of static files on GitHub Pages, so its demos run on signals alone and none of them calls an action.",
        ),
        para(
            site,
            "Next, [Describing the page](learn/describing-the-page/) shows how the HTML is built.",
        ),
    ])
}

pub fn describing_the_page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DescribingThePage).title()),
        lead(
            site,
            "A page is a tree of `El` values built with Rust functions and rendered to HTML on the server.",
        ),
        heading("Elements"),
        para(
            site,
            "The [`html`](api:hypersig/html/index.html) module has one function per common tag, such as `div()`, `p()`, and `button()`, and `el(\"tag\")` for any other. Each returns an `El<S>`, where `S` is the page's signal struct, and its methods return the element, so a tree is one expression.",
        ),
        show(Snippet::Elements),
        para(
            site,
            "`text` appends static text, and `child` appends an element or a string. `href` points a link at a page from a `Route`, which [Pages and links](learn/pages/) covers. `attr` sets a plain attribute. `id` and `class` have their own methods, and `attr` panics on them and on names starting with `data-hs-`, which only the typed methods write.",
        ),
        heading("The page"),
        para(
            site,
            "[`Page`](api:hypersig/struct.Page.html) wraps the tree in a full document. `Page::new` takes the title and the initial values of the signals, `child` appends to `<body>`, and `render` returns the HTML a handler sends. The document loads the runtime from `/_hypersig/runtime.js`, which `RouterExt::hypersig_runtime` serves.",
        ),
        heading("Values that follow the signals"),
        para(
            site,
            "An element can show a value that changes in the browser. Each binding takes an expression over the signals and is rendered with the initial values first, so the HTML shows what the runtime would compute.",
        ),
        show(Snippet::Bindings),
        bullets(
            site,
            &[
                "`bind_text` replaces the element's text.",
                "`bind_attr` keeps an attribute equal to a string expression, and `attr_when` adds a boolean attribute such as `disabled` while a condition holds.",
                "`show` hides the element while a condition is false. `when(cond, el)` removes it instead, so its `on_load` runs again and its inputs start empty each time it returns.",
                "`class_when` and `css_when` add a class or a style while a condition holds. [Styling](learn/styling/) covers them.",
            ],
        ),
        para(
            site,
            "The demo binds a text field to `name` and shows one of two paragraphs depending on whether it is empty.",
        ),
        demo::show(site, Demo::Greeting),
        heading("Components"),
        para(
            site,
            "A component is a Rust function that returns an `El<S>`. Make it generic over `S` and take the signals it reads as arguments, and it works on any page.",
        ),
        show(Snippet::Component),
        para(
            site,
            "`hypersig_ui` is a set of such functions styled after shadcn/ui. The demos on this site use its buttons and text fields.",
        ),
    ])
}
