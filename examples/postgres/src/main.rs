//! The todo list of the `todo` example, kept in PostgreSQL with sqlx. Every
//! server process started on the same database serves the same list, and a
//! change made through one process reaches the open tabs of the others over
//! PostgreSQL's `LISTEN` and `NOTIFY`.
//!
//! Each query is a file under `queries/`, which a `sqlx::query_file!` macro
//! reads and checks, with its parameters and its columns, against the tables
//! of `migrations/` when the example is built. The build reads that check's input from `.sqlx`, so it needs no
//! database. After changing a query or a migration, apply the migrations to a
//! database and write `.sqlx` again:
//!
//! ```sh
//! export DATABASE_URL=postgres://postgres:password@localhost/postgres
//! cargo sqlx migrate run --source examples/postgres/migrations
//! (cd examples/postgres && cargo sqlx prepare)
//! ```
//!
//! Start PostgreSQL, then run `cargo run -p example-postgres` with
//! `DATABASE_URL` set and open <http://localhost:3011>. The server applies
//! the migrations when it starts. A hosted database, such as Supabase,
//! Cloud SQL, or RDS, takes its own connection string in `DATABASE_URL`.

use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{El, Id, Role, a, attr, button, div, form, h1, input, li, p, span, ul};
use hypersig::{Action, Fragment, Page, Patches, Route, RouterExt, Signals, StreamAction};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::{PgListener, PgPoolOptions};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Signals, Serialize, Deserialize)]
struct TodoPage {
    draft: String,
    remaining: i32,
    problem: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct TodoDetail {
    done: bool,
}

/// The `id` column of a todo, a `BIGINT`. A query under `queries/` names it
/// as `id AS "id: TodoId"`, and the macro reads the column into this type.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
struct TodoId(i64);

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    const TODO: Route<i64> = "/todos/{id}" => todo;
    action ADD: Action<TodoPage, String> = post "/todos" => add;
    action TOGGLE: Action<TodoPage, TodoId> = post "/todos/toggle" => toggle;
    action REMOVE: Action<TodoPage, TodoId> = post "/todos/remove" => remove;
    stream UPDATES: StreamAction<TodoPage> = "/todos/updates" => updates;
}

const TODO_ROW: Id = Id::new("todo");

/// The channel the trigger of `migrations/` notifies after each change to
/// `todos`, and each process listens on. The two names are written apart, so
/// a listener on another name hears nothing and the tabs of other processes
/// show a change only once they load the page again.
const CHANGED: &str = "todos_changed";

struct Todo {
    id: TodoId,
    title: String,
    done: bool,
}

/// A query that failed.
struct DbError(sqlx::Error);

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        Self(error)
    }
}

/// The visitor reads this response's body under the form, so it says what
/// failed without the query or the connection string, which the server's
/// log keeps.
impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        eprintln!("database: {}", self.0);
        let message = "The todo list could not be read or saved. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

/// A pool of connections. sqlx's calls are `async`, so a handler awaits a
/// query and the worker thread serves other requests meanwhile.
#[derive(Clone)]
struct Db(PgPool);

impl Db {
    async fn list(&self) -> Result<Vec<Todo>, DbError> {
        let todos = sqlx::query_file_as!(Todo, "queries/list_todos.sql")
            .fetch_all(&self.0)
            .await?;
        Ok(todos)
    }

    async fn find(&self, id: TodoId) -> Result<Option<Todo>, DbError> {
        let todo = sqlx::query_file_as!(Todo, "queries/find_todo.sql", id.0)
            .fetch_optional(&self.0)
            .await?;
        Ok(todo)
    }

    async fn add(&self, title: &str) -> Result<(), DbError> {
        sqlx::query_file!("queries/add_todo.sql", title)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    async fn toggle(&self, id: TodoId) -> Result<(), DbError> {
        sqlx::query_file!("queries/toggle_todo.sql", id.0)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    async fn remove(&self, id: TodoId) -> Result<(), DbError> {
        sqlx::query_file!("queries/remove_todo.sql", id.0)
            .execute(&self.0)
            .await?;
        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    db: Db,
    /// Tells this process's update streams that a todo changed. `listen`
    /// sends on it for each notification on `CHANGED`, which the trigger
    /// sends after a write by any process on the database, this one included.
    changed: broadcast::Sender<()>,
}

/// The patch an action answers with once its write succeeded. The trigger
/// tells the streams, so the list arrives over them.
fn written() -> Patches<TodoPage> {
    TodoPage::problem().patch(String::new()).into()
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
    const ID: Id = Id::new("todo-list");

    fn render(&self) -> El<TodoPage> {
        let problem = TodoPage::problem();
        ul().css(styles::list())
            .children(self.items.iter().map(|todo| {
                let title = span().css(styles::title(todo.done)).child(
                    a().css(styles::link())
                        .href(TODO.link_with(todo.id.0))
                        .text(todo.title.as_str()),
                );
                li().id(TODO_ROW.with_key(todo.id.0))
                    .css(styles::item())
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr(attr::ARIA_LABEL, "toggle")
                            .on_click(TOGGLE.call_with(&todo.id).error(problem))
                            .text(if todo.done { "✓" } else { "○" }),
                    )
                    .child(title)
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr(attr::ARIA_LABEL, "remove")
                            .on_click(REMOVE.call_with(&todo.id).error(problem))
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

    pub fn problem() -> Style {
        Style::new().color(Color::hex(0x00b4_2318))
    }
}

async fn index(state: AppState, (): ()) -> Result<Page<TodoPage>, DbError> {
    let list = TodoList {
        items: state.db.list().await?,
    };
    let draft = TodoPage::draft();
    let remaining = TodoPage::remaining();
    let problem = TodoPage::problem();
    Ok(Page::new(
        "hypersig postgres",
        TodoPage {
            draft: String::new(),
            remaining: list.remaining(),
            problem: String::new(),
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
                    .on_submit(ADD.call_with(draft).error(problem))
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
            .child(
                p().id(Id::new("problem"))
                    .role(Role::Alert)
                    .css(styles::problem())
                    .show(!problem.is_empty())
                    .bind_text(problem),
            )
            .child(hypersig::fragment(&list))
            .child(
                p().id(Id::new("summary-left"))
                    .css(styles::summary())
                    .show(remaining.gt(0))
                    .bind_text(remaining.to_text().concat(" left")),
            )
            .child(
                p().id(Id::new("summary-done"))
                    .css(styles::summary())
                    .show(remaining.equals(0))
                    .text("All done."),
            ),
    ))
}

async fn todo(state: AppState, id: i64) -> Result<Option<Page<TodoDetail>>, DbError> {
    let Some(todo) = state.db.find(TodoId(id)).await? else {
        return Ok(None);
    };
    let done = TodoDetail::done();
    Ok(Some(
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
    ))
}

async fn add(state: AppState, draft: String) -> Result<Patches<TodoPage>, DbError> {
    let title = draft.trim();
    let patches = if title.is_empty() {
        Patches::new()
    } else {
        state.db.add(title).await?;
        written()
    };
    Ok(patches.signals(TodoPage::draft().patch(String::new())))
}

async fn toggle(state: AppState, id: TodoId) -> Result<Patches<TodoPage>, DbError> {
    state.db.toggle(id).await?;
    Ok(written())
}

async fn remove(state: AppState, id: TodoId) -> Result<Patches<TodoPage>, DbError> {
    state.db.remove(id).await?;
    Ok(written())
}

/// Sends the list once, then again after each change any process makes. A
/// failed read is logged and skipped, so the tab keeps the list it shows and
/// takes the next one.
fn updates(state: AppState, (): ()) -> impl Stream<Item = Patches<TodoPage>> {
    let changes = BroadcastStream::new(state.changed.subscribe()).map(|_| ());
    stream::once(async {})
        .chain(changes)
        .then(move |()| {
            let db = state.db.clone();
            async move { db.list().await }
        })
        .filter_map(|items| async move {
            match items {
                Ok(items) => Some(TodoList { items }.patches()),
                Err(error) => {
                    eprintln!("database: {}", error.0);
                    None
                }
            }
        })
}

/// Forwards each notification on `CHANGED` to `changed`. When the
/// connection breaks, `recv` returns the error and the next call connects
/// again, and a notification sent in between is lost, so the streams are
/// told to read the list again then too.
async fn listen(mut listener: PgListener, changed: broadcast::Sender<()>) {
    loop {
        if let Err(error) = listener.recv().await {
            eprintln!("database: {error}");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        let _ = changed.send(());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        return Err("set DATABASE_URL to a PostgreSQL connection string".into());
    };
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    // Applies the migrations a database has not run yet. A second process
    // that starts meanwhile waits on sqlx's lock and finds them applied.
    sqlx::migrate!().run(&pool).await?;
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(CHANGED).await?;
    let (changed, _) = broadcast::channel(16);
    tokio::spawn(listen(listener, changed.clone()));

    let state = AppState {
        db: Db(pool),
        changed,
    };
    let app = pages(Router::new()).hypersig_runtime().with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3011".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
