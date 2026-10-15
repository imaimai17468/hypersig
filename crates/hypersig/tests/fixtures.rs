//! Writes the HTML that `runtime/test/*.test.ts` runs against, so the browser
//! tests exercise exactly what the Rust builder emits.
//!
//! The test fails when a fixture on disk differs from what the builder writes
//! now. Run it with `HYPERSIG_FIXTURES=overwrite` to rewrite them.

use std::path::PathBuf;
use std::time::Duration;

use hypersig::html::{
    El, Event, Key, button, div, each, each_keyed, input, label, li, main, select, span, ul, when,
};
use hypersig::{Action, Expr, NodeRef, Page, Signals, Stmt};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Signals, Serialize, Deserialize, Clone, PartialEq)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct TodoApp {
    todos: Vec<Todo>,
    draft: String,
    next_id: i32,
    editing: Option<i32>,
}

const EDIT: NodeRef<TodoApp> = NodeRef::new("edit");

fn todos() -> (El<TodoApp>, TodoApp) {
    let todos = TodoApp::todos();
    let draft = TodoApp::draft();
    let next_id = TodoApp::next_id();
    let editing = TodoApp::editing();
    let blank = Expr::lit(&Todo {
        id: 0,
        title: String::new(),
        done: false,
    });
    let add = Stmt::when(
        !draft.trim().is_empty(),
        todos
            .push(
                blank
                    .with(Todo::id(), next_id)
                    .with(Todo::title(), draft.trim()),
            )
            .then(next_id.set(next_id + 1))
            .then(draft.set("")),
    );
    let el = main()
        .child(input().id("draft").bind(draft).on_key(Key::Enter, add))
        .child(ul().id("list").child(each_keyed(
            todos,
            |todo| todo.at(Todo::id()),
            |todo| {
                let id = todo.at(Todo::id());
                li().class_when("done", todo.at(Todo::done()))
                    .child(input().attr("type", "checkbox").bind(todo.at(Todo::done())))
                    .child(
                        label()
                            .bind_text(todo.at(Todo::title()))
                            .on_double_click(editing.set(id.some()).then(EDIT.focus())),
                    )
                    .child(when(
                        editing.equals(id.some()),
                        input()
                            .class("edit")
                            .node_ref(EDIT)
                            .bind(todo.at(Todo::title()))
                            .on_key(Key::Enter, editing.set(&None))
                            .on_key(Key::Escape, editing.set(&None)),
                    ))
                    .child(button().class("remove").on_click(todo.remove()).text("×"))
            },
        )))
        .child(
            span()
                .id("left")
                .bind_text(todos.count(|todo| !todo.at(Todo::done())).to_text()),
        )
        .child(
            button()
                .id("clear")
                .show(todos.count(|todo| todo.at(Todo::done())).gt(0))
                .on_click(todos.retain(|todo| !todo.at(Todo::done())))
                .text("Clear completed"),
        )
        .child(
            input()
                .id("all")
                .attr("type", "checkbox")
                .bind_checked(todos.count(|todo| !todo.at(Todo::done())).equals(0))
                .on_change(todos.set_all(
                    Todo::done(),
                    todos.count(|todo| !todo.at(Todo::done())).gt(0),
                )),
        );
    let signals = TodoApp {
        todos: vec![Todo {
            id: 1,
            title: "tea".to_owned(),
            done: false,
        }],
        draft: String::new(),
        next_id: 2,
        editing: None,
    };
    (el, signals)
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Size {
    Small,
    Large,
}

#[derive(Signals, Serialize, Deserialize)]
struct Controls {
    quantity: i32,
    price: f64,
    size: Size,
    gift: bool,
}

fn controls() -> (El<Controls>, Controls) {
    let quantity = Controls::quantity();
    let size = Controls::size();
    let el = main()
        .child(input().id("quantity").attr("type", "number").bind(quantity))
        .child(
            input()
                .id("price")
                .attr("type", "number")
                .bind(Controls::price()),
        )
        .child(
            select()
                .id("size")
                .bind_select(size, [(Size::Small, "Small"), (Size::Large, "Large")]),
        )
        .child(input().id("small").bind_radio(size, &Size::Small))
        .child(input().id("large").bind_radio(size, &Size::Large))
        .child(
            button()
                .id("buy")
                .attr_when("disabled", quantity.le(0))
                .bind_attr("title", (Controls::price() * 2.0).to_fixed(2))
                .bind_style("width", quantity.to_text().concat("rem"))
                .text("Buy"),
        )
        .child(span().id("total").bind_text(hypersig::if_else(
            size.equals(&Size::Large),
            "large",
            "small",
        )));
    let signals = Controls {
        quantity: 1,
        price: 2.5,
        size: Size::Small,
        gift: false,
    };
    (el, signals)
}

#[derive(Signals, Serialize, Deserialize)]
struct Timers {
    ticks: i32,
    query: String,
    searched: String,
    effects: i32,
    echo: String,
    saving: bool,
    error: String,
}

const SEARCH: Action<Timers> = Action::post("/search");

fn timers() -> (El<Timers>, Timers) {
    let ticks = Timers::ticks();
    let query = Timers::query();
    let effects = Timers::effects();
    let el = main()
        .child(
            span()
                .id("clock")
                .on_interval(Duration::from_millis(100), ticks.set(ticks + 1)),
        )
        .child(input().id("query").bind(query).on_debounced(
            Event::Input,
            Duration::from_millis(50),
            Timers::searched().set(query),
        ))
        .child(
            span()
                .id("effect")
                .on_signal_change(query, effects.set(effects + 1))
                .effect(Timers::echo().set(query.to_uppercase())),
        )
        .child(
            button().id("save").on_click(
                SEARCH
                    .call()
                    .indicator(Timers::saving())
                    .error(Timers::error()),
            ),
        );
    let signals = Timers {
        ticks: 0,
        query: String::new(),
        searched: String::new(),
        effects: 0,
        echo: String::new(),
        saving: false,
        error: String::new(),
    };
    (el, signals)
}

#[derive(Signals, Serialize, Deserialize)]
struct Tag {
    label: String,
    on: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Row {
    name: String,
    tags: Vec<Tag>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Nested {
    rows: Vec<Row>,
    hovered: String,
    focused: bool,
}

fn nested() -> (El<Nested>, Nested) {
    let hovered = Nested::hovered();
    let el = main()
        .child(ul().id("rows").child(each_keyed(
            Nested::rows(),
            |row| row.at(Row::name()),
            |row| {
                let tags = row.at(Row::tags());
                li().on(Event::MouseEnter, hovered.set(row.at(Row::name())))
                    .on(Event::MouseLeave, hovered.set(""))
                    .child(span().class("name").bind_text(row.at(Row::name())))
                    .child(
                        span()
                            .class("count")
                            .bind_text(tags.count(|tag| tag.at(Tag::on())).to_text()),
                    )
                    .child(ul().child(each(tags, |tag| {
                        li().child(
                            button()
                                .class_when("on", tag.at(Tag::on()))
                                .on_click(tag.at(Tag::on()).toggle())
                                .bind_text(
                                    row.at(Row::name()).concat("/").concat(tag.at(Tag::label())),
                                ),
                        )
                    })))
            },
        )))
        .child(
            div()
                .id("field")
                .on(Event::Focus, Nested::focused().set(true))
                .on_blur(Nested::focused().set(false))
                .child(input()),
        );
    let tag = |label: &str, on: bool| Tag {
        label: label.to_owned(),
        on,
    };
    let signals = Nested {
        rows: vec![
            Row {
                name: "a".to_owned(),
                tags: vec![tag("x", true), tag("y", false)],
            },
            Row {
                name: "b".to_owned(),
                tags: vec![tag("z", false)],
            },
        ],
        hovered: String::new(),
        focused: false,
    };
    (el, signals)
}

fn check(name: &str, el: &El<impl Signals>, signals: &impl Serialize) {
    let fixture = json!({ "html": el.render(), "signals": signals });
    write_or_compare(
        name,
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&fixture).expect("fixture JSON")
        ),
    );
}

fn write_or_compare(name: &str, text: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../runtime/test/fixtures")
        .join(format!("{name}.json"));
    if std::env::var("HYPERSIG_FIXTURES").as_deref() == Ok("overwrite") {
        std::fs::write(&path, text).expect("write fixture");
        return;
    }
    let on_disk = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        on_disk == text,
        "{} is out of date; rerun with HYPERSIG_FIXTURES=overwrite",
        path.display()
    );
}

#[derive(Signals, Serialize, Deserialize)]
struct Sums {
    a: i32,
    b: i32,
    sum: i32,
    doubled: i32,
}

fn computed_page() -> String {
    let (a, b, sum, doubled) = (Sums::a(), Sums::b(), Sums::sum(), Sums::doubled());
    Page::new(
        "computed",
        Sums {
            a: 1,
            b: 2,
            sum: 0,
            doubled: 0,
        },
    )
    .computed(sum, a + b)
    .computed(doubled, sum * 2)
    .child(
        main()
            .child(span().id("sum").bind_text(sum.to_text()))
            .child(span().id("doubled").bind_text(doubled.to_text())),
    )
    .render()
}

/// Writes the page as `Page::render` serves it, with the server-side values applied.
fn check_page<S: Signals>(name: &str, el: El<S>, signals: S) {
    let html = Page::new(name, signals).child(el).render();
    write_or_compare(
        name,
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({ "document": html })).expect("fixture JSON")
        ),
    );
}

#[test]
fn fixtures_match_the_builder() {
    let (el, signals) = todos();
    check("todos", &el, &signals);
    let (el, signals) = controls();
    check("controls", &el, &signals);
    let (el, signals) = timers();
    check("timers", &el, &signals);
    let (el, signals) = nested();
    check("nested", &el, &signals);
    let (el, signals) = todos();
    check_page("todos_page", el, signals);
    let (el, signals) = nested();
    check_page("nested_page", el, signals);
    let (el, signals) = controls();
    check_page("controls_page", el, signals);
    let document = computed_page();
    write_or_compare(
        "computed_page",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({ "document": document })).expect("fixture JSON")
        ),
    );
}
