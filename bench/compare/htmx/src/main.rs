//! TodoMVC-2000 in htmx 2.0.11, served by axum.
//!
//! Pattern: hypermedia. The server renders the whole page, and each change is
//! an htmx request whose response is the HTML fragment that changed:
//!
//! - Toggle: `hx-post` on `input.toggle` answers the new `li` (swapped with
//!   `outerHTML`) and the `.todo-count` out-of-band (`hx-swap-oob`).
//! - Delete: `hx-delete` on `button.destroy` answers an empty body, so the
//!   `outerHTML` swap removes the `li`, plus the out-of-band count.
//! - Add: the form posts, the new `li` is appended to `ul.todo-list`
//!   (`beforeend`) with the out-of-band count, and an `hx-on::after-request`
//!   handler resets the form. Blank text answers 204, which htmx does not swap.
//!
//! `hx-target="closest li"` and `hx-swap="outerHTML"` are set once on the `ul`
//! and inherited by every item, so each `li` carries only its own URL.
//!
//! Choice a reader might dispute: filtering stays in the browser. A small
//! `hx-on:click` handler on `.filters` sets `data-filter` on the list and CSS
//! hides the other items with `display: none`. Asking the server for the
//! filtered list would resend up to 2000 items per click to change no data,
//! and the list class keeps working for items swapped in later. The filter is
//! not kept in the URL or across reloads.

use std::fmt::Write as _;
use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::Form;
use serde::Deserialize;

const STYLE: &str = include_str!("../../shared/todomvc.css");
const HTMX: &str = include_str!("../assets/htmx.min.js");
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
        "<li{class}><input class=\"toggle\" type=\"checkbox\"{checked} hx-post=\"/todos/{id}/toggle\"><label>"
    );
    escape(out, &todo.title);
    let _ = write!(
        out,
        "</label><button class=\"destroy\" hx-delete=\"/todos/{id}\"></button></li>"
    );
}

fn render_count(out: &mut String, todos: &Todos, oob: bool) {
    let left = todos.items.iter().filter(|todo| !todo.done).count();
    let oob = if oob { " hx-swap-oob=\"true\"" } else { "" };
    let noun = if left == 1 { "item" } else { "items" };
    let _ = write!(
        out,
        "<span id=\"todo-count\" class=\"todo-count\"{oob}><strong>{left}</strong> {noun} left</span>"
    );
}

fn render_page(todos: &Todos) -> String {
    let mut out = String::with_capacity(256 * 1024);
    out.push_str("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
    out.push_str("<title>htmx TodoMVC</title><link rel=\"icon\" href=\"data:,\"><style>");
    out.push_str(STYLE);
    out.push_str(FILTER_STYLE);
    out.push_str("</style><script src=\"/htmx.min.js\"></script></head><body>");
    out.push_str("<section class=\"todoapp\"><header><h1>todos</h1>");
    out.push_str(
        "<form hx-post=\"/todos\" hx-target=\"#todo-list\" hx-swap=\"beforeend\" \
         hx-on::after-request=\"if (event.detail.successful) this.reset()\">\
         <input class=\"new-todo\" name=\"title\" placeholder=\"What needs to be done?\" \
         autocomplete=\"off\" autofocus></form></header>",
    );
    out.push_str(
        "<section class=\"main\"><ul id=\"todo-list\" class=\"todo-list\" \
         hx-target=\"closest li\" hx-swap=\"outerHTML\">",
    );
    for todo in &todos.items {
        render_item(&mut out, todo);
    }
    out.push_str("</ul></section><footer class=\"footer\">");
    render_count(&mut out, todos, false);
    out.push_str(
        "<ul class=\"filters\" hx-on:click=\"\
         const a = event.target.closest('a'); if (!a) return; event.preventDefault();\
         document.getElementById('todo-list').dataset.filter = a.dataset.filter;\
         this.querySelector('.selected').classList.remove('selected'); a.classList.add('selected')\">\
         <li><a href=\"#/\" data-filter=\"all\" class=\"selected\">All</a></li>\
         <li><a href=\"#/active\" data-filter=\"active\">Active</a></li>\
         <li><a href=\"#/completed\" data-filter=\"completed\">Completed</a></li>\
         </ul></footer></section></body></html>",
    );
    out
}

async fn index(State(state): State<AppState>) -> Html<String> {
    Html(render_page(&state.lock().expect("todo lock poisoned")))
}

async fn htmx_js() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        ],
        HTMX,
    )
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

async fn add(State(state): State<AppState>, Form(form): Form<NewTodo>) -> Response {
    let title = form.title.trim();
    if title.is_empty() {
        return StatusCode::NO_CONTENT.into_response();
    }
    let mut todos = state.lock().expect("todo lock poisoned");
    let todo = Todo {
        id: todos.next_id,
        title: title.to_owned(),
        done: false,
    };
    todos.next_id += 1;
    let mut out = String::new();
    render_item(&mut out, &todo);
    todos.items.push(todo);
    render_count(&mut out, &todos, true);
    Html(out).into_response()
}

async fn toggle(State(state): State<AppState>, Path(id): Path<u32>) -> Response {
    let mut todos = state.lock().expect("todo lock poisoned");
    let Some(todo) = todos.items.iter_mut().find(|todo| todo.id == id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    todo.done = !todo.done;
    let mut out = String::new();
    render_item(&mut out, todo);
    render_count(&mut out, &todos, true);
    Html(out).into_response()
}

async fn remove(State(state): State<AppState>, Path(id): Path<u32>) -> Response {
    let mut todos = state.lock().expect("todo lock poisoned");
    todos.items.retain(|todo| todo.id != id);
    let mut out = String::new();
    render_count(&mut out, &todos, true);
    Html(out).into_response()
}

async fn reset(State(state): State<AppState>) -> StatusCode {
    *state.lock().expect("todo lock poisoned") = seed();
    StatusCode::NO_CONTENT
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/htmx.min.js", get(htmx_js))
        .route("/todos", post(add))
        .route("/todos/{id}/toggle", post(toggle))
        .route("/todos/{id}", delete(remove))
        .route("/bench-reset", post(reset))
        .with_state(Arc::new(Mutex::new(seed())));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4102").await?;
    axum::serve(listener, app).await
}
