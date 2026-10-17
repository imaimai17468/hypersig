//! The pages on signals, expressions and statements, and lists.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::prose::{Part, heading, lead, note, para, parts, subheading, table_of, title};

pub fn signals(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Signals).title(site)),
        lead(
            site,
            "The page's state is one Rust struct, and a signal is a typed reference to one of its fields.",
        ),
        heading(site, "The signal struct"),
        para(
            site,
            "Derive `Signals` next to serde's `Serialize` and `Deserialize`. The derive adds one function per field, named after the field, that returns a [`Sig`](api:hypersig/struct.Sig.html). `Sig<Counter, i32>` reads: an `i32` inside the signal struct `Counter`.",
        ),
        show(Snippet::SignalPaths),
        para(
            site,
            "The browser keeps the struct as one JSON object, and every expression, statement, and patch names its fields through a `Sig`. A misspelled field has no function to call, so it fails to compile:",
        ),
        show(Snippet::WrongField),
        para(
            site,
            "The `S` in `Sig<S, T>` also stops a signal of one page from being used on another. An element of a `Counter` page is an `El<Counter>`, and it accepts only expressions over `Counter`.",
        ),
        heading(site, "Fields inside fields"),
        para(
            site,
            "A field may itself be a `Signals` struct. `at` reaches into it, and the result is a `Sig` like any other. A path is at most 8 fields deep.",
        ),
        show(Snippet::Nested),
        para(
            site,
            "A `Vec` of values or of structs is a list, which [Lists](learn/lists/) covers. An `Option` field has `is_some`, `is_none`, and `unwrap_or`.",
        ),
        heading(site, "The first render"),
        para(
            site,
            "`Page::new` takes the initial values, and `Page::render` evaluates every binding with them. Text, visibility, classes, attributes, form values, and list items are in the HTML before the runtime loads, and the runtime adopts those elements rather than building them again. The browser fetches nothing to show the page, because the data is the signal struct's JSON on `<body>`.",
        ),
        heading(site, "Computed signals"),
        para(
            site,
            "`Page::computed(signal, value)` keeps a field equal to an expression over other fields. Bindings read it like any field and run again only when its value changes, rather than each evaluating the expression. Computed signals are evaluated in the order they are declared, so one may read an earlier one.",
        ),
        demo::show(site, Demo::Cart),
        note(
            site,
            "Nothing stops a statement or the server from setting a computed field. The value it sets lasts until a field the expression reads changes.",
        ),
    ])
}

pub fn expressions(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Expressions).title(site)),
        lead(
            site,
            "An expression computes a value from the signals, and a statement changes them. Both are Rust values that the browser receives as JSON.",
        ),
        heading(site, "Expressions"),
        para(
            site,
            "An [`Expr<S, T>`](api:hypersig/struct.Expr.html) evaluates to a `T` in the browser. A `Sig` is an expression, and the operators and methods build larger ones. The server evaluates the same expressions for the first HTML.",
        ),
        show(Snippet::Operators),
        table_of(
            site,
            &["On", "Operations"],
            &[
                &[
                    "`i32` and `f64`",
                    "`+`, `-`, `*`, `%`, `min`, `max`, `to_fixed`, and `/` on `f64`. `to_f64` turns an `i32` into an `f64`.",
                ],
                &[
                    "`String`",
                    "`concat`, `trim`, `to_lowercase`, `to_uppercase`, `contains`, `starts_with`, `is_empty`, `char_count`",
                ],
                &["`bool`", "`and`, `or`, `!`"],
                &[
                    "Ordered values",
                    "`gt`, `ge`, `lt`, `le` on `i32`, `f64`, and `String`",
                ],
                &[
                    "Any value",
                    "`equals`, `not_equals`, `to_text` for display, `if_else(cond, then, otherwise)`",
                ],
                &[
                    "Structs",
                    "`at` for a field, `with` for a copy with one field replaced",
                ],
            ],
        ),
        para(
            site,
            "`Expr::lit(&value)` turns a Rust value into a constant expression, and most methods accept a plain `&value` where they take an expression. An expression of the wrong type is a compile error, with a message that names the type the place expects:",
        ),
        show(Snippet::WrongType),
        heading(site, "Statements"),
        para(
            site,
            "A [`Stmt<S>`](api:hypersig/struct.Stmt.html) runs in the browser on an event. `sig.set(value)` assigns, `flag.toggle()` flips a `bool`, and `then` runs one statement after another. `Stmt::when` and `Stmt::if_else` branch on a condition.",
        ),
        show(Snippet::Statements),
        heading(site, "Events"),
        para(
            site,
            "`on_click`, `on_input`, `on_submit`, `on_change`, and `on_blur` run a statement on their event, and `on(Event::Focus, ...)` covers the rest of [`Event`](api:hypersig/html/enum.Event.html). `on_key` filters `keydown` by a [`Key`](api:hypersig/html/enum.Key.html), and `on_debounced` waits until the event has stopped firing for a given time.",
        ),
        para(
            site,
            "`bind` ties a text field, a checkbox, or a number field to a signal in both directions. `bind_radio` and `bind_select` do the same for radio buttons and a `<select>`.",
        ),
        show(Snippet::Events),
        para(
            site,
            "The converter below computes Fahrenheit and a description from one `f64`, and each button is one statement.",
        ),
        demo::show(site, Demo::Temperature),
        note(
            site,
            "The browser runs only the operations hypersig provides. Anything else, such as formatting a date or calling a JavaScript library, goes through an action to the server and responds a network round trip later.",
        ),
    ])
}

pub fn lists(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Lists).title(site)),
        lead(
            site,
            "A `Vec` field is a list. `each` renders an element per item in the browser, and list statements change it without a request.",
        ),
        heading(site, "Rendering a list"),
        para(
            site,
            "[`each`](api:hypersig/html/fn.each.html) takes the list and a function that builds the element for one item. The function runs once on the server to build a template, receiving an [`Item`](api:hypersig/struct.Item.html) that stands for whichever item the browser is rendering. `item.at(field)` reads a field of the item, `item.index()` its position, and `item.value()` the item itself.",
        ),
        show(Snippet::Each),
        para(
            site,
            "`each` matches items to elements by position. [`each_keyed`](api:hypersig/html/fn.each_keyed.html) matches them by a key, so an item keeps its element when the list is reordered or an item before it is removed, and focus and the state of its inputs move with it.",
        ),
        heading(site, "Changing a list"),
        show(Snippet::ListStatements),
        table_of(
            site,
            &["Kind", "Methods on a list signal"],
            &[
                &[
                    "Statements",
                    "`push`, `extend`, `remove_at`, `clear`, `retain`, `swap`, `sort_by`, `set_all`, `update_where`",
                ],
                &["Expressions", "`len`, `is_empty`, `nth`, `count`"],
                &[
                    "On one item",
                    "`item.remove()`, and `set` on `item.at(field)`",
                ],
            ],
        ),
        subheading(site, "Item signals stay in the browser"),
        para(
            site,
            "A field of an item is a `Sig<S, T, Local>`, whose path is resolved per item in the browser. It has no `patch` method, so the server cannot address one item's field by position. The server replaces the whole list with a patch of the list field, or re-renders a part of the page as a fragment, which [Talking to the server](learn/server/) covers.",
        ),
        heading(site, "A to-do list"),
        para(
            site,
            "The demo builds a new item with `Expr::lit(&blank).with(...)`, which copies a constant struct and replaces two fields with expressions. The count of open items is an expression over the list.",
        ),
        demo::show(site, Demo::Todos),
    ])
}
