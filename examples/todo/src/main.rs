//! A shared todo list. Every open tab receives changes over a long-lived stream,
//! and each todo has a page of its own at `/todos/{id}`.
//!
//! Run with `cargo run -p example-todo` and open <http://localhost:3001> in two tabs.
//!
//! The list and the channel that tells the streams of a change live in this
//! process's memory, so a second process serves a list of its own.
//! `examples/postgres` keeps the list in PostgreSQL and tells the streams of
//! every process.

use std::sync::{Arc, Mutex};

use axum::Router;
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{El, Id, a, attr, button, div, each_keyed, form, h1, input, li, p, span, ul};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals, StreamAction, if_else};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Signals, Serialize, Deserialize)]
struct TodoPage {
    draft: String,
    todos: Vec<Todo>,
}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Todo {
    id: u64,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct TodoDetail {
    done: bool,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    const TODO: Route<u64> = "/todos/{id}" => todo;
}

const ADD: Action<TodoPage, String> = Action::post("/todos");
const TOGGLE: Action<TodoPage, u64> = Action::post("/todos/toggle");
const REMOVE: Action<TodoPage, u64> = Action::post("/todos/remove");
const UPDATES: StreamAction<TodoPage> = StreamAction::new("/todos/updates");

#[derive(Default)]
struct Todos {
    next_id: u64,
    items: Vec<Todo>,
}

/// One change to the list, which each open stream turns into a patch of the
/// one todo it touched.
#[derive(Clone)]
enum Change {
    Put(Todo),
    Removed(u64),
}

impl Change {
    fn patches(&self) -> Patches<TodoPage> {
        let todos = TodoPage::todos();
        match self {
            Change::Put(todo) => todos
                .patch_upsert(std::slice::from_ref(todo), |todo| todo.at(Todo::id()))
                .into(),
            Change::Removed(id) => todos
                .patch_remove(&[*id], |todo| todo.at(Todo::id()))
                .into(),
        }
    }
}

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Todos>>,
    changed: broadcast::Sender<Change>,
}

impl AppState {
    fn snapshot(&self) -> Vec<Todo> {
        self.todos.lock().expect("todo lock poisoned").items.clone()
    }

    /// Applies `change` to the list and sends each stream the change it returns.
    fn update(&self, change: impl FnOnce(&mut Todos) -> Option<Change>) {
        let change = change(&mut self.todos.lock().expect("todo lock poisoned"));
        if let Some(change) = change {
            let _ = self.changed.send(change);
        }
    }
}

fn todo_list() -> El<TodoPage> {
    ul().id(Id::new("todo-list"))
        .css(styles::list())
        .child(each_keyed(
            TodoPage::todos(),
            |todo| todo.at(Todo::id()),
            |todo| {
                let id = todo.at(Todo::id());
                let done = todo.at(Todo::done());
                li().css(styles::item())
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr(attr::ARIA_LABEL, "toggle")
                            .on_click(TOGGLE.call_with(id))
                            .bind_text(if_else(done, "✓", "○")),
                    )
                    .child(
                        span()
                            .css(styles::title())
                            .css_when(styles::title_done(), done)
                            .child(
                                a().css(styles::link())
                                    .href(TODO.link_with(id))
                                    .bind_text(todo.at(Todo::title())),
                            ),
                    )
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr(attr::ARIA_LABEL, "remove")
                            .on_click(REMOVE.call_with(id))
                            .text("×"),
                    )
            },
        ))
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

    pub fn title() -> Style {
        Style::new().flex_grow(1)
    }

    pub fn title_done() -> Style {
        Style::new()
            .text_decoration_line(TextDecorationLine::LineThrough)
            .color(Color::hex(0x0088_8888))
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
    let draft = TodoPage::draft();
    let remaining = TodoPage::todos().count(|todo| !todo.at(Todo::done()));
    Page::new(
        "hypersig todo",
        TodoPage {
            draft: String::new(),
            todos: state.snapshot(),
        },
    )
    .child(
        div()
            .css(styles::page())
            .on_load(UPDATES.call().reconnect())
            .child(h1().text("Todo"))
            .child(
                form()
                    .css(styles::form())
                    .on_submit(ADD.call_with(draft))
                    .child(
                        input()
                            .css(styles::input())
                            .attr(attr::PLACEHOLDER, "What needs doing?")
                            .bind(draft),
                    )
                    .child(
                        button()
                            .css(styles::button())
                            .show(!draft.is_empty())
                            .text("Add"),
                    ),
            )
            .child(todo_list())
            .child(
                p().id(Id::new("summary-left"))
                    .css(styles::summary())
                    .show(remaining.clone().gt(0))
                    .bind_text(remaining.clone().to_text().concat(" left")),
            )
            .child(
                p().id(Id::new("summary-done"))
                    .css(styles::summary())
                    .show(remaining.equals(0))
                    .text("All done."),
            ),
    )
}

async fn todo(state: AppState, id: u64) -> Option<Page<TodoDetail>> {
    let todo = state.snapshot().into_iter().find(|todo| todo.id == id)?;
    let done = TodoDetail::done();
    Some(
        Page::new(todo.title.clone(), TodoDetail { done: todo.done }).child(
            div()
                .css(styles::page())
                .child(h1().text(todo.title))
                .child(p().id(Id::new("status")).show(done).text("Done."))
                .child(
                    p().id(Id::new("status-open"))
                        .show(!done)
                        .text("Still to do."),
                )
                .child(a().href(INDEX.link()).text("All todos")),
        ),
    )
}

async fn add(state: AppState, draft: String) -> Patches<TodoPage> {
    let title = draft.trim().to_owned();
    if !title.is_empty() {
        state.update(|todos| {
            todos.next_id += 1;
            let todo = Todo {
                id: todos.next_id,
                title,
                done: false,
            };
            todos.items.push(todo.clone());
            Some(Change::Put(todo))
        });
    }
    TodoPage::draft().patch(String::new()).into()
}

async fn toggle(state: AppState, id: u64) -> Patches<TodoPage> {
    state.update(|todos| {
        let todo = todos.items.iter_mut().find(|todo| todo.id == id)?;
        todo.done = !todo.done;
        Some(Change::Put(todo.clone()))
    });
    Patches::new()
}

async fn remove(state: AppState, id: u64) -> Patches<TodoPage> {
    state.update(|todos| {
        let before = todos.items.len();
        todos.items.retain(|todo| todo.id != id);
        (todos.items.len() < before).then_some(Change::Removed(id))
    });
    Patches::new()
}

/// Sends the whole list once, then one patch per change. A stream that falls
/// more than the channel's capacity behind has missed changes, so it gets the
/// whole list again.
fn updates(state: AppState, (): ()) -> impl Stream<Item = Patches<TodoPage>> {
    let changes = BroadcastStream::new(state.changed.subscribe());
    let current: Patches<TodoPage> = TodoPage::todos().patch(state.snapshot()).into();
    stream::once(async { current }).chain(changes.map(move |change| match change {
        Ok(change) => change.patches(),
        Err(_lagged) => TodoPage::todos().patch(state.snapshot()).into(),
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
