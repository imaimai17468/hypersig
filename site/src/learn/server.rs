//! The pages on talking to the server, styling, and escape hatches.

use super::snippet::{Snippet, show};
use super::{Program, program};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, subheading, title};

pub fn server(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Server).title()),
        lead(
            site,
            "The browser calls an action, and the server answers with patches: new values for signals, or new HTML for a part of the page.",
        ),
        heading("Actions"),
        para(
            site,
            "An [`Action<S, A>`](api:hypersig/struct.Action.html) is a server endpoint for the page whose signals are `S`, receiving arguments of type `A`. Declare it as a `const` and use the same value on both sides. The page calls it with `call()`, or `call_with(args)` where `args` is an expression evaluated in the browser when the call runs, and the router serves it with `RouterExt::action`. A handler that needs several values takes a tuple, which the page passes as a tuple of expressions.",
        ),
        para(
            site,
            "The handler receives the router state and the arguments, already deserialized, and returns anything that converts into `Patches<S>`. A request whose arguments do not deserialize gets a 400 before the handler runs.",
        ),
        program(Program::Actions),
        subheading("Busy and failed calls"),
        para(
            site,
            "`indicator(flag)` sets a `bool` signal while the request is in flight, which the example uses to disable the button. `error(message)` sets a `String` signal to the error text when the call fails, which is a network error or a response that is not a success, and to `\"\"` when it succeeds.",
        ),
        subheading("Optimistic updates"),
        para(
            site,
            "A statement before the call changes the page at once, and the request follows: `count.set(count + 1).then(LIKE.call())`. The handler's patch then sets the count the server stored. When the server answers with an error, the change stays in the browser. `error` can show the failure, but nothing rolls the value back.",
        ),
        heading("Patches"),
        para(
            site,
            "[`Patches<S>`](api:hypersig/struct.Patches.html) is a batch the runtime applies in order. `sig.patch(value)` sets one signal, `and` adds more to the same event, and `Patches::new().all_signals(&state)` replaces all of them. Only a page-rooted signal has `patch`, so the server addresses fields by name and never by a list position.",
        ),
        show(Snippet::Patches),
        para(
            site,
            "Each patch travels as one Server-Sent Event. `to_event_stream` returns the body for a server that does not use axum.",
        ),
        heading("Fragments"),
        para(
            site,
            "A [`Fragment`](api:hypersig/trait.Fragment.html) is a part of the page the server can render again on its own. It owns its element id, so the id in the first page and the id in a later patch cannot drift apart. `fragment(&value)` renders it in the page, and `Patches::new().element(&value)` sends it again, which the runtime morphs into the element with that id. A patch carries the CSS rules its elements use.",
        ),
        program(Program::Fragments),
        heading("Streams"),
        para(
            site,
            "`Action::get` with `RouterExt::stream` serves a handler that returns a stream of patches. The response stays open until the stream ends or the browser leaves the page. Call it from `on_load` to push changes made elsewhere, such as by other users.",
        ),
        program(Program::Streams),
        note(
            site,
            "These programs need a server, so this site shows them as code. Each is built by `cargo test` from `site/examples/` in the repository.",
        ),
    ])
}

pub fn pages(site: Site) -> Part {
    parts(
        [
            title(Doc::Learn(Learn::Pages).title()),
            lead(
                site,
                "A route is a page the server renders at a path. The router serves it and the links on other pages point to it from the same value, so a parameter of the wrong type or in the wrong number is a compile error.",
            ),
            heading("Routes"),
            para(
                site,
                "A [`Route<P>`](api:hypersig/struct.Route.html) is a path whose parameters are `P`. A parameter is a whole segment written `{name}`, as in axum, and `P` is `()` for a path without one, one [`Segment`](api:hypersig/trait.Segment.html) for one, and a tuple for two to six, in path order. A `Segment` is an integer, a `String`, or an enum whose variants have no fields with `#[derive(Segment)]`, whose segment is the variant's name. Declare it in [`routes!`](api:hypersig/macro.routes.html), on a line that names the handler serving it, so a route without a handler cannot be declared. The macro counts the segments, so a path whose count differs from `P` fails to compile. `routes!` needs the `axum` feature, which is on by default.",
            ),
            show(Snippet::WrongRoute),
            para(
                site,
                "The first line of `routes!` names a function and the router state, and the function registers each route on `GET` with its handler. A handler receives the router state and the parameters, already parsed, and returns the `Page`. A path whose segments do not parse into the parameter types gets a 404, and so does a handler that returns `None`, as `member` does below for an id it does not know.",
            ),
            program(Program::Pages),
            heading("Links"),
            para(
                site,
                "`El::href` takes a [`Link`](api:hypersig/struct.Link.html), which `link()` builds for a route without parameters and `link_with(params)` for one with them. The parameters are expressions of the route's types, as the arguments of an action call are. Literals give a fixed `href`. A parameter that reads a signal or a list item, as on the team page above, is evaluated in the browser and kept up to date. Both sides percent-encode a value as `encodeURIComponent` does, so the first HTML and the runtime write the same URL.",
            ),
            show(Snippet::Links),
            heading("Pages under a page"),
            para(
                site,
                "`in PARENT` after the path declares a route under the page of an earlier line. Its path is the parent's path followed by more segments, and its parameters are the parent's followed by its own, so a link to the child needs the parent's values too. A path that does not continue the parent's, or parameters that do not start with the parent's, fail to compile.",
            ),
            show(Snippet::ChildRoutes),
            para(
                site,
                "Call the function `routes!` writes on the router the application serves. Called on a router passed to axum's `nest`, it serves the pages under a longer path than their links point to, so each of them answers every request with a 500 that names both paths.",
            ),
        ]
        .into_iter()
        .chain([
            heading("Search parameters"),
            para(
                site,
                "A route's second type, `Route<P, Q>`, types its query string. `Q` is a struct with `#[derive(Signals)]`, and a link and the handler then take `(P, Q)`, or `Q` alone for a path without parameters. A field is a string, a number, a `bool`, or an `Option` of one. The query string lists the fields in the order of their names and leaves out a field that is `None`, on the server and in the browser alike. A query string that does not deserialize into `Q` gets a 400.",
            ),
            show(Snippet::SearchParams),
            heading("Redirects and navigation"),
            para(
                site,
                "The server writes a page's URL with `url(params)`, which takes the route's parameter values themselves and encodes them as a link does. A page handler that sends the browser elsewhere returns a [`PageResponse`](api:hypersig/struct.PageResponse.html), built with `PageResponse::redirect(url)` or converted from a `Page`. An action does the same with `Patches::navigate(url)`, which the browser follows once the patches before it are applied.",
            ),
            para(
                site,
                "In the browser, `link.navigate()` is a statement that loads the linked page, for a handler that should leave the page after it runs other statements.",
            ),
            show(Snippet::Navigation),
            heading("What routes leave unchecked"),
            bullets(
                site,
                &[
                    "Nothing checks that the function `routes!` writes is called. A block whose function is never called answers 404 on every page, and rustc reports its handler functions as never used, though not the function itself or a handler written as a closure.",
                    "A parent after `in` is a page itself, so a path prefix with no page of its own cannot be one.",
                    "A link is a plain `<a href>`, so following it loads the next page from the server, and the signals of the page you leave are gone.",
                    "`attr(\"href\", ...)` still writes any URL, for a link to another site or to a `#fragment`.",
                ],
            ),
        ]),
    )
}

pub fn styling(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Styling).title()),
        lead(
            site,
            "Styles are Rust values too. A misspelled property or keyword, or a value of the wrong type, is a compile error.",
        ),
        heading("Atomic styles"),
        para(
            site,
            "[`Style`](api:hypersig/style/struct.Style.html) has one method per standard CSS property, generated from MDN's CSS data. `El::css` adds each declaration as one class named after the declaration, as in the atomic CSS of Panda CSS and Tailwind CSS. The page writes the rules its elements use into `<head>`, and a patch that brings new elements brings their rules.",
        ),
        show(Snippet::Styles),
        bullets(
            site,
            &[
                "`on(PseudoClass::Hover, ...)` and `on(Media::Md, ...)` apply a style under a condition. Conditions cover pseudo-classes and pseudo-elements, media features, container sizes, and the state of a marked ancestor or sibling (`group` and `peer`).",
                "`css_when(style, cond)` adds a style while a signal condition holds, and `css_match(value, cases)` picks one style per value of a signal.",
                "A `Token` is a CSS custom property with a typed value. `Page::theme` sets tokens for the page, for example under `Media::Dark`.",
                "`Global` holds rules for elements by tag with a small reset, `@font-face`, and `@keyframes`. `hypersig::style::pattern` has layouts such as `stack` and `grid`.",
            ],
        ),
        para(
            site,
            "The badge below takes its color from `css_match` over an enum, and the green comes from a token with a dark value.",
        ),
        demo::show(site, Demo::Status),
        heading("Values the types do not check"),
        para(
            site,
            "A value with several parts, such as `margin: 0 auto`, a shadow, or a `calc()`, goes through `raw(...)`, which is passed to the browser unchecked, and so is a value outside a property's range, such as a negative width. A page that loads an existing stylesheet keeps it: `Stylesheet::class(\"name\")` checks at compile time that the stylesheet has a selector for the class.",
        ),
        para(
            site,
            "The [`style`](https://github.com/imaimai17468/hypersig/tree/main/examples/style) example in the repository puts each feature on one page, and the [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) example shows the components of `hypersig_ui`, which are built with it.",
        ),
    ])
}

pub fn escape_hatches(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::EscapeHatches).title()),
        lead(
            site,
            "Most pages need only bindings and statements. These cover focus, timers, and reactions to a change, links outside the router, and serving a page as a static file.",
        ),
        heading("Node refs"),
        para(
            site,
            "A [`NodeRef`](api:hypersig/struct.NodeRef.html) names one element for a DOM call that no binding covers. Declare it as a `const`, attach it with `El::node_ref`, and use its statements: `focus`, `blur`, `select`, `scroll_into_view`, `show_popover`, and `hide_popover`. The reset button below stops the stopwatch and moves focus to the start button.",
        ),
        demo::show(site, Demo::Stopwatch),
        heading("Timers and reactions"),
        bullets(
            site,
            &[
                "`on_interval(period, stmt)` runs a statement while the element is in the document, and `on_timeout(delay, stmt)` runs one once.",
                "`on_load(stmt)` runs once when the element enters the document.",
                "`effect(stmt)` runs when the element enters the document and again whenever a signal the statement reads changes.",
                "`on_signal_change(sig, stmt)` runs whenever the signal, or any field inside it, changes, whatever changed it.",
                "`Expr::now()` reads the browser's clock, and `Expr::random_below(n)` draws a random integer.",
            ],
        ),
        show(Snippet::Effects),
        heading("Keyboard patterns"),
        para(
            site,
            "`roving_focus` moves focus between the items inside an element with the arrow keys, Home, and End, as a tab list or a menu needs. `active_descendant` keeps focus on a text field while the up and down arrows move through the options of a listbox, as a combobox needs. `hypersig_ui` builds its tabs, menus, listbox, and combobox on them.",
        ),
        heading("Links the router does not serve"),
        para(
            site,
            "`El::href` takes only a link built from a [`Route`](learn/pages/). A link to another site, or to a `#fragment` on the same page, goes through `attr(\"href\", url)`, which writes the string as it is and checks nothing.",
        ),
        heading("Static pages"),
        para(
            site,
            "A page that calls no action needs no server. `Page::runtime_src` loads the runtime from another URL, and `RUNTIME_JS` holds its source to write next to the page. This site is built that way: a Rust program renders each page with hypersig and writes the files that GitHub Pages serves.",
        ),
        show(Snippet::StaticPage),
        heading("Where hypersig does not fit"),
        para(
            site,
            "The server holds the state, and the browser runs only the operations hypersig provides. It does not suit these applications:",
        ),
        bullets(
            site,
            &[
                "An application that runs a JavaScript library in the browser, such as a map, a chart, or a rich text editor. These have to run inside the browser, and the server cannot stand in for them.",
                "An application that works without a connection to the server, such as one used offline.",
            ],
        ),
    ])
}
