//! TodoMVC-2000 in Datastar 1.0.4, served by axum with the `datastar` SDK 0.4.1.
//!
//! Pattern: hypermedia over server-sent events. The server renders the whole
//! page, and each change is a Datastar action whose SSE response patches only
//! what changed. The page shows a change once that response arrives:
//!
//! - Toggle: `data-on:change` on `input.toggle` runs `@post` to
//!   `/todos/{id}/toggle`. The response carries the new `li`, which Datastar
//!   morphs into the one with the same `id="todo-{id}"`, and the count.
//! - Delete: `data-on:click` on `button.destroy` runs `@delete` to
//!   `/todos/{id}`. The response removes `#todo-{id}` (remove mode) and carries
//!   the count.
//! - Add: `input.new-todo` is bound to the `title` signal with `data-bind`, and
//!   Enter runs `@post` to `/todos`, which sends the signals as JSON. The
//!   handler reads `title` with `ReadSignals`, appends the new `li` to
//!   `#todo-list` (append mode), and answers the count and a patch that sets
//!   `title` back to `""`. Blank text answers an event stream with no events,
//!   because Datastar reports any other kind of response as an error.
//!
//! Choice a reader might dispute: filtering stays in the browser. Clicking a
//! `.filters a` sets the `_filter` signal, `data-attr:data-filter` copies it
//! onto the list, and CSS hides the other items with `display: none`, while
//! `data-class:selected` moves the link's class. Asking the server for the
//! filtered list would resend up to 2000 items per click to change no data,
//! and the CSS rule keeps working for items appended later. The leading `_`
//! keeps the signal out of the JSON that every action sends. The filter is not
//! kept in the URL or across reloads.

use std::convert::Infallible;
use std::fmt::Write as _;
use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::sse::Event;
use axum::response::{Html, IntoResponse, Response, Sse};
use axum::routing::{delete, get, post};
use datastar::axum::ReadSignals;
use datastar::prelude::{ElementPatchMode, PatchElements, PatchSignals};
use serde::Deserialize;

const STYLE: &str = include_str!("../../shared/todomvc.css");
const DATASTAR: &str = include_str!("../assets/datastar.js");
const FILTER_STYLE: &str = ".todo-list[data-filter=active] li.completed, \
.todo-list[data-filter=completed] li:not(.completed) { display: none; }";

struct Todo {
    id: u32,
    title: String,
    done: bool,
}

struct Todos {
    items: Vec<Todo>,
    next_id: u32,
}

type AppState = Arc<Mutex<Todos>>;

type Events = Sse<tokio_stream::Iter<std::vec::IntoIter<Result<Event, Infallible>>>>;

fn seed() -> Todos {
    Todos {
        items: (1..=2000)
            .map(|id| Todo {
                id,
                title: format!("todo {id}"),
                done: id % 3 == 0,
            })
            .collect(),
        next_id: 2001,
    }
}

fn escape(out: &mut String, text: &str) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
}

fn render_item(out: &mut String, todo: &Todo) {
    let (class, checked) = if todo.done {
        (" class=\"completed\"", " checked")
    } else {
        ("", "")
    };
    let id = todo.id;
    let _ = write!(
        out,
        "<li id=\"todo-{id}\"{class}><input class=\"toggle\" type=\"checkbox\"{checked} \
         data-on:change=\"@post('/todos/{id}/toggle')\"><label>"
    );
    escape(out, &todo.title);
    let _ = write!(
        out,
        "</label><button class=\"destroy\" data-on:click=\"@delete('/todos/{id}')\"></button></li>"
    );
}

fn render_count(out: &mut String, todos: &Todos) {
    let left = todos.items.iter().filter(|todo| !todo.done).count();
    let noun = if left == 1 { "item" } else { "items" };
    let _ = write!(
        out,
        "<span id=\"todo-count\" class=\"todo-count\"><strong>{left}</strong> {noun} left</span>"
    );
}

fn count_patch(todos: &Todos) -> Event {
    let mut out = String::new();
    render_count(&mut out, todos);
    PatchElements::new(out).into()
}

fn render_filter(out: &mut String, href: &str, filter: &str, text: &str) {
    let _ = write!(
        out,
        "<li><a href=\"{href}\" data-on:click__prevent=\"$_filter = '{filter}'\" \
         data-class:selected=\"$_filter === '{filter}'\">{text}</a></li>"
    );
}

fn render_page(todos: &Todos) -> String {
    let mut out = String::with_capacity(320 * 1024);
    out.push_str("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
    out.push_str("<title>Datastar TodoMVC</title><link rel=\"icon\" href=\"data:,\"><style>");
    out.push_str(STYLE);
    out.push_str(FILTER_STYLE);
    out.push_str("</style><script type=\"module\" src=\"/datastar.js\"></script></head><body>");
    out.push_str(
        "<section class=\"todoapp\" data-signals=\"{title: '', _filter: 'all'}\">\
         <header><h1>todos</h1>\
         <input class=\"new-todo\" data-bind:title \
         data-on:keydown=\"evt.key === 'Enter' && @post('/todos')\" \
         placeholder=\"What needs to be done?\" autocomplete=\"off\" autofocus></header>",
    );
    out.push_str(
        "<section class=\"main\"><ul id=\"todo-list\" class=\"todo-list\" \
         data-attr:data-filter=\"$_filter\">",
    );
    for todo in &todos.items {
        render_item(&mut out, todo);
    }
    out.push_str("</ul></section><footer class=\"footer\">");
    render_count(&mut out, todos);
    out.push_str("<ul class=\"filters\">");
    render_filter(&mut out, "#/", "all", "All");
    render_filter(&mut out, "#/active", "active", "Active");
    render_filter(&mut out, "#/completed", "completed", "Completed");
    out.push_str("</ul></footer></section></body></html>");
    out
}

fn events(events: Vec<Event>) -> Events {
    Sse::new(tokio_stream::iter(
        events.into_iter().map(Ok).collect::<Vec<_>>(),
    ))
}

async fn index(State(state): State<AppState>) -> Html<String> {
    Html(render_page(&state.lock().expect("todo lock poisoned")))
}

async fn datastar_js() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        ],
        DATASTAR,
    )
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

async fn add(State(state): State<AppState>, ReadSignals(signals): ReadSignals<NewTodo>) -> Events {
    let title = signals.title.trim();
    if title.is_empty() {
        return events(Vec::new());
    }
    let mut todos = state.lock().expect("todo lock poisoned");
    let todo = Todo {
        id: todos.next_id,
        title: title.to_owned(),
        done: false,
    };
    todos.next_id += 1;
    let mut item = String::new();
    render_item(&mut item, &todo);
    todos.items.push(todo);
    let appended = PatchElements::new(item)
        .selector("#todo-list")
        .mode(ElementPatchMode::Append);
    let patches = vec![
        appended.into(),
        count_patch(&todos),
        PatchSignals::new(r#"{"title":""}"#).into(),
    ];
    drop(todos);
    events(patches)
}

async fn toggle(State(state): State<AppState>, Path(id): Path<u32>) -> Response {
    let mut todos = state.lock().expect("todo lock poisoned");
    let Some(todo) = todos.items.iter_mut().find(|todo| todo.id == id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    todo.done = !todo.done;
    let mut item = String::new();
    render_item(&mut item, todo);
    let patches = vec![PatchElements::new(item).into(), count_patch(&todos)];
    drop(todos);
    events(patches).into_response()
}

async fn remove(State(state): State<AppState>, Path(id): Path<u32>) -> Events {
    let mut todos = state.lock().expect("todo lock poisoned");
    todos.items.retain(|todo| todo.id != id);
    let patches = vec![
        PatchElements::new_remove(format!("#todo-{id}")).into(),
        count_patch(&todos),
    ];
    drop(todos);
    events(patches)
}

async fn reset(State(state): State<AppState>) -> StatusCode {
    *state.lock().expect("todo lock poisoned") = seed();
    StatusCode::NO_CONTENT
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/datastar.js", get(datastar_js))
        .route("/todos", post(add))
        .route("/todos/{id}/toggle", post(toggle))
        .route("/todos/{id}", delete(remove))
        .route("/bench-reset", post(reset))
        .with_state(Arc::new(Mutex::new(seed())));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4102").await?;
    axum::serve(listener, app).await
}
