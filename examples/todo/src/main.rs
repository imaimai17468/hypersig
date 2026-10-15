//! A shared todo list. Every open tab receives changes over a long-lived stream.
//!
//! Run with `cargo run -p example-todo` and open <http://localhost:3001> in two tabs.

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{El, button, div, form, h1, input, li, p, span, ul};
use hypersig::{Action, Fragment, Page, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Signals, Serialize, Deserialize)]
struct TodoPage {
    draft: String,
    remaining: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
struct TodoId(u64);

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
        ul().children(self.items.iter().map(|todo| {
            let title = span().class("title").text(todo.title.as_str());
            li().id(format!("todo-{}", todo.id.0))
                .class(if todo.done { "done" } else { "open" })
                .child(
                    button()
                        .class("check")
                        .attr("aria-label", "toggle")
                        .on_click(TOGGLE.call_with(&todo.id))
                        .text(if todo.done { "✓" } else { "○" }),
                )
                .child(title)
                .child(
                    button()
                        .class("remove")
                        .attr("aria-label", "remove")
                        .on_click(REMOVE.call_with(&todo.id))
                        .text("×"),
                )
        }))
    }
}

const STYLE: &str = "
body { font-family: system-ui, sans-serif; max-width: 32rem; margin: 3rem auto; padding: 0 1rem; }
form { display: flex; gap: .5rem; }
input { flex: 1; font-size: 1rem; padding: .4rem; }
button { font-size: 1rem; cursor: pointer; }
ul { list-style: none; padding: 0; }
li { display: flex; gap: .5rem; align-items: center; padding: .3rem 0; border-bottom: 1px solid #eee; }
li .title { flex: 1; }
li.done .title { text-decoration: line-through; color: #888; }
.check, .remove { border: none; background: none; }
.summary { color: #555; }
";

async fn index(State(state): State<AppState>) -> Html<String> {
    let list = TodoList {
        items: state.snapshot(),
    };
    let draft = TodoPage::draft();
    let remaining = TodoPage::remaining();
    let page = Page::new(
        "hypersig todo",
        TodoPage {
            draft: String::new(),
            remaining: list.remaining(),
        },
    )
    .style(STYLE)
    .child(
        div()
            .on_load(UPDATES.call())
            .child(h1().text("Todo"))
            .child(
                form()
                    .on_submit(ADD.call_with(draft))
                    .child(input().attr("placeholder", "What needs doing?").bind(draft))
                    .child(button().show(!draft.is_empty()).text("Add")),
            )
            .child(hypersig::fragment(&list))
            .child(
                p().class("summary")
                    .show(remaining.gt(0))
                    .bind_text(remaining.to_text().concat(" left")),
            )
            .child(
                p().class("summary")
                    .show(remaining.equals(0))
                    .text("All done."),
            ),
    );
    Html(page.render())
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
    let app = Router::new()
        .route("/", get(index))
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
