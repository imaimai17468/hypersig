//! A shared todo list. Every open tab receives changes over a long-lived stream,
//! and each todo has a page of its own at `/todos/{id}`.
//!
//! Run with `cargo run -p example-todo` and open <http://localhost:3001> in two tabs.

use std::sync::{Arc, Mutex};

use axum::Router;
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{El, a, button, div, form, h1, input, li, p, span, ul};
use hypersig::{Action, Fragment, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Signals, Serialize, Deserialize)]
struct TodoPage {
    draft: String,
    remaining: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct TodoDetail {
    done: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
struct TodoId(u64);

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    const TODO: Route<u64> = "/todos/{id}" => todo;
}

const ADD: Action<TodoPage, String> = Action::post("/todos");
const TOGGLE: Action<TodoPage, TodoId> = Action::post("/todos/toggle");
const REMOVE: Action<TodoPage, TodoId> = Action::post("/todos/remove");
const UPDATES: Action<TodoPage> = Action::get("/todos/updates");

#[derive(Clone)]
struct Todo {
    id: TodoId,
    title: String,
    done: bool,
}

#[derive(Default)]
struct Todos {
    next_id: u64,
    items: Vec<Todo>,
}

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Todos>>,
    changed: broadcast::Sender<()>,
}

impl AppState {
    fn snapshot(&self) -> Vec<Todo> {
        self.todos.lock().expect("todo lock poisoned").items.clone()
    }

    fn update(&self, change: impl FnOnce(&mut Todos)) {
        change(&mut self.todos.lock().expect("todo lock poisoned"));
        let _ = self.changed.send(());
    }
}

struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    fn remaining(&self) -> i32 {
        let open = self.items.iter().filter(|todo| !todo.done).count();
        i32::try_from(open).unwrap_or(i32::MAX)
    }

    fn patches(&self) -> Patches<TodoPage> {
        Patches::new()
            .element(self)
            .signals(TodoPage::remaining().patch(self.remaining()))
    }
}

impl Fragment for TodoList {
    type Signals = TodoPage;
    const ID: &'static str = "todo-list";

    fn render(&self) -> El<TodoPage> {
        ul().css(styles::list())
            .children(self.items.iter().map(|todo| {
                let title = span().css(styles::title(todo.done)).child(
                    a().css(styles::link())
                        .href(TODO.link_with(&todo.id.0))
                        .text(todo.title.as_str()),
                );
                li().id(format!("todo-{}", todo.id.0))
                    .css(styles::item())
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr("aria-label", "toggle")
                            .on_click(TOGGLE.call_with(&todo.id))
                            .text(if todo.done { "✓" } else { "○" }),
                    )
                    .child(title)
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr("aria-label", "remove")
                            .on_click(REMOVE.call_with(&todo.id))
                            .text("×"),
                    )
            }))
    }
}

mod styles {
    use hypersig::style::Style;
    use hypersig::style::value::{
        AlignItems, BorderBottomStyle, BorderStyle, Color, Cursor, Display, FontFamily, ListStyle,
        MarginInline, TextDecorationLine, px, rem,
    };

    pub fn page() -> Style {
        Style::new()
            .font_family(FontFamily::SystemUi)
            .max_width(rem(32))
            .margin_block(rem(3))
            .margin_inline(MarginInline::Auto)
            .padding_inline(rem(1))
    }

    pub fn form() -> Style {
        Style::new().display(Display::Flex).gap(rem(0.5))
    }

    pub fn input() -> Style {
        Style::new()
            .flex_grow(1)
            .font_size(rem(1))
            .padding(rem(0.4))
    }

    pub fn button() -> Style {
        Style::new().font_size(rem(1)).cursor(Cursor::Pointer)
    }

    pub fn list() -> Style {
        Style::new().list_style(ListStyle::None).padding(px(0))
    }

    pub fn item() -> Style {
        Style::new()
            .display(Display::Flex)
            .gap(rem(0.5))
            .align_items(AlignItems::Center)
            .padding_block(rem(0.3))
            .border_bottom_width(px(1))
            .border_bottom_style(BorderBottomStyle::Solid)
            .border_bottom_color(Color::hex(0x00ee_eeee))
    }

    pub fn title(done: bool) -> Style {
        let title = Style::new().flex_grow(1);
        if done {
            title
                .text_decoration_line(TextDecorationLine::LineThrough)
                .color(Color::hex(0x0088_8888))
        } else {
            title
        }
    }

    pub fn icon_button() -> Style {
        button()
            .border_style(BorderStyle::None)
            .background_color(Color::TRANSPARENT)
    }

    pub fn link() -> Style {
        Style::new().color(Color::CURRENT_COLOR)
    }

    pub fn summary() -> Style {
        Style::new().color(Color::hex(0x0055_5555))
    }
}

async fn index(state: AppState, (): ()) -> Page<TodoPage> {
    let list = TodoList {
        items: state.snapshot(),
    };
    let draft = TodoPage::draft();
    let remaining = TodoPage::remaining();
    Page::new(
        "hypersig todo",
        TodoPage {
            draft: String::new(),
            remaining: list.remaining(),
        },
    )
    .child(
        div()
            .css(styles::page())
            .on_load(UPDATES.call())
            .child(h1().text("Todo"))
            .child(
                form()
                    .css(styles::form())
                    .on_submit(ADD.call_with(draft))
                    .child(
                        input()
                            .css(styles::input())
                            .attr("placeholder", "What needs doing?")
                            .bind(draft),
                    )
                    .child(
                        button()
                            .css(styles::button())
                            .show(!draft.is_empty())
                            .text("Add"),
                    ),
            )
            .child(hypersig::fragment(&list))
            .child(
                p().id("summary-left")
                    .css(styles::summary())
                    .show(remaining.gt(0))
                    .bind_text(remaining.to_text().concat(" left")),
            )
            .child(
                p().id("summary-done")
                    .css(styles::summary())
                    .show(remaining.equals(0))
                    .text("All done."),
            ),
    )
}

async fn todo(state: AppState, id: u64) -> Option<Page<TodoDetail>> {
    let todo = state
        .snapshot()
        .into_iter()
        .find(|todo| todo.id == TodoId(id))?;
    let done = TodoDetail::done();
    Some(
        Page::new(todo.title.clone(), TodoDetail { done: todo.done }).child(
            div()
                .css(styles::page())
                .child(h1().text(todo.title))
                .child(p().id("status").show(done).text("Done."))
                .child(p().id("status-open").show(!done).text("Still to do."))
                .child(a().href(INDEX.link()).text("All todos")),
        ),
    )
}

async fn add(state: AppState, draft: String) -> Patches<TodoPage> {
    let title = draft.trim().to_owned();
    if !title.is_empty() {
        state.update(|todos| {
            todos.next_id += 1;
            let id = TodoId(todos.next_id);
            todos.items.push(Todo {
                id,
                title,
                done: false,
            });
        });
    }
    TodoPage::draft().patch(String::new()).into()
}

async fn toggle(state: AppState, id: TodoId) -> Patches<TodoPage> {
    state.update(|todos| {
        if let Some(todo) = todos.items.iter_mut().find(|todo| todo.id == id) {
            todo.done = !todo.done;
        }
    });
    Patches::new()
}

async fn remove(state: AppState, id: TodoId) -> Patches<TodoPage> {
    state.update(|todos| todos.items.retain(|todo| todo.id != id));
    Patches::new()
}

fn updates(state: AppState, (): ()) -> impl Stream<Item = Patches<TodoPage>> {
    let changes = BroadcastStream::new(state.changed.subscribe());
    let current = TodoList {
        items: state.snapshot(),
    }
    .patches();
    stream::once(async { current }).chain(changes.map(move |_| {
        TodoList {
            items: state.snapshot(),
        }
        .patches()
    }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (changed, _) = broadcast::channel(16);
    let state = AppState {
        todos: Arc::default(),
        changed,
    };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(ADD, add)
        .action(TOGGLE, toggle)
        .action(REMOVE, remove)
        .stream(UPDATES, updates)
        .with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3001".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
