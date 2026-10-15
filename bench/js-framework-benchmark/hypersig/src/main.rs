//! js-framework-benchmark's keyed table in hypersig.
//!
//! `cargo run --release` writes `index.html` and `bundled-dist/hypersig.js`.
//! The page is static: every operation runs in the browser on the `rows`
//! signal, and the browser draws the rows' labels, as the benchmark requires.
//! The markup follows the benchmark's `vanillajs` reference.

use std::fs;

use hypersig::html::{El, Node, a, button, div, each_keyed, h1, span, table, tbody, td, tr};
use hypersig::{Expr, Page, Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Row {
    id: i32,
    label: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Bench {
    rows: Vec<Row>,
    next_id: i32,
    selected: i32,
}

const ADJECTIVES: &[&str] = &[
    "pretty", "large", "big", "small", "tall", "short", "long", "handsome", "plain", "quaint",
    "clean", "elegant", "easy", "angry", "crazy", "helpful", "mushy", "odd", "unsightly",
    "adorable", "important", "inexpensive", "cheap", "expensive", "fancy",
];
const COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];
const NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

/// One of `words`, drawn in the browser.
fn pick(words: &[&str]) -> Expr<Bench, String> {
    let words: Vec<String> = words.iter().map(|&word| word.to_owned()).collect();
    let bound = i32::try_from(words.len()).expect("a word list fits in i32");
    Expr::lit(&words)
        .nth(Expr::random_below(bound))
        .unwrap_or("")
}

/// Appends `count` rows with the next ids and random labels.
fn create(count: i32) -> Stmt<Bench> {
    let rows = Bench::rows();
    let next_id = Bench::next_id();
    let blank = Expr::lit(&Row {
        id: 0,
        label: String::new(),
    });
    rows.extend(count, |i| {
        let label = pick(ADJECTIVES)
            .concat(" ")
            .concat(pick(COLOURS))
            .concat(" ")
            .concat(pick(NOUNS));
        blank.with(Row::id(), next_id + i).with(Row::label(), label)
    })
    .then(next_id.set(next_id + count))
}

fn action(id: &'static str, text: &str, stmt: Stmt<Bench>) -> El<Bench> {
    div().class("col-sm-6").class("smallpad").child(
        button()
            .attr("type", "button")
            .class("btn")
            .class("btn-primary")
            .class("btn-block")
            .id(id)
            .on_click(stmt)
            .text(text),
    )
}

fn rows() -> Node<Bench> {
    let selected = Bench::selected();
    each_keyed(
        Bench::rows(),
        |row| row.at(Row::id()),
        |row| {
            let id = row.at(Row::id());
            tr().class_when("danger", selected.equals(id))
                .child(td().class("col-md-1").bind_text(id.to_text()))
                .child(
                    td().class("col-md-4").child(
                        a().on_click(selected.set(id))
                            .bind_text(row.at(Row::label())),
                    ),
                )
                .child(
                    td().class("col-md-1").child(
                        a().on_click(row.remove()).child(
                            span()
                                .class("glyphicon")
                                .class("glyphicon-remove")
                                .attr("aria-hidden", "true"),
                        ),
                    ),
                )
                .child(td().class("col-md-6"))
        },
    )
}

fn page() -> Page<Bench> {
    let rows_signal = Bench::rows();
    let buttons = div()
        .class("row")
        .child(action("run", "Create 1,000 rows", rows_signal.clear().then(create(1000))))
        .child(action("runlots", "Create 10,000 rows", rows_signal.clear().then(create(10_000))))
        .child(action("add", "Append 1,000 rows", create(1000)))
        .child(action(
            "update",
            "Update every 10th row",
            rows_signal.update_where(
                |row| (row.index() % 10).equals(0),
                Row::label(),
                |row| row.at(Row::label()).concat(" !!!"),
            ),
        ))
        .child(action("clear", "Clear", rows_signal.clear()))
        .child(action("swaprows", "Swap Rows", rows_signal.swap(1, 998)));
    let jumbotron = div().class("jumbotron").child(
        div().class("row")
            .child(div().class("col-md-6").child(h1().text("hypersig-\"keyed\"")))
            .child(div().class("col-md-6").child(buttons)),
    );
    let signals = Bench {
        rows: Vec::new(),
        next_id: 1,
        selected: 0,
    };
    Page::new("hypersig-\"keyed\"", signals)
        .stylesheet("/css/currentStyle.css")
        .runtime_src("bundled-dist/hypersig.js")
        .child(
            div().id("main").child(
                div()
                    .class("container")
                    .child(jumbotron)
                    .child(
                        table()
                            .class("table")
                            .class("table-hover")
                            .class("table-striped")
                            .class("test-data")
                            .child(tbody().id("tbody").child(rows())),
                    )
                    .child(
                        span()
                            .class("preloadicon")
                            .class("glyphicon")
                            .class("glyphicon-remove")
                            .attr("aria-hidden", "true"),
                    ),
            ),
        )
}

fn main() -> std::io::Result<()> {
    fs::write("index.html", page().render())?;
    fs::create_dir_all("bundled-dist")?;
    fs::write("bundled-dist/hypersig.js", hypersig::RUNTIME_JS)
}
