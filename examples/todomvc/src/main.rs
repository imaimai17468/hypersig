//! [TodoMVC](https://todomvc.com/), with every interaction handled in the browser.
//!
//! The list lives in a `Vec` signal. The server only renders the first page
//! and stores the list whenever it changes, so a reload keeps it.
//!
//! Run with `cargo run -p example-todomvc` and open <http://localhost:3002>.

use std::sync::{Arc, Mutex};

use axum::Router;
use hypersig::html::{
    El, Id, Key, a, attr, button, each_keyed, footer, h1, header, input, label, li, section, span,
    strong, ul, when,
};
use hypersig::{Action, Expr, Page, Patches, Route, RouterExt, SafeUrl, Signals, Stmt, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, PartialEq)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Signals, Serialize, Deserialize)]
struct TodoMvc {
    todos: Vec<Todo>,
    next_id: i32,
    draft: String,
    filter: Filter,
    editing: Option<i32>,
    edit_text: String,
    left: i32,
    completed: i32,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

const SAVE: Action<TodoMvc, Vec<Todo>> = Action::post("/save");
hypersig::node_ref!(const EDIT: NodeRef<TodoMvc>);
const TOGGLE_ALL: Id = Id::new("toggle-all");

#[derive(Clone, Default)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

fn new_todo() -> El<TodoMvc> {
    let todos = TodoMvc::todos();
    let draft = TodoMvc::draft();
    let next_id = TodoMvc::next_id();
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
    input()
        .css(styles::new_todo())
        .attr(attr::PLACEHOLDER, "What needs to be done?")
        .attr(attr::AUTOFOCUS, "")
        .bind(draft)
        .on_key(Key::Enter, add)
}

fn todo_list() -> El<TodoMvc> {
    let todos = TodoMvc::todos();
    let filter = TodoMvc::filter();
    let editing = TodoMvc::editing();
    let edit_text = TodoMvc::edit_text();
    ul().id(Id::new("todo-list"))
        .css(styles::list())
        .child(each_keyed(
            todos,
            |todo| todo.at(Todo::id()),
            |todo| {
                let id = todo.at(Todo::id());
                let title = todo.at(Todo::title());
                let done = todo.at(Todo::done());
                let is_editing = editing.equals(id.some());
                let visible = filter
                    .equals(&Filter::All)
                    .or(filter.equals(&Filter::Active).and(!done))
                    .or(filter.equals(&Filter::Completed).and(done));
                let commit = Stmt::when(
                    &is_editing,
                    Stmt::if_else(
                        edit_text.trim().is_empty(),
                        todo.remove(),
                        title.set(edit_text.trim()),
                    )
                    .then(editing.set(&None)),
                );
                li().show(visible)
                    .css(styles::item())
                    .child(
                        input()
                            .css(styles::toggle())
                            .css_when(styles::hidden(), &is_editing)
                            .attr(attr::TYPE, "checkbox")
                            .attr(attr::ARIA_LABEL, "Toggle")
                            .bind(done),
                    )
                    .child(
                        label()
                            .css(styles::title())
                            .css_when(styles::completed_title(), done)
                            .css_when(styles::hidden(), &is_editing)
                            .bind_text(title)
                            .on_double_click(
                                edit_text
                                    .set(title)
                                    .then(editing.set(id.some()))
                                    .then(EDIT.focus()),
                            ),
                    )
                    .child(
                        button()
                            .css(styles::destroy())
                            .css_when(styles::hidden(), is_editing)
                            .attr(attr::ARIA_LABEL, "Delete")
                            .on_click(todo.remove()),
                    )
                    .child(when(
                        editing.equals(id.some()),
                        input()
                            .css(styles::edit())
                            .attr(attr::ARIA_LABEL, "Edit")
                            .node_ref(EDIT)
                            .bind(edit_text)
                            .on_key(Key::Enter, commit.clone())
                            .on_key(Key::Escape, editing.set(&None))
                            .on_blur(commit),
                    ))
            },
        ))
}

fn filter_link(filter: Filter, text: &str) -> El<TodoMvc> {
    li().child(
        a().href(SafeUrl::new("#"))
            .css(styles::filter_link())
            .css_when(styles::selected(), TodoMvc::filter().equals(&filter))
            .on_click(TodoMvc::filter().set(&filter))
            .text(text),
    )
}

fn page(todos: Vec<Todo>) -> Page<TodoMvc> {
    let list = TodoMvc::todos();
    let left = TodoMvc::left();
    let completed = TodoMvc::completed();
    let next_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    let signals = TodoMvc {
        todos,
        next_id,
        draft: String::new(),
        filter: Filter::All,
        editing: None,
        edit_text: String::new(),
        left: 0,
        completed: 0,
    };
    Page::new("hypersig TodoMVC", signals)
        .global(styles::global())
        .computed(left, list.count(|todo| !todo.at(Todo::done())))
        .computed(completed, list.count(|todo| todo.at(Todo::done())))
        .child(
            section()
                .css(styles::app())
                .on_signal_change(list, SAVE.call_with(list))
                .child(
                    header()
                        .child(h1().css(styles::heading()).text("todos"))
                        .child(new_todo()),
                )
                .child(
                    section()
                        .id(Id::new("main"))
                        .css(styles::main())
                        .show(!list.is_empty())
                        .child(
                            input()
                                .id(TOGGLE_ALL)
                                .css(styles::toggle_all())
                                .attr(attr::TYPE, "checkbox")
                                .bind_checked(left.equals(0))
                                .on_change(list.set_all(Todo::done(), left.gt(0))),
                        )
                        .child(
                            label()
                                .css(styles::toggle_all_label())
                                .label_for(&TOGGLE_ALL)
                                .text("Mark all as complete"),
                        )
                        .child(todo_list()),
                )
                .child(
                    footer()
                        .css(styles::footer())
                        .show(!list.is_empty())
                        .child(
                            span()
                                .id(Id::new("todo-count"))
                                .child(strong().bind_text(left.to_text()))
                                .child(span().bind_text(if_else(
                                    left.equals(1),
                                    " item left",
                                    " items left",
                                ))),
                        )
                        .child(
                            ul().id(Id::new("filters"))
                                .css(styles::filters())
                                .child(filter_link(Filter::All, "All"))
                                .child(filter_link(Filter::Active, "Active"))
                                .child(filter_link(Filter::Completed, "Completed")),
                        )
                        .child(
                            button()
                                .id(Id::new("clear-completed"))
                                .css(styles::plain_button())
                                .show(completed.gt(0))
                                .on_click(list.retain(|todo| !todo.at(Todo::done())))
                                .text("Clear completed"),
                        ),
                ),
        )
}

async fn index(state: AppState, (): ()) -> Page<TodoMvc> {
    let todos = state.todos.lock().expect("todo lock poisoned").clone();
    page(todos)
}

async fn save(state: AppState, todos: Vec<Todo>) -> Patches<TodoMvc> {
    *state.todos.lock().expect("todo lock poisoned") = todos;
    Patches::new()
}

/// The look of the app, one function per kind of element.
mod styles {
    use hypersig::style::value::{
        AlignItems, Border, BoxSizing, Color, Content, CssWideKeyword, Cursor, Display, FontWeight,
        JustifyContent, ListStyle, MarginInline, Outline, Position, TextAlign, TextDecoration,
        WordBreak, deg, pct, px, quoted, raw,
    };
    use hypersig::style::{
        Condition, Element, Global, Property, PseudoClass, PseudoElement, Selector, Style,
    };

    const GRAY: u32 = 0x0094_9494;

    pub fn global() -> Global {
        Global::new()
            .rule(
                Selector::Everything,
                Style::new().box_sizing(BoxSizing::BorderBox),
            )
            .rule(
                Element::Body,
                Style::new()
                    .font_size(px(14))
                    .font_family(raw("'Helvetica Neue', Helvetica, Arial, sans-serif"))
                    .background_color(Color::hex(0x00f5_f5f5))
                    .color(Color::hex(0x0011_1111))
                    .margin(px(0)),
            )
    }

    pub fn app() -> Style {
        Style::new()
            .background_color(Color::WHITE)
            .max_width(px(550))
            .margin_top(px(130))
            .margin_inline(MarginInline::Auto)
            .margin_bottom(px(40))
            .position(Position::Relative)
            .box_shadow(raw("0 2px 4px rgba(0,0,0,.2), 0 25px 50px rgba(0,0,0,.1)"))
    }

    pub fn heading() -> Style {
        Style::new()
            .position(Position::Absolute)
            .top(px(-110))
            .width(pct(100))
            .font_size(px(80))
            .font_weight(FontWeight::Number(200.0))
            .text_align(TextAlign::Center)
            .color(Color::hex(0x00b8_3f45))
            .margin(px(0))
    }

    fn text_field() -> Style {
        Style::new()
            .width(pct(100))
            .font_size(px(24))
            .border(Border::None)
            .outline(Outline::None)
    }

    pub fn new_todo() -> Style {
        text_field()
            .padding_block(px(16))
            .padding_right(px(16))
            .padding_left(px(60))
            .box_shadow(raw("inset 0 -2px 1px rgba(0,0,0,.03)"))
    }

    pub fn edit() -> Style {
        text_field()
            .margin_left(px(43))
            .padding_block(px(12))
            .padding_inline(px(16))
            .border(raw("1px solid #999"))
            .box_shadow(raw("inset 0 -1px 5px rgba(0,0,0,.2)"))
    }

    pub fn main() -> Style {
        Style::new()
            .position(Position::Relative)
            .border_top(raw("1px solid #e6e6e6"))
    }

    /// The checkbox is invisible; the label after it draws the arrow, which
    /// darkens while the box is checked.
    pub fn toggle_all() -> Style {
        Style::new().position(Position::Absolute).opacity(0).peer()
    }

    pub fn toggle_all_label() -> Style {
        let arrow = Style::new()
            .content(quoted("❯"))
            .font_size(px(22))
            .color(Color::hex(GRAY))
            .transform(raw(format!("rotate({})", deg(90))));
        Style::new()
            .position(Position::Absolute)
            .top(px(-52))
            .left(px(0))
            .width(px(45))
            .height(px(65))
            .font_size(px(0))
            .display(Display::Flex)
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .pseudo(PseudoElement::Before, arrow)
            .on(
                Condition::Peer(PseudoClass::Checked),
                Style::new().pseudo(
                    PseudoElement::Before,
                    Style::new().color(Color::hex(0x0048_4848)),
                ),
            )
    }

    pub fn list() -> Style {
        Style::new()
            .margin(px(0))
            .padding(px(0))
            .list_style(ListStyle::None)
    }

    /// The item is the group its delete button watches for hover.
    pub fn item() -> Style {
        Style::new()
            .position(Position::Relative)
            .font_size(px(24))
            .border_bottom(raw("1px solid #ededed"))
            .display(Display::Flex)
            .align_items(AlignItems::Center)
            .group()
    }

    pub fn toggle() -> Style {
        Style::new()
            .width(px(40))
            .height(px(40))
            .margin_block(px(0))
            .margin_inline(px(10))
    }

    pub fn title() -> Style {
        Style::new()
            .flex(1)
            .padding(px(15))
            .word_break(WordBreak::BreakAll)
            .transition(raw("color .2s"))
    }

    pub fn completed_title() -> Style {
        Style::new()
            .color(Color::hex(GRAY))
            .text_decoration(TextDecoration::LineThrough)
    }

    pub fn hidden() -> Style {
        Style::new().display(Display::None)
    }

    pub fn plain_button() -> Style {
        Style::new()
            .border(Border::None)
            .background_color(Color::TRANSPARENT)
            .cursor(Cursor::Pointer)
            .keyword(Property::Color, CssWideKeyword::Inherit)
    }

    /// Hidden until the pointer is over its item.
    pub fn destroy() -> Style {
        plain_button()
            .display(Display::None)
            .font_size(px(30))
            .color(Color::hex(GRAY))
            .margin_right(px(10))
            .pseudo(
                PseudoElement::After,
                Style::new().content(Content::Quoted(quoted("×"))),
            )
            .on(
                Condition::Group(PseudoClass::Hover),
                Style::new().display(Display::Block),
            )
    }

    pub fn footer() -> Style {
        Style::new()
            .padding_block(px(10))
            .padding_inline(px(15))
            .display(Display::Flex)
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::SpaceBetween)
            .border_top(raw("1px solid #e6e6e6"))
            .color(Color::hex(0x0011_1111))
    }

    pub fn filters() -> Style {
        list().display(Display::Flex).gap(px(4))
    }

    pub fn filter_link() -> Style {
        Style::new()
            .keyword(Property::Color, CssWideKeyword::Inherit)
            .text_decoration(TextDecoration::None)
            .padding_block(px(3))
            .padding_inline(px(7))
            .border(raw("1px solid transparent"))
            .border_radius(px(3))
    }

    pub fn selected() -> Style {
        Style::new().border_color(Color::hex(0x00ce_4646))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(SAVE, save)
        .with_state(AppState::default());
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3002".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
