//! Writes the HTML that `runtime/test/*.test.ts` runs against, so the browser
//! tests exercise exactly what the Rust builder emits.
//!
//! The test fails when a fixture on disk differs from what the builder writes
//! now. Run it with `HYPERSIG_FIXTURES=overwrite` to rewrite them.

use std::path::PathBuf;
use std::time::Duration;

use hypersig::html::{
    El, Event, Key, Orientation, a, button, div, each, each_keyed, el, input, label, li, main, p,
    select, span, textarea, ul, when,
};
use hypersig::style::value::{Color, pct, px};
use hypersig::style::{Keyframes, Media, Property, PseudoClass, Style};
use hypersig::{
    Action, CharClass, Expr, FileInput, Form, Fragment, IntlLocale, NodeRef, Page, Patches,
    PluralCategory, Rest, Route, Rules, Search, Sig, Signals, Stmt, StreamAction, Upload, Validate,
    if_else,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod class {
    use hypersig::html::Class;

    pub const DONE: Class = Class::new("done");
    pub const EDIT: Class = Class::new("edit");
    pub const REMOVE: Class = Class::new("remove");
    pub const NAME: Class = Class::new("name");
    pub const COUNT: Class = Class::new("count");
    pub const ON: Class = Class::new("on");
}

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
                li().class_when(class::DONE, todo.at(Todo::done()))
                    .child(input().attr("type", "checkbox").bind(todo.at(Todo::done())))
                    .child(
                        label()
                            .bind_text(todo.at(Todo::title()))
                            .on_double_click(editing.set(id.some()).then(EDIT.focus())),
                    )
                    .child(when(
                        editing.equals(id.some()),
                        input()
                            .class(class::EDIT)
                            .node_ref(EDIT)
                            .bind(todo.at(Todo::title()))
                            .on_key(Key::Enter, editing.set(&None))
                            .on_key(Key::Escape, editing.set(&None)),
                    ))
                    .child(
                        button()
                            .class(class::REMOVE)
                            .on_click(todo.remove())
                            .text("×"),
                    )
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
    note: Option<String>,
    limit: Option<i64>,
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
        .child(input().id("note").bind(Controls::note()))
        .child(
            input()
                .id("limit")
                .attr("type", "number")
                .bind(Controls::limit()),
        )
        .child(input().id("small").bind_radio(size, &Size::Small))
        .child(input().id("large").bind_radio(size, &Size::Large))
        .child(
            button()
                .id("buy")
                .attr_when("disabled", quantity.le(0))
                .bind_attr("title", (Controls::price() * 2.0).to_fixed(2))
                .bind_style(Property::Width, quantity.to_text().concat("rem"))
                .text("Buy"),
        )
        .child(
            span().id("price-label").bind_text(
                (Controls::price() * 1000.0)
                    .format(&hypersig::NumberFormat::new().digits(2).prefix("$")),
            ),
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
        note: Some("gift wrap".to_owned()),
        limit: None,
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
    fired: i32,
    find: String,
    finding: bool,
    found: String,
}

const SEARCH: Action<Timers> = Action::post("/search");
const LOOKUP: Action<Timers, String> = Action::get("/lookup");
const FEED: StreamAction<Timers> = StreamAction::new("/feed");

fn timers() -> (El<Timers>, Timers) {
    let ticks = Timers::ticks();
    let query = Timers::query();
    let effects = Timers::effects();
    let fired = Timers::fired();
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
        )
        .child(
            input().id("find").bind(Timers::find()).on_input(
                LOOKUP
                    .call_with(Timers::find())
                    .indicator(Timers::finding())
                    .latest(),
            ),
        )
        .child(
            button()
                .id("follow")
                .on_click(FEED.call().error(Timers::error()).reconnect()),
        )
        .child(
            span()
                .id("soon")
                .on_timeout(Duration::from_millis(100), fired.set(fired + 1)),
        )
        .child(
            span()
                .id("gone")
                .on_timeout(Duration::from_millis(100), fired.set(fired + 10)),
        );
    let signals = Timers {
        ticks: 0,
        query: String::new(),
        searched: String::new(),
        effects: 0,
        echo: String::new(),
        saving: false,
        error: String::new(),
        fired: 0,
        find: String::new(),
        finding: false,
        found: String::new(),
    };
    (el, signals)
}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Signup {
    email: String,
    password: String,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let email = form.at(Signup::email());
        rules
            .check(
                Signup::email(),
                !email.is_empty(),
                "Enter an email address.",
            )
            .check(
                Signup::email(),
                email.contains("@"),
                "That is not an email address.",
            )
            .check(
                Signup::password(),
                form.at(Signup::password()).char_count().ge(8),
                "Use 8 or more characters.",
            );
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

const JOIN: Action<Join, Signup> = Action::post("/join");

fn join() -> (El<Join>, Join) {
    let signup = Join::signup();
    let (email, password) = (Signup::email(), Signup::password());
    let el = signup
        .form(JOIN.call_with(signup.value()))
        .id("join")
        .child(
            input()
                .id("email")
                .bind(signup.field(email))
                .on_blur(signup.touch(email)),
        )
        .child(span().id("email-error").bind_text(signup.error(email)))
        .child(
            input()
                .id("password")
                .bind(signup.field(password))
                .on_blur(signup.touch(password)),
        )
        .child(
            span()
                .id("password-error")
                .bind_text(signup.error(password)),
        )
        .child(span().id("dirty").show(signup.is_dirty()).text("unsaved"))
        .child(span().id("valid").show(signup.is_valid()).text("ok"))
        .child(button().id("submit").attr("type", "submit"))
        .child(
            button()
                .id("reset")
                .attr("type", "button")
                .on_click(signup.reset()),
        );
    let signals = Join {
        signup: Form::new(Signup {
            email: "ada@example.com".to_owned(),
            password: String::new(),
        }),
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
                    .child(span().class(class::NAME).bind_text(row.at(Row::name())))
                    .child(
                        span()
                            .class(class::COUNT)
                            .bind_text(tags.count(|tag| tag.at(Tag::on())).to_text()),
                    )
                    .child(ul().child(each(tags, |tag| {
                        li().child(
                            button()
                                .class_when(class::ON, tag.at(Tag::on()))
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
                .child(input())
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

#[derive(Signals, Serialize, Deserialize)]
struct Line {
    id: i32,
    label: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Table {
    lines: Vec<Line>,
    next_id: i32,
    stamp: f64,
}

fn table() -> (El<Table>, Table) {
    let lines = Table::lines();
    let next_id = Table::next_id();
    let words = Expr::lit(&vec!["a".to_owned(), "b".to_owned()]);
    let blank = Expr::lit(&Line {
        id: 0,
        label: String::new(),
    });
    let el = main()
        .child(
            button().id("run").on_click(
                lines
                    .clear()
                    .then(lines.extend(3, |i| {
                        blank.clone().with(Line::id(), next_id + i).with(
                            Line::label(),
                            words.clone().nth(Expr::random_below(2)).unwrap_or("?"),
                        )
                    }))
                    .then(next_id.set(next_id + 3)),
            ),
        )
        .child(button().id("update").on_click(lines.update_where(
            |line| (line.index() % 2).equals(0),
            Line::label(),
            |line| line.at(Line::label()).concat(" !"),
        )))
        .child(button().id("swap").on_click(lines.swap(0, 2)))
        .child(
            button()
                .id("sort")
                .on_click(lines.sort_by(|line| line.at(Line::id()))),
        )
        .child(
            button()
                .id("sort-desc")
                .on_click(lines.sort_by_desc(|line| line.at(Line::id()))),
        )
        .child(
            button()
                .id("stamp")
                .on_click(Table::stamp().set(Expr::now())),
        )
        .child(ul().id("lines").child(each_keyed(
            lines,
            |line| line.at(Line::id()),
            |line| li().bind_text(line.at(Line::label())),
        )));
    let signals = Table {
        lines: Vec::new(),
        next_id: 1,
        stamp: 0.0,
    };
    (el, signals)
}

#[derive(Signals, Serialize, Deserialize)]
struct Roving {
    tab: i32,
    taken: i32,
}

const MENU: NodeRef<Roving> = NodeRef::new("menu");

/// A tab list whose third tab is disabled and whose last tab takes the right
/// arrow for itself, and a vertical menu inside it whose items belong to the
/// menu rather than to the tab list.
fn roving() -> (El<Roving>, Roving) {
    let tab = Roving::tab();
    let taken = Roving::taken();
    let tab_button = |i: i32| {
        button()
            .id(format!("tab{i}"))
            .bind_attr("tabindex", if_else(tab.equals(i), "0", "-1"))
            .on(Event::Focus, tab.set(i))
    };
    let el = main().child(
        div()
            .id("tabs")
            .roving_focus(Orientation::Horizontal)
            .child(tab_button(0))
            .child(tab_button(1))
            .child(tab_button(2).attr("disabled", ""))
            .child(tab_button(3).on_key(Key::ArrowRight, taken.set(taken + 1)))
            .child(
                div()
                    .id("menu")
                    .roving_focus(Orientation::Vertical)
                    .node_ref(MENU)
                    .on_key(Key::Tab, MENU.hide_popover())
                    .on_key(Key::Tab.shift(), MENU.hide_popover())
                    .on_key(Key::Enter, MENU.show_popover())
                    .children(
                        (0..2).map(|i| button().id(format!("item{i}")).attr("tabindex", "-1")),
                    ),
            ),
    );
    (el, Roving { tab: 0, taken: 0 })
}

#[derive(Signals, Serialize, Deserialize)]
struct Combo {
    query: String,
    active: String,
    picked: String,
    downs: i32,
}

const OPTIONS: NodeRef<Combo> = NodeRef::new("options");

/// A text field that tracks the active option of a listbox whose second
/// option is hidden while the query is empty and whose third is disabled.
fn combo() -> (El<Combo>, Combo) {
    let query = Combo::query();
    let active = Combo::active();
    let picked = Combo::picked();
    let downs = Combo::downs();
    let option = |i: i32| {
        div()
            .id(format!("o{i}"))
            .attr("role", "option")
            .on_click(picked.set(format!("o{i}")))
    };
    let el = main()
        .child(
            input()
                .id("field")
                .bind(query)
                .active_descendant(OPTIONS, active)
                .on_key(Key::ArrowDown, downs.set(downs + 1)),
        )
        .child(
            div()
                .attr("role", "listbox")
                .node_ref(OPTIONS)
                .child(option(0))
                .child(option(1).show(!query.is_empty()))
                .child(option(2).attr("aria-disabled", "true"))
                .child(option(3)),
        );
    let signals = Combo {
        query: String::new(),
        active: String::new(),
        picked: String::new(),
        downs: 0,
    };
    (el, signals)
}

#[derive(Signals, Serialize, Deserialize)]
struct Bar {
    id: i32,
    height: i32,
    marks: Vec<i32>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Plot {
    bars: Vec<Bar>,
    marked: bool,
    collected: Vec<i32>,
}

/// A list, a nested list, a condition, and a `show` inside an `<svg>`,
/// where the parser reads `<template>` as an SVG element.
fn plot() -> (El<Plot>, Plot) {
    let heights = Plot::bars();
    let tallest = heights
        .max_of(|bar| bar.at(Bar::height()))
        .unwrap_or(0)
        .to_f64();
    let shortest = heights
        .min_of(|bar| bar.at(Bar::height()))
        .unwrap_or(0)
        .to_f64();
    let extent = tallest.let_in(|top| {
        (top.clone() / 3.0)
            .round()
            .to_text()
            .concat(" ")
            .concat(Expr::lit(&10.0).pow(top.log10().floor()).to_text())
            .concat(" ")
            .concat((shortest / 4.0).ceil().to_text())
            .concat(" ")
            .concat(heights.sum_of(|bar| bar.at(Bar::height())).to_text())
    });
    let el = main()
        .child(button().id("mark").on_click(Plot::marked().toggle()))
        .child(span().id("extent").bind_text(extent))
        .child(
            span().id("second").bind_text(
                heights
                    .map(|bar| bar.at(Bar::id()))
                    .nth(1)
                    .unwrap_or(0)
                    .to_text(),
            ),
        )
        .child(
            button()
                .id("collect")
                .on_click(Plot::collected().append(heights.map(|bar| bar.at(Bar::id())))),
        )
        .child(
            span()
                .id("collected")
                .bind_text(Plot::collected().len().to_text()),
        )
        .child(
            el("svg")
                .id("plot")
                .attr("viewBox", "0 0 100 100")
                .child(
                    el("rect")
                        .id("frame")
                        .attr("width", "100")
                        .attr("height", "100")
                        .show(Plot::marked()),
                )
                .child(each_keyed(
                    Plot::bars(),
                    |bar| bar.at(Bar::id()),
                    |bar| {
                        el("g")
                            .child(
                                el("rect")
                                    .attr("width", "5")
                                    .bind_attr("height", bar.at(Bar::height()).to_text()),
                            )
                            .child(each(bar.at(Bar::marks()), |mark| {
                                el("line").bind_attr("y1", mark.value().to_text())
                            }))
                    },
                ))
                .child(when(Plot::marked(), el("circle").id("dot").attr("r", "2"))),
        );
    let signals = Plot {
        bars: vec![
            Bar {
                id: 1,
                height: 10,
                marks: vec![1, 2],
            },
            Bar {
                id: 2,
                height: 20,
                marks: Vec::new(),
            },
        ],
        marked: false,
        collected: Vec::new(),
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
struct Values {
    zip: String,
    code: String,
    count: i32,
}

/// Text the server writes into the first HTML and the runtime writes again,
/// one `<span>` per value operation, so a test can compare the two.
fn values() -> (El<Values>, Values) {
    let zip = Values::zip();
    let code = Values::code();
    let count = Values::count();
    let yes_no = |test: Expr<Values, bool>| if_else(test, "yes", "no");
    let el = main().child(
        div()
            .id("values")
            .child(
                span()
                    .id("zip-digits")
                    .bind_text(yes_no(zip.all_chars(CharClass::AsciiDigit))),
            )
            .child(
                span()
                    .id("code-digits")
                    .bind_text(yes_no(code.all_chars(CharClass::AsciiDigit))),
            )
            .child(
                span()
                    .id("code-upper")
                    .bind_text(yes_no(code.any_char(CharClass::AsciiUppercase))),
            )
            .child(
                span()
                    .id("code-space")
                    .bind_text(yes_no(code.any_char(CharClass::AsciiWhitespace))),
            )
            .child(
                span()
                    .id("items-en")
                    .bind_text(
                        count
                            .to_text()
                            .concat(count.plural(IntlLocale::En, |category| match category {
                                PluralCategory::One => " item",
                                _ => " items",
                            })),
                    ),
            )
            .child(
                span().id("items-ja").bind_text(
                    count
                        .to_text()
                        .concat(count.plural(IntlLocale::Ja, |_| " 点")),
                ),
            ),
    );
    let signals = Values {
        zip: "1500001".to_owned(),
        code: "Ab 12\u{ff11}".to_owned(),
        count: 1,
    };
    (el, signals)
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
    .runtime_src(RUNTIME_SRC)
    .computed(sum, a + b)
    .computed(doubled, sum * 2)
    .bind_title(doubled.to_text().concat(" doubled"))
    .child(
        main()
            .child(span().id("sum").bind_text(sum.to_text()))
            .child(span().id("doubled").bind_text(doubled.to_text())),
    )
    .render()
}

#[derive(Signals, Serialize, Deserialize)]
struct Shortcuts {
    palette: i32,
    saves: i32,
    escapes: i32,
    newlines: i32,
    text: String,
}

/// A page with a page-wide shortcut, a section whose plain `s` must not reach
/// the field inside it, and a field that takes Escape and Enter for itself.
fn shortcuts_page() -> String {
    Page::new(
        "shortcuts",
        Shortcuts {
            palette: 0,
            saves: 0,
            escapes: 0,
            newlines: 0,
            text: String::new(),
        },
    )
    .runtime_src(RUNTIME_SRC)
    .on_key(
        Key::Char('k').primary(),
        Shortcuts::palette().set(Shortcuts::palette() + 1),
    )
    .on_key(
        Key::Char('/'),
        Shortcuts::palette().set(Shortcuts::palette() + 10),
    )
    .child(
        main()
            .id("section")
            .attr("tabindex", "-1")
            .on_key(
                Key::Char('s'),
                Shortcuts::saves().set(Shortcuts::saves() + 1),
            )
            .on_key(
                Key::Char('s').ctrl().shift(),
                Shortcuts::saves().set(Shortcuts::saves() + 100),
            )
            .child(input().id("field").bind(Shortcuts::text()))
            .child(textarea().id("notes").on_key(
                Key::Escape,
                Shortcuts::escapes().set(Shortcuts::escapes() + 1),
            ))
            .child(
                input()
                    .id("own")
                    .on_key(
                        Key::Char('s'),
                        Shortcuts::saves().set(Shortcuts::saves() + 1000),
                    )
                    .on_key(
                        Key::Enter.shift(),
                        Shortcuts::newlines().set(Shortcuts::newlines() + 1),
                    ),
            ),
    )
    .render()
}

#[derive(Signals, Serialize, Deserialize)]
struct Styled {
    on: bool,
}

/// A page whose rules sit in a breakpoint bucket and the toggled layer, with
/// an empty slot a patch fills.
fn styled() -> (El<Styled>, Styled) {
    let el = main()
        .child(
            div()
                .id("box")
                .css(Style::new().on(Media::Md, Style::new().width(pct(50))))
                .css_when(Style::new().color(Color::CRIMSON), Styled::on())
                .on_click(Styled::on().toggle()),
        )
        .child(div().id("slot"));
    (el, Styled { on: false })
}

/// Fills the slot with rules in two buckets the page does not have, one that
/// sorts before the page's breakpoint bucket and one between, and with an
/// `@keyframes` rule, which sorts after every layer.
struct Panel;

impl Fragment for Panel {
    type Signals = Styled;
    const ID: &'static str = "slot";

    fn render(&self) -> El<Styled> {
        let pulse = Keyframes::new("pulse")
            .at(0, Style::new().opacity(1))
            .at(50, Style::new().opacity(0.5));
        div().css(
            Style::new()
                .width(pct(100))
                .padding(px(4))
                .keyframes(&pulse)
                .on(PseudoClass::Hover, Style::new().color(Color::BLUE)),
        )
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Member {
    team: String,
    id: i32,
}

#[derive(Signals, Search, Serialize, Deserialize, Clone)]
struct Filter {
    q: String,
    page: i32,
    open: bool,
    tag: Option<String>,
    color: Vec<String>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Roster {
    members: Vec<Member>,
    filter: Filter,
    file: Rest,
}

hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route = "/" => no_page;
    const MEMBER: Route<(String, i32)> = "/teams/{team}/members/{id}" => no_page;
    const FIND: Route<(), Filter> = "/search" => no_page;
    const TEAM_SEARCH: Route<String, Filter> = "/teams/{team}" => no_page;
    const FILE: Route<Rest> = "/files/{*path}" => no_page;
}

#[expect(
    dead_code,
    reason = "the fixtures link to these pages and never serve them"
)]
async fn no_page<A>((): (), _: A) -> Option<Page<Roster>> {
    None
}

/// Links whose parameters need percent-encoding, so the server's encoding
/// and the browser's `encodeURIComponent` are compared on the same values.
fn links() -> (El<Roster>, Roster) {
    let el = main()
        .child(a().id("home").href(HOME.link()))
        .child(ul().id("members").child(each(Roster::members(), |member| {
            let params = || (member.at(Member::team()), member.at(Member::id()));
            let search = Expr::lit(&filter()).with(Filter::page(), member.at(Member::id()));
            li().child(a().href(MEMBER.link_with(params())))
                .child(button().on_click(MEMBER.link_with(params()).navigate()))
                .child(a().href(TEAM_SEARCH.link_with((member.at(Member::team()), search))))
        })))
        .child(a().id("search-fixed").href(FIND.link_with(&filter())))
        .child(
            a().id("search-bound")
                .href(FIND.link_with(Roster::filter())),
        )
        .child(
            a().id("file-fixed")
                .href(FILE.link_with(&Rest::new("docs/a b/ü?.md"))),
        )
        .child(a().id("file-bound").href(FILE.link_with(Roster::file())))
        .child(
            button()
                .id("push")
                .on_click(FIND.link_with(Roster::filter()).push_url()),
        )
        .child(
            button()
                .id("replace")
                .on_click(FILE.link_with(Roster::file()).replace_url()),
        );
    let signals = Roster {
        members: vec![
            Member {
                team: "red team".to_owned(),
                id: 1,
            },
            Member {
                team: "café/ü?&#".to_owned(),
                id: -2,
            },
        ],
        filter: Filter {
            tag: Some("a+b".to_owned()),
            color: vec!["red".to_owned(), "dark blue".to_owned()],
            ..filter()
        },
        file: Rest::new("docs/a b/ü?.md"),
    };
    (el, signals)
}

/// Search parameters whose fields are declared out of name order, with
/// values that need percent-encoding, a `None` that is left out, and an
/// empty list that writes nothing.
fn filter() -> Filter {
    Filter {
        q: "a b&c=d#é".to_owned(),
        page: 2,
        open: true,
        tag: None,
        color: Vec::new(),
    }
}

/// The runtime's path in a page fixture, fixed so that a change to the
/// runtime, which changes `RUNTIME_PATH`, leaves the fixtures as they are.
const RUNTIME_SRC: &str = "/_hypersig/runtime.js";

/// Writes the page as `Page::render` serves it, with the server-side values applied.
fn check_page<S: Signals>(name: &str, el: El<S>, signals: S) {
    let html = Page::new(name, signals)
        .runtime_src(RUNTIME_SRC)
        .child(el)
        .render();
    write_or_compare(
        name,
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({ "document": html })).expect("fixture JSON")
        ),
    );
}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Note {
    id: i32,
    body: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    notes: Vec<Note>,
    older: Vec<Note>,
}

fn note(id: i32, body: &str) -> Note {
    Note {
        id,
        body: body.to_owned(),
    }
}

fn feed() -> (El<Feed>, Feed) {
    let el = main()
        .child(
            button()
                .id("older")
                .on_click(Feed::notes().prepend(Feed::older()))
                .text("Older"),
        )
        .child(ul().id("feed").child(each_keyed(
            Feed::notes(),
            |note| note.at(Note::id()),
            |note| li().bind_text(note.at(Note::body())),
        )));
    let signals = Feed {
        notes: vec![note(1, "one"), note(2, "two")],
        older: vec![note(-1, "minus one"), note(0, "zero")],
    };
    (el, signals)
}

fn feed_patches() -> String {
    let notes = Feed::notes();
    let by_id = |note: hypersig::Item<Feed, Note>| note.at(Note::id());
    Patches::<Feed>::new()
        .list(notes.patch_append(&[note(3, "three")]))
        .list(
            notes
                .patch_prepend(&[note(0, "zero")])
                .and(notes.patch_upsert(&[note(1, "one!"), note(4, "four")], by_id)),
        )
        .list(notes.patch_remove(&[2, 3], by_id))
        .to_event_stream()
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
    let (el, signals) = table();
    check("table", &el, &signals);
    let (el, signals) = roving();
    check("roving", &el, &signals);
    let (el, signals) = plot();
    check("plot", &el, &signals);
    let (el, signals) = plot();
    check_page("plot_page", el, signals);
    let (el, signals) = combo();
    check("combo", &el, &signals);
    let (el, signals) = join();
    check("join", &el, &signals);
    let (el, signals) = join();
    check_page("join_page", el, signals);
    let (el, signals) = links();
    check("links", &el, &signals);
    let (el, signals) = todos();
    check_page("todos_page", el, signals);
    let (el, signals) = links();
    check_page("links_page", el, signals);
    let (el, signals) = nested();
    check_page("nested_page", el, signals);
    let (el, signals) = controls();
    check_page("controls_page", el, signals);
    let (el, signals) = styled();
    check_page("styled_page", el, signals);
    write_or_compare(
        "styled_patch",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "stream": Patches::<Styled>::new().element(&Panel).to_event_stream()
            }))
            .expect("fixture JSON")
        ),
    );
    write_or_compare(
        "history_patch",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "stream": Patches::<Roster>::new()
                    .push_url(FIND.url(filter()))
                    .replace_url(MEMBER.url(("red team".to_owned(), 1)))
                    .to_event_stream()
            }))
            .expect("fixture JSON")
        ),
    );
    let (el, signals) = feed();
    check("feed", &el, &signals);
    write_or_compare(
        "feed_patches",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({ "stream": feed_patches() }))
                .expect("fixture JSON")
        ),
    );
    let (el, signals) = values();
    check("values", &el, &signals);
    let (el, signals) = values();
    check_page("values_page", el, signals);
    let document = computed_page();
    write_or_compare(
        "computed_page",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({ "document": document })).expect("fixture JSON")
        ),
    );
    let document = shortcuts_page();
    write_or_compare(
        "shortcuts_page",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({ "document": document })).expect("fixture JSON")
        ),
    );
}

#[derive(Signals, Serialize, Deserialize)]
struct Uploads {
    caption: String,
    uploading: bool,
    error: String,
    saved: String,
}

const PICTURES: FileInput<Uploads> = FileInput::new("pictures");
const SEND: Upload<Uploads, String> = Upload::post("/pictures");
// The same name as the file input, which the browser looks up apart from it.
const PICTURES_NOTE: NodeRef<Uploads> = NodeRef::new("pictures");

fn uploads() -> (El<Uploads>, Uploads) {
    let el = main()
        .child(
            p().id("note")
                .attr("tabindex", "-1")
                .node_ref(PICTURES_NOTE)
                .text("Pick pictures"),
        )
        .child(button().id("focus").on_click(PICTURES_NOTE.focus()))
        .child(PICTURES.input().id("pictures").attr("multiple", ""))
        .child(input().id("caption").bind(Uploads::caption()))
        .child(
            button().id("send").on_click(
                SEND.call_with(PICTURES, Uploads::caption())
                    .indicator(Uploads::uploading())
                    .error(Uploads::error())
                    .then(PICTURES.clear()),
            ),
        );
    let signals = Uploads {
        caption: String::new(),
        uploading: false,
        error: String::new(),
        saved: String::new(),
    };
    (el, signals)
}

#[test]
fn the_upload_fixture_matches_the_builder() {
    let (el, signals) = uploads();
    check("upload", &el, &signals);
}
