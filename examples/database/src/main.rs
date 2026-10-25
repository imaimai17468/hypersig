//! The todo list of the `todo` example, kept in a SQLite file. The todos
//! survive a restart, and two server processes opened on one file serve the
//! same list. The channel that tells the streams of a change belongs to one
//! process, so a tab shows a write through the other process when it loads
//! the page again. `examples/postgres` tells the streams of every process.
//!
//! Run with `cargo run -p example-database` and open <http://localhost:3010>.
//! `DATABASE_PATH` names the file, `todos.db` in the working directory by
//! default.

use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{El, Id, Role, a, attr, button, div, form, h1, input, li, p, span, ul};
use hypersig::{Action, Fragment, Page, Patches, Route, RouterExt, Signals, StreamAction};
use rusqlite::{Connection, OptionalExtension as _};
use serde::{Deserialize, Serialize};
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

/// The `id` column of a todo. SQLite's integers are signed 64-bit numbers.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
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

/// Runs each time the file is opened. `IF NOT EXISTS` leaves the table of an
/// earlier run in place, and the journal mode is stored in the file.
const SCHEMA: &str = "
    PRAGMA journal_mode = WAL;
    CREATE TABLE IF NOT EXISTS todos (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        done INTEGER NOT NULL DEFAULT 0
    );
";

struct Todo {
    id: TodoId,
    title: String,
    done: bool,
}

impl Todo {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: TodoId(row.get("id")?),
            title: row.get("title")?,
            done: row.get("done")?,
        })
    }
}

/// A query that failed, or whose task panicked.
struct DbError(String);

impl From<rusqlite::Error> for DbError {
    fn from(error: rusqlite::Error) -> Self {
        Self(error.to_string())
    }
}

impl From<tokio::task::JoinError> for DbError {
    fn from(error: tokio::task::JoinError) -> Self {
        Self(error.to_string())
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "database: {}", self.0)
    }
}

/// The visitor reads this response's body under the form, so it says what
/// failed without the query or the file path, which the server's log keeps.
impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        eprintln!("{self}");
        let message = "The todo list could not be read or saved. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

/// The connection to the file. rusqlite's calls block the thread until
/// SQLite answers, so each query runs on tokio's blocking pool and the
/// async workers keep serving other requests meanwhile.
#[derive(Clone)]
struct Db(Arc<Mutex<Connection>>);

impl Db {
    fn open(path: &str) -> rusqlite::Result<Self> {
        let connection = Connection::open(path)?;
        // A write takes a lock on the whole file. Another process that writes
        // meanwhile waits this long for it instead of failing at once.
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch(SCHEMA)?;
        Ok(Self(Arc::new(Mutex::new(connection))))
    }

    async fn run<T: Send + 'static>(
        &self,
        query: impl FnOnce(&Connection) -> rusqlite::Result<T> + Send + 'static,
    ) -> Result<T, DbError> {
        let connection = Arc::clone(&self.0);
        let result = tokio::task::spawn_blocking(move || {
            query(&connection.lock().expect("database lock poisoned"))
        })
        .await?;
        Ok(result?)
    }

    async fn list(&self) -> Result<Vec<Todo>, DbError> {
        self.run(|connection| {
            connection
                .prepare("SELECT id, title, done FROM todos ORDER BY id")?
                .query_map([], Todo::from_row)?
                .collect()
        })
        .await
    }

    async fn find(&self, id: TodoId) -> Result<Option<Todo>, DbError> {
        self.run(move |connection| {
            connection
                .query_row(
                    "SELECT id, title, done FROM todos WHERE id = ?1",
                    [id.0],
                    Todo::from_row,
                )
                .optional()
        })
        .await
    }

    async fn execute(
        &self,
        sql: &'static str,
        params: impl rusqlite::Params + Send + 'static,
    ) -> Result<(), DbError> {
        self.run(move |connection| connection.execute(sql, params).map(drop))
            .await
    }
}

#[derive(Clone)]
struct AppState {
    db: Db,
    /// Tells this process's update streams that a todo changed. A write by
    /// another process on the same file sends nothing here, so this
    /// process's open tabs show it once they load the page again.
    changed: broadcast::Sender<()>,
}

impl AppState {
    async fn write(
        &self,
        sql: &'static str,
        params: impl rusqlite::Params + Send + 'static,
    ) -> Result<Patches<TodoPage>, DbError> {
        self.db.execute(sql, params).await?;
        let _ = self.changed.send(());
        Ok(TodoPage::problem().patch(String::new()).into())
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
        "hypersig database",
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
    let title = draft.trim().to_owned();
    let patches = if title.is_empty() {
        Patches::new()
    } else {
        state
            .write("INSERT INTO todos (title) VALUES (?1)", [title])
            .await?
    };
    Ok(patches.signals(TodoPage::draft().patch(String::new())))
}

async fn toggle(state: AppState, id: TodoId) -> Result<Patches<TodoPage>, DbError> {
    state
        .write("UPDATE todos SET done = NOT done WHERE id = ?1", [id.0])
        .await
}

async fn remove(state: AppState, id: TodoId) -> Result<Patches<TodoPage>, DbError> {
    state.write("DELETE FROM todos WHERE id = ?1", [id.0]).await
}

/// Sends the list once, then again after each change this process makes. A
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
                    eprintln!("{error}");
                    None
                }
            }
        })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "todos.db".to_owned());
    let db = Db::open(&path).map_err(std::io::Error::other)?;
    let (changed, _) = broadcast::channel(16);
    let state = AppState { db, changed };
    let app = pages(Router::new()).hypersig_runtime().with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3010".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}, todos in {path}");
    axum::serve(listener, app).await
}
