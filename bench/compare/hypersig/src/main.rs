//! TodoMVC-2000 in hypersig, following `examples/todomvc`.
//!
//! Pattern: the list is a `Vec` signal that the browser updates in place. Each
//! add, toggle, and delete also calls an action with that change's arguments,
//! sent before the local update, and the server applies the same change. The
//! remaining count is a computed signal.

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use hypersig::html::{
    El, Key, a, button, each_keyed, footer, h1, header, input, label, li, section, span, strong,
    ul,
};
use hypersig::{Action, Expr, Page, Patches, RouterExt, Signals, Stmt, if_else};
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
    left: i32,
}

const ADD: Action<TodoMvc, (i32, String)> = Action::post("/add");
const SET_DONE: Action<TodoMvc, (i32, bool)> = Action::post("/done");
const DELETE: Action<TodoMvc, i32> = Action::post("/delete");
const STYLE: &str = include_str!("../../shared/todomvc.css");

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

fn seed() -> Vec<Todo> {
    (1..=2000)
        .map(|id| Todo {
            id,
            title: format!("todo {id}"),
            done: id % 3 == 0,
        })
        .collect()
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
        ADD.call_with((next_id, draft.trim()))
            .then(todos.push(
                blank
                    .with(Todo::id(), next_id)
                    .with(Todo::title(), draft.trim()),
            ))
            .then(next_id.set(next_id + 1))
            .then(draft.set("")),
    );
    input()
        .class("new-todo")
        .attr("placeholder", "What needs to be done?")
        .bind(draft)
        .on_key(Key::Enter, add)
}

fn todo_list() -> El<TodoMvc> {
    let filter = TodoMvc::filter();
    ul().class("todo-list").child(each_keyed(
        TodoMvc::todos(),
        |todo| todo.at(Todo::id()),
        |todo| {
            let id = todo.at(Todo::id());
            let done = todo.at(Todo::done());
            let visible = filter
                .equals(&Filter::All)
                .or(filter.equals(&Filter::Active).and(!done))
                .or(filter.equals(&Filter::Completed).and(done));
            li().show(visible)
                .class_when("completed", done)
                .child(
                    input()
                        .class("toggle")
                        .attr("type", "checkbox")
                        .bind(done)
                        .on_change(SET_DONE.call_with((id, done))),
                )
                .child(label().bind_text(todo.at(Todo::title())))
                .child(
                    button()
                        .class("destroy")
                        .on_click(DELETE.call_with(id).then(todo.remove())),
                )
        },
    ))
}

fn filter_link(filter: Filter, text: &str) -> El<TodoMvc> {
    li().child(
        a().attr("href", "#")
            .class_when("selected", TodoMvc::filter().equals(&filter))
            .on_click(TodoMvc::filter().set(&filter))
            .text(text),
    )
}

fn page(todos: Vec<Todo>) -> Page<TodoMvc> {
    let list = TodoMvc::todos();
    let left = TodoMvc::left();
    let next_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    let signals = TodoMvc {
        todos,
        next_id,
        draft: String::new(),
        filter: Filter::All,
        left: 0,
    };
    Page::new("hypersig TodoMVC", signals)
        .style(STYLE)
        .computed(left, list.count(|todo| !todo.at(Todo::done())))
        .child(
            section()
                .class("todoapp")
                .child(header().child(h1().text("todos")).child(new_todo()))
                .child(section().class("main").child(todo_list()))
                .child(
                    footer()
                        .class("footer")
                        .child(
                            span()
                                .class("todo-count")
                                .child(strong().bind_text(left.to_text()))
                                .child(span().bind_text(if_else(
                                    left.equals(1),
                                    " item left",
                                    " items left",
                                ))),
                        )
                        .child(
                            ul().class("filters")
                                .child(filter_link(Filter::All, "All"))
                                .child(filter_link(Filter::Active, "Active"))
                                .child(filter_link(Filter::Completed, "Completed")),
                        ),
                ),
        )
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let todos = state.todos.lock().expect("todo lock poisoned").clone();
    Html(page(todos).render())
}

async fn add(state: AppState, (id, title): (i32, String)) -> Patches<TodoMvc> {
    let mut todos = state.todos.lock().expect("todo lock poisoned");
    todos.push(Todo {
        id,
        title,
        done: false,
    });
    Patches::new()
}

async fn set_done(state: AppState, (id, done): (i32, bool)) -> Patches<TodoMvc> {
    let mut todos = state.todos.lock().expect("todo lock poisoned");
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        todo.done = done;
    }
    Patches::new()
}

async fn delete(state: AppState, id: i32) -> Patches<TodoMvc> {
    state
        .todos
        .lock()
        .expect("todo lock poisoned")
        .retain(|todo| todo.id != id);
    Patches::new()
}

async fn reset(State(state): State<AppState>) -> StatusCode {
    *state.todos.lock().expect("todo lock poisoned") = seed();
    StatusCode::NO_CONTENT
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/bench-reset", post(reset))
        .hypersig_runtime()
        .action(ADD, add)
        .action(SET_DONE, set_done)
        .action(DELETE, delete)
        .with_state(AppState {
            todos: Arc::new(Mutex::new(seed())),
        });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4100").await?;
    axum::serve(listener, app).await
}
