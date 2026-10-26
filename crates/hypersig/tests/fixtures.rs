//! Writes the HTML that `runtime/test/*.test.ts` runs against, so the browser
//! tests exercise exactly what the Rust builder emits.
//!
//! The test fails when a fixture on disk differs from what the builder writes
//! now. Run it with `HYPERSIG_FIXTURES=overwrite` to rewrite them.

use std::path::PathBuf;
use std::time::Duration;

use hypersig::html::{
    CustomEvent, El, Event, Id, Key, Orientation, Role, a, attr, button, div, each, each_keyed, el,
    input, label, li, main, p, select, span, textarea, ul, when,
};
use hypersig::style::value::{Color, pct, px};
use hypersig::style::{Keyframes, Media, Property, PseudoClass, Style};
use hypersig::{
    Action, CharClass, DateFormat, DateStyle, Expr, FileInput, Form, Fragment, IntlLocale, Page,
    Part, Patches, PluralCategory, Rest, Route, Rules, SafeUrl, Search, Segment, Sig, Signals,
    Stmt, StreamAction, TimeStyle, TimeZone, Timestamp, Upload, Validate, if_else,
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

hypersig::node_ref!(const EDIT: NodeRef<TodoApp>);

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
        .child(
            input()
                .id(Id::new("draft"))
                .bind(draft)
                .on_key(Key::Enter, add),
        )
        .child(ul().id(Id::new("list")).child(each_keyed(
            todos,
            |todo| todo.at(Todo::id()),
            |todo| {
                let id = todo.at(Todo::id());
                li().class_when(class::DONE, todo.at(Todo::done()))
                    .child(
                        input()
                            .attr(attr::TYPE, "checkbox")
                            .bind(todo.at(Todo::done())),
                    )
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
                .id(Id::new("left"))
                .bind_text(todos.count(|todo| !todo.at(Todo::done())).to_text()),
        )
        .child(
            button()
                .id(Id::new("clear"))
                .show(todos.count(|todo| todo.at(Todo::done())).gt(0))
                .on_click(todos.retain(|todo| !todo.at(Todo::done())))
                .text("Clear completed"),
        )
        .child(
            input()
                .id(Id::new("all"))
                .attr(attr::TYPE, "checkbox")
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
        .child(
            input()
                .id(Id::new("quantity"))
                .attr(attr::TYPE, "number")
                .bind(quantity),
        )
        .child(
            input()
                .id(Id::new("price"))
                .attr(attr::TYPE, "number")
                .bind(Controls::price()),
        )
        .child(
            select()
                .id(Id::new("size"))
                .bind_select(size, [(Size::Small, "Small"), (Size::Large, "Large")]),
        )
        .child(input().id(Id::new("note")).bind(Controls::note()))
        .child(
            input()
                .id(Id::new("limit"))
                .attr(attr::TYPE, "number")
                .bind(Controls::limit()),
        )
        .child(input().id(Id::new("small")).bind_radio(size, &Size::Small))
        .child(input().id(Id::new("large")).bind_radio(size, &Size::Large))
        .child(
            button()
                .id(Id::new("buy"))
                .attr_when(attr::DISABLED, quantity.le(0))
                .bind_attr(attr::TITLE, (Controls::price() * 2.0).to_fixed(2))
                .bind_style(Property::Width, quantity.to_text().concat("rem"))
                .text("Buy"),
        )
        .child(
            span().id(Id::new("price-label")).bind_text(
                (Controls::price() * 1000.0)
                    .format(&hypersig::NumberFormat::new().digits(2).prefix("$")),
            ),
        )
        .child(span().id(Id::new("total")).bind_text(hypersig::if_else(
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
    failure: i32,
    fired: i32,
    find: String,
    finding: bool,
    found: String,
}

hypersig::routes! {
    fn timer_actions(Router<()>);
    action SEARCH: Action<Timers> = post "/search" => no_answer;
    action LOOKUP: Action<Timers, String> = get "/lookup" => no_answer;
    stream FEED: StreamAction<Timers> = "/feed" => no_stream;
}

fn timers() -> (El<Timers>, Timers) {
    let ticks = Timers::ticks();
    let query = Timers::query();
    let effects = Timers::effects();
    let fired = Timers::fired();
    let el = main()
        .child(
            span()
                .id(Id::new("clock"))
                .on_interval(Duration::from_millis(100), ticks.set(ticks + 1)),
        )
        .child(input().id(Id::new("query")).bind(query).on_debounced(
            Event::Input,
            Duration::from_millis(50),
            Timers::searched().set(query),
        ))
        .child(
            span()
                .id(Id::new("effect"))
                .on_signal_change(query, effects.set(effects + 1))
                .effect(Timers::echo().set(query.to_uppercase())),
        )
        .child(
            button().id(Id::new("save")).on_click(
                SEARCH
                    .call()
                    .indicator(Timers::saving())
                    .error(Timers::error())
                    .status(Timers::failure()),
            ),
        )
        .child(
            input().id(Id::new("find")).bind(Timers::find()).on_input(
                LOOKUP
                    .call_with(Timers::find())
                    .indicator(Timers::finding())
                    .latest(),
            ),
        )
        .child(
            button().id(Id::new("follow")).on_click(
                FEED.call()
                    .error(Timers::error())
                    .status(Timers::failure())
                    .reconnect(),
            ),
        )
        .child(
            span()
                .id(Id::new("soon"))
                .on_timeout(Duration::from_millis(100), fired.set(fired + 1)),
        )
        .child(
            span()
                .id(Id::new("gone"))
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
        failure: 0,
        fired: 0,
        find: String::new(),
        finding: false,
        found: String::new(),
    };
    (el, signals)
}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Letter {
    id: i32,
    text: String,
    state: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Outbox {
    letters: Vec<Letter>,
    draft: String,
    next: i32,
}

hypersig::routes! {
    fn outbox_actions(Router<()>);
    action DELIVER: Action<Outbox, i32> = post "/send" => no_answer;
}

/// Sends each line in its own call, and marks only the line whose call failed.
fn outbox() -> (El<Outbox>, Outbox) {
    let letters = Outbox::letters();
    let next = Outbox::next();
    let draft = Outbox::draft();
    let blank = Expr::lit(&Letter {
        id: 0,
        text: String::new(),
        state: "sending".to_owned(),
    });
    let send = next.let_in_stmt(|id| {
        let mark = |state: Expr<Outbox, String>| {
            letters.update_where(
                |line| line.at(Letter::id()).equals(&id),
                Letter::state(),
                |_| state,
            )
        };
        letters
            .push(
                blank
                    .with(Letter::id(), id.clone())
                    .with(Letter::text(), draft),
            )
            .then(
                DELIVER
                    .call_with(id.clone())
                    .on_error(mark)
                    .on_success(mark(Expr::text("sent"))),
            )
            .then(next.set(next + 1))
    });
    let el = main()
        .child(input().id(Id::new("draft")).bind(draft))
        .child(button().id(Id::new("send")).on_click(send))
        .child(ul().id(Id::new("letters")).child(each(letters, |line| {
            li().child(span().class(class::NAME).bind_text(line.at(Letter::text())))
                .child(
                    span()
                        .class(class::COUNT)
                        .bind_text(line.at(Letter::state())),
                )
                .child(
                    button().class(class::EDIT).on_click(
                        line.at(Letter::state()).set("sending").then(
                            DELIVER
                                .call_with(line.at(Letter::id()))
                                .on_error(|message| line.at(Letter::state()).set(message))
                                .on_success(line.at(Letter::state()).set("sent")),
                        ),
                    ),
                )
        })));
    let signals = Outbox {
        letters: Vec::new(),
        draft: String::new(),
        next: 1,
    };
    (el, signals)
}

#[derive(Signals, Serialize, Deserialize)]
struct Mailing {
    email: String,
    status: String,
}

hypersig::routes! {
    fn mailing_actions(Router<()>);
    action SUBSCRIBE: Action<Mailing, String> = post "/subscribe" => no_answer;
}

impl Part for Mailing {
    fn render() -> El<Self> {
        div()
            .child(input().id(Id::new("email")).bind(Mailing::email()))
            .child(
                button()
                    .id(Id::new("subscribe"))
                    .on_click(SUBSCRIBE.call_with(Mailing::email())),
            )
            .child(span().id(Id::new("status")).bind_text(Mailing::status()))
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Storefront {
    footer: Mailing,
}

/// A page that embeds a part at its `footer` field.
fn storefront() -> (El<Storefront>, Storefront) {
    let el = main().child(hypersig::html::part(Storefront::footer()));
    let signals = Storefront {
        footer: Mailing {
            email: "ada@example.com".to_owned(),
            status: String::new(),
        },
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

hypersig::routes! {
    fn join_actions(Router<()>);
    action JOIN: Action<Join, Signup> = post "/join" => no_answer;
}

fn join() -> (El<Join>, Join) {
    let signup = Join::signup();
    let (email, password) = (Signup::email(), Signup::password());
    let el = signup
        .form(JOIN.call_with(signup.value()))
        .id(Id::new("join"))
        .child(
            input()
                .id(Id::new("email"))
                .bind(signup.field(email))
                .on_blur(signup.touch(email)),
        )
        .child(
            span()
                .id(Id::new("email-error"))
                .bind_text(signup.error(email)),
        )
        .child(
            input()
                .id(Id::new("password"))
                .bind(signup.field(password))
                .on_blur(signup.touch(password)),
        )
        .child(
            span()
                .id(Id::new("password-error"))
                .bind_text(signup.error(password)),
        )
        .child(
            span()
                .id(Id::new("dirty"))
                .show(signup.is_dirty())
                .text("unsaved"),
        )
        .child(
            span()
                .id(Id::new("valid"))
                .show(signup.is_valid())
                .text("ok"),
        )
        .child(button().id(Id::new("submit")).attr(attr::TYPE, "submit"))
        .child(
            button()
                .id(Id::new("reset"))
                .attr(attr::TYPE, "button")
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
        .child(ul().id(Id::new("rows")).child(each_keyed(
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
                .id(Id::new("field"))
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
            button().id(Id::new("run")).on_click(
                lines
                    .clear()
                    .then(lines.extend(3, |i| {
                        blank.with(Line::id(), next_id + i).with(
                            Line::label(),
                            words.nth(Expr::random_below(2)).unwrap_or("?"),
                        )
                    }))
                    .then(next_id.set(next_id + 3)),
            ),
        )
        .child(button().id(Id::new("update")).on_click(lines.update_where(
            |line| (line.index() % 2).equals(0),
            Line::label(),
            |line| line.at(Line::label()).concat(" !"),
        )))
        .child(button().id(Id::new("swap")).on_click(lines.swap(0, 2)))
        .child(
            button()
                .id(Id::new("sort"))
                .on_click(lines.sort_by(|line| line.at(Line::id()))),
        )
        .child(
            button()
                .id(Id::new("sort-desc"))
                .on_click(lines.sort_by_desc(|line| line.at(Line::id()))),
        )
        .child(
            button()
                .id(Id::new("stamp"))
                .on_click(Table::stamp().set(Expr::now())),
        )
        .child(ul().id(Id::new("lines")).child(each_keyed(
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

hypersig::node_ref!(const MENU: NodeRef<Roving>);

/// A tab list whose third tab is disabled and whose last tab takes the right
/// arrow for itself, and a vertical menu inside it whose items belong to the
/// menu rather than to the tab list.
fn roving() -> (El<Roving>, Roving) {
    let tab = Roving::tab();
    let taken = Roving::taken();
    let tab_button = |i: i32| {
        const TABS: [Id; 4] = [
            Id::new("tab0"),
            Id::new("tab1"),
            Id::new("tab2"),
            Id::new("tab3"),
        ];
        button()
            .id(TABS[usize::try_from(i).expect("a tab index")].clone())
            .bind_attr(attr::TABINDEX, if_else(tab.equals(i), "0", "-1"))
            .on(Event::Focus, tab.set(i))
    };
    let el = main().child(
        div()
            .id(Id::new("tabs"))
            .roving_focus(Orientation::Horizontal)
            .child(tab_button(0))
            .child(tab_button(1))
            .child(tab_button(2).attr(attr::DISABLED, ""))
            .child(tab_button(3).on_key(Key::ArrowRight, taken.set(taken + 1)))
            .child(
                div()
                    .id(Id::new("menu"))
                    .roving_focus(Orientation::Vertical)
                    .node_ref(MENU)
                    .on_key(Key::Tab, MENU.hide_popover())
                    .on_key(Key::Tab.shift(), MENU.hide_popover())
                    .on_key(Key::Enter, MENU.show_popover())
                    .children(
                        [Id::new("item0"), Id::new("item1")]
                            .map(|id| button().id(id).attr(attr::TABINDEX, "-1")),
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

hypersig::node_ref!(const OPTIONS: NodeRef<Combo>);

/// A text field that tracks the active option of a listbox whose second
/// option is hidden while the query is empty and whose third is disabled.
fn combo() -> (El<Combo>, Combo) {
    let query = Combo::query();
    let active = Combo::active();
    let picked = Combo::picked();
    let downs = Combo::downs();
    let option = |i: i32| {
        const OPTIONS: [Id; 4] = [Id::new("o0"), Id::new("o1"), Id::new("o2"), Id::new("o3")];
        div()
            .id(OPTIONS[usize::try_from(i).expect("an option index")].clone())
            .role(Role::Option)
            .on_click(picked.set(format!("o{i}")))
    };
    let el = main()
        .child(
            input()
                .id(Id::new("field"))
                .bind(query)
                .active_descendant(OPTIONS, active)
                .on_key(Key::ArrowDown, downs.set(downs + 1)),
        )
        .child(
            div()
                .role(Role::ListBox)
                .node_ref(OPTIONS)
                .child(option(0))
                .child(option(1).show(!query.is_empty()))
                .child(option(2).attr(attr::ARIA_DISABLED, "true"))
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
        .child(
            button()
                .id(Id::new("mark"))
                .on_click(Plot::marked().toggle()),
        )
        .child(span().id(Id::new("extent")).bind_text(extent))
        .child(
            span().id(Id::new("second")).bind_text(
                heights
                    .map(|bar| bar.at(Bar::id()))
                    .nth(1)
                    .unwrap_or(0)
                    .to_text(),
            ),
        )
        .child(
            button()
                .id(Id::new("collect"))
                .on_click(Plot::collected().append(heights.map(|bar| bar.at(Bar::id())))),
        )
        .child(
            span()
                .id(Id::new("collected"))
                .bind_text(Plot::collected().len().to_text()),
        )
        .child(
            el(hypersig::tag!("svg"))
                .id(Id::new("plot"))
                .attr(attr::VIEW_BOX, "0 0 100 100")
                .child(
                    el(hypersig::tag!("rect"))
                        .id(Id::new("frame"))
                        .attr(attr::WIDTH, "100")
                        .attr(attr::HEIGHT, "100")
                        .show(Plot::marked()),
                )
                .child(each_keyed(
                    Plot::bars(),
                    |bar| bar.at(Bar::id()),
                    |bar| {
                        el(hypersig::tag!("g"))
                            .child(
                                el(hypersig::tag!("rect"))
                                    .attr(attr::WIDTH, "5")
                                    .bind_attr(attr::HEIGHT, bar.at(Bar::height()).to_text()),
                            )
                            .child(each(bar.at(Bar::marks()), |mark| {
                                el(hypersig::tag!("line"))
                                    .bind_attr(attr::Y1, mark.value().to_text())
                            }))
                    },
                ))
                .child(when(
                    Plot::marked(),
                    el(hypersig::tag!("circle"))
                        .id(Id::new("dot"))
                        .attr(attr::R, "2"),
                )),
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
    posted: Timestamp,
    now: Timestamp,
    status: Status,
}

#[derive(Segment, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Status {
    Open,
    InReview,
    Closed,
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
            .id(Id::new("values"))
            .child(
                span()
                    .id(Id::new("zip-digits"))
                    .bind_text(yes_no(zip.all_chars(CharClass::AsciiDigit))),
            )
            .child(
                span()
                    .id(Id::new("code-digits"))
                    .bind_text(yes_no(code.all_chars(CharClass::AsciiDigit))),
            )
            .child(
                span()
                    .id(Id::new("code-upper"))
                    .bind_text(yes_no(code.any_char(CharClass::AsciiUppercase))),
            )
            .child(
                span()
                    .id(Id::new("code-space"))
                    .bind_text(yes_no(code.any_char(CharClass::AsciiWhitespace))),
            )
            .child(
                span()
                    .id(Id::new("items-en"))
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
                span().id(Id::new("items-ja")).bind_text(
                    count
                        .to_text()
                        .concat(count.plural(IntlLocale::Ja, |_| " 点")),
                ),
            )
            .child(
                span()
                    .id(Id::new("status"))
                    .bind_text(Values::status().label(|status| match status {
                        Status::Open => "Open",
                        Status::InReview => "In review",
                        Status::Closed => "Closed",
                    })),
            ),
    );
    let el = el.child(dates());
    let signals = Values {
        zip: "1500001".to_owned(),
        code: "Ab 12\u{ff11}".to_owned(),
        count: 1,
        status: Status::InReview,
        // 2026-01-05 15:04:05 UTC, and three minutes later.
        posted: Timestamp::from_millis(1_767_625_445_000),
        now: Timestamp::from_millis(1_767_625_625_000),
    };
    (el, signals)
}

/// Dates in fixed zones and relative times, which the server writes, and a
/// date in the browser's zone, which it leaves out.
fn dates() -> El<Values> {
    let posted = Values::posted();
    let now = Values::now();
    let date = |id: &'static str, format: DateFormat| {
        span()
            .id(Id::new(id))
            .bind_text(posted.format_date(&format))
    };
    let ago = |id: &'static str, locale: IntlLocale| {
        span()
            .id(Id::new(id))
            .bind_text(posted.relative_to(now, locale))
    };
    div()
        .child(
            div()
                .id(Id::new("dates"))
                .child(date(
                    "date-en-full",
                    DateFormat::date_time(IntlLocale::En, DateStyle::Full, TimeStyle::Medium),
                ))
                .child(date(
                    "date-en-long",
                    DateFormat::date(IntlLocale::En, DateStyle::Long),
                ))
                .child(date(
                    "date-en-medium",
                    DateFormat::date_time(IntlLocale::En, DateStyle::Medium, TimeStyle::Short),
                ))
                .child(date(
                    "date-en-short",
                    DateFormat::date_time(IntlLocale::En, DateStyle::Short, TimeStyle::Short),
                ))
                .child(date(
                    "time-en",
                    DateFormat::time(IntlLocale::En, TimeStyle::Medium),
                ))
                .child(date(
                    "date-ja-full",
                    DateFormat::date_time(IntlLocale::Ja, DateStyle::Full, TimeStyle::Medium),
                ))
                .child(date(
                    "date-ja-long",
                    DateFormat::date(IntlLocale::Ja, DateStyle::Long),
                ))
                .child(date(
                    "date-ja-short",
                    DateFormat::date_time(IntlLocale::Ja, DateStyle::Short, TimeStyle::Short),
                ))
                .child(date(
                    "date-tokyo",
                    DateFormat::date_time(IntlLocale::Ja, DateStyle::Medium, TimeStyle::Short)
                        .zone(TimeZone::Offset(540)),
                ))
                .child(ago("ago-en", IntlLocale::En))
                .child(ago("ago-ja", IntlLocale::Ja)),
        )
        .child(date(
            "date-local",
            DateFormat::date_time(IntlLocale::En, DateStyle::Medium, TimeStyle::Short)
                .zone(TimeZone::Browser),
        ))
}

#[derive(Signals, Serialize, Deserialize)]
struct Sums {
    a: i32,
    b: i32,
    #[signals(computed)]
    sum: i32,
    #[signals(computed)]
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
            .child(span().id(Id::new("sum")).bind_text(sum.to_text()))
            .child(span().id(Id::new("doubled")).bind_text(doubled.to_text())),
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
            .id(Id::new("section"))
            .attr(attr::TABINDEX, "-1")
            .on_key(
                Key::Char('s'),
                Shortcuts::saves().set(Shortcuts::saves() + 1),
            )
            .on_key(
                Key::Char('s').ctrl().shift(),
                Shortcuts::saves().set(Shortcuts::saves() + 100),
            )
            .child(input().id(Id::new("field")).bind(Shortcuts::text()))
            .child(textarea().id(Id::new("notes")).on_key(
                Key::Escape,
                Shortcuts::escapes().set(Shortcuts::escapes() + 1),
            ))
            .child(
                input()
                    .id(Id::new("own"))
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
                .id(Id::new("box"))
                .css(Style::new().on(Media::Md, Style::new().width(pct(50))))
                .css_when(Style::new().color(Color::CRIMSON), Styled::on())
                .on_click(Styled::on().toggle()),
        )
        .child(div().id(Id::new("slot")));
    (el, Styled { on: false })
}

/// Fills the slot with rules in two buckets the page does not have, one that
/// sorts before the page's breakpoint bucket and one between, and with an
/// `@keyframes` rule, which sorts after every layer.
struct Panel;

impl Fragment for Panel {
    type Signals = Styled;
    const ID: Id = Id::new("slot");

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

#[expect(
    dead_code,
    reason = "the fixtures call these actions from the browser, which no server answers"
)]
async fn no_answer<S: hypersig::Signals, A>((): (), _: A) -> Patches<S> {
    Patches::new()
}

#[expect(
    dead_code,
    reason = "the fixtures open this stream from the browser, which no server answers"
)]
fn no_stream<S, A>((): (), _: A) -> futures::stream::Empty<Patches<S>> {
    futures::stream::empty()
}

#[expect(
    dead_code,
    reason = "the fixtures send this upload from the browser, which no server answers"
)]
async fn no_upload<S: hypersig::Signals, A>((): (), _: hypersig::Uploaded<A>) -> Patches<S> {
    Patches::new()
}

/// Links whose parameters need percent-encoding, so the server's encoding
/// and the browser's `encodeURIComponent` are compared on the same values.
fn links() -> (El<Roster>, Roster) {
    let el = main()
        .child(a().id(Id::new("home")).href(HOME.link()))
        .child(
            ul().id(Id::new("members"))
                .child(each(Roster::members(), |member| {
                    let params = || (member.at(Member::team()), member.at(Member::id()));
                    let search = Expr::lit(&filter()).with(Filter::page(), member.at(Member::id()));
                    li().child(a().href(MEMBER.link_with(params())))
                        .child(button().on_click(MEMBER.link_with(params()).navigate()))
                        .child(a().href(TEAM_SEARCH.link_with((member.at(Member::team()), search))))
                })),
        )
        .child(
            a().id(Id::new("search-fixed"))
                .href(FIND.link_with(&filter())),
        )
        .child(
            a().id(Id::new("search-bound"))
                .href(FIND.link_with(Roster::filter())),
        )
        .child(
            a().id(Id::new("file-fixed"))
                .href(FILE.link_with(&Rest::new("docs/a b/ü?.md"))),
        )
        .child(
            a().id(Id::new("file-bound"))
                .href(FILE.link_with(Roster::file())),
        )
        .child(
            button()
                .id(Id::new("push"))
                .on_click(FIND.link_with(Roster::filter()).push_url()),
        )
        .child(
            button()
                .id(Id::new("replace"))
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
const RUNTIME_SRC: hypersig::Asset = hypersig::Asset::new("/_hypersig/runtime.js");

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
                .id(Id::new("older"))
                .on_click(Feed::notes().prepend(Feed::older()))
                .text("Older"),
        )
        .child(ul().id(Id::new("feed")).child(each_keyed(
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

#[derive(Signals, Serialize, Deserialize)]
struct Point {
    lat: f64,
    lng: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Pin {
    name: String,
    picked: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Widgets {
    stars: i32,
    center: Point,
    pins: Vec<Pin>,
    site: SafeUrl,
}

const RATE: CustomEvent<i32> = CustomEvent::new("rate");
const MOVED: CustomEvent<Point> = CustomEvent::new("map-moved");
const PICK: CustomEvent<()> = CustomEvent::new("pick");
const LINKED: CustomEvent<SafeUrl> = CustomEvent::new("linked");
hypersig::node_ref!(const MAP: NodeRef<Widgets>);

/// A fragment holding a map that a script draws into, which a patch replaces around it.
struct Atlas(&'static str);

impl Fragment for Atlas {
    type Signals = Widgets;
    const ID: Id = Id::new("atlas");

    fn render(&self) -> El<Widgets> {
        div().child(p().text(self.0)).child(
            div()
                .foreign(MAP)
                .bind_attr(hypersig::attr!("data-stars"), Widgets::stars().to_text()),
        )
    }
}

fn widgets() -> (El<Widgets>, Widgets) {
    let center = Widgets::center();
    let el = main()
        .child(
            el(hypersig::tag!("x-rating"))
                .id(Id::new("rating"))
                .on_custom(RATE, |stars| Widgets::stars().set(stars))
                .on_custom(RATE, |stars| Widgets::stars().set(Widgets::stars() + stars)),
        )
        .child(
            el(hypersig::tag!("x-map"))
                .id(Id::new("map"))
                .on_custom(MOVED, |point| {
                    center
                        .at(Point::lat())
                        .set(point.at(Point::lat()))
                        .then(center.at(Point::lng()).set(point.at(Point::lng())))
                }),
        )
        .child(Atlas("Tokyo").render().id(Atlas::ID))
        .child(
            a().id(Id::new("site"))
                .on_custom(LINKED, |url| Widgets::site().set(url))
                .href(Widgets::site()),
        )
        .child(ul().id(Id::new("pins")).child(each(Widgets::pins(), |pin| {
            li().child(
                el(hypersig::tag!("x-pin"))
                    .on_custom(PICK, move |_| pin.at(Pin::picked()).set(true))
                    .bind_text(pin.at(Pin::name())),
            )
        })));
    let signals = Widgets {
        stars: 0,
        center: Point { lat: 0.0, lng: 0.0 },
        pins: vec![
            Pin {
                name: "a".to_owned(),
                picked: false,
            },
            Pin {
                name: "b".to_owned(),
                picked: false,
            },
        ],
        site: SafeUrl::new("https://example.com"),
    };
    (el, signals)
}

#[test]
fn the_widget_fixtures_match_the_builder() {
    let (el, signals) = widgets();
    check("widgets", &el, &signals);
    write_or_compare(
        "widgets_patch",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "stream": Patches::<Widgets>::new().element(&Atlas("Kyoto")).to_event_stream()
            }))
            .expect("fixture JSON")
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
    let (el, signals) = outbox();
    check("outbox", &el, &signals);
    let (el, signals) = storefront();
    check("storefront", &el, &signals);
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
hypersig::routes! {
    fn upload_actions(Router<()>);
    upload SEND: Upload<Uploads, String> = "/pictures" => no_upload;
}
// The same name as the file input, which the browser looks up apart from it.
hypersig::node_ref!(const PICTURES_NOTE: NodeRef<Uploads>);

fn uploads() -> (El<Uploads>, Uploads) {
    let el = main()
        .child(
            p().id(Id::new("note"))
                .attr(attr::TABINDEX, "-1")
                .node_ref(PICTURES_NOTE)
                .text("Pick pictures"),
        )
        .child(
            button()
                .id(Id::new("focus"))
                .on_click(PICTURES_NOTE.focus()),
        )
        .child(
            PICTURES
                .input()
                .id(Id::new("pictures"))
                .attr(attr::MULTIPLE, ""),
        )
        .child(input().id(Id::new("caption")).bind(Uploads::caption()))
        .child(
            button().id(Id::new("send")).on_click(
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
