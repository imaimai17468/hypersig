//! Writes the HTML that `runtime/test/*.test.ts` runs against, so the browser
//! tests exercise exactly what the Rust builder emits.
//!
//! The test fails when a fixture on disk differs from what the builder writes
//! now. Run it with `HYPERSIG_FIXTURES=overwrite` to rewrite them.

use std::path::PathBuf;
use std::time::Duration;

use hypersig::html::{
    El, Event, Key, Orientation, a, button, div, each, each_keyed, input, label, li, main, select,
    span, ul, when,
};
use hypersig::style::value::{Color, pct, px};
use hypersig::style::{Keyframes, Media, Property, PseudoClass, Style};
use hypersig::{
    Action, Expr, Fragment, NodeRef, Page, Patches, Rest, Route, Search, Signals, Stmt, if_else,
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
                .bind_style(Property::Width, quantity.to_text().concat("rem"))
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
    fired: i32,
}

const SEARCH: Action<Timers> = Action::post("/search");

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
        .child(a().id("file-bound").href(FILE.link_with(Roster::file())));
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
            ..filter()
        },
        file: Rest::new("docs/a b/ü?.md"),
    };
    (el, signals)
}

/// Search parameters whose fields are declared out of name order, with
/// values that need percent-encoding and a `None` that is left out.
fn filter() -> Filter {
    Filter {
        q: "a b&c=d#é".to_owned(),
        page: 2,
        open: true,
        tag: None,
    }
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
    let (el, signals) = table();
    check("table", &el, &signals);
    let (el, signals) = roving();
    check("roving", &el, &signals);
    let (el, signals) = combo();
    check("combo", &el, &signals);
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
    let document = computed_page();
    write_or_compare(
        "computed_page",
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({ "document": document })).expect("fixture JSON")
        ),
    );
}
