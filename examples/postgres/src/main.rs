//! The todo list of the `todo` example, kept in PostgreSQL with sqlx. Every
//! server process started on the same database serves the same list, and a
//! change made through one process reaches the open tabs of the others over
//! PostgreSQL's `LISTEN` and `NOTIFY`.
//!
//! Start PostgreSQL, then run
//! `DATABASE_URL=postgres://postgres:password@localhost/postgres cargo run -p example-postgres`
//! and open <http://localhost:3011>. A hosted database, such as Supabase,
//! Cloud SQL, or RDS, takes its own connection string in `DATABASE_URL`.

use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{El, a, button, div, form, h1, input, li, p, span, ul};
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

/// The `id` column of a todo, a `BIGINT`.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
struct TodoId(i64);

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    const TODO: Route<i64> = "/todos/{id}" => todo;
}

const ADD: Action<TodoPage, String> = Action::post("/todos");
const TOGGLE: Action<TodoPage, TodoId> = Action::post("/todos/toggle");
const REMOVE: Action<TodoPage, TodoId> = Action::post("/todos/remove");
const UPDATES: StreamAction<TodoPage> = StreamAction::new("/todos/updates");

/// Runs when the server starts. `IF NOT EXISTS` leaves the table of an
/// earlier run, or of another process, in place.
const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS todos (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
)";

/// The channel each write notifies and each process listens on.
const CHANGED: &str = "todos_changed";

struct Todo {
    id: TodoId,
    title: String,
    done: bool,
}

impl From<(i64, String, bool)> for Todo {
    fn from((id, title, done): (i64, String, bool)) -> Self {
        Self {
            id: TodoId(id),
            title,
            done,
        }
    }
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
        let rows: Vec<(i64, String, bool)> =
            sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id")
                .fetch_all(&self.0)
                .await?;
        Ok(rows.into_iter().map(Todo::from).collect())
    }

    async fn find(&self, id: TodoId) -> Result<Option<Todo>, DbError> {
        let row: Option<(i64, String, bool)> =
            sqlx::query_as("SELECT id, title, done FROM todos WHERE id = $1")
                .bind(id.0)
                .fetch_optional(&self.0)
                .await?;
        Ok(row.map(Todo::from))
    }
}

#[derive(Clone)]
struct AppState {
    db: Db,
    /// Tells this process's update streams that a todo changed. `listen`
    /// sends on it for each notification on `CHANGED`, which comes from a
    /// write by any process on the database, this one included.
    changed: broadcast::Sender<()>,
}

impl AppState {
    /// Runs `query` and notifies `CHANGED` in one transaction, so a process
    /// hears of the change only once it is committed.
    async fn write(
        &self,
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<Patches<TodoPage>, DbError> {
        let mut transaction = self.db.0.begin().await?;
        query.execute(&mut *transaction).await?;
        sqlx::query("SELECT pg_notify($1, '')")
            .bind(CHANGED)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
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
    const ID: &'static str = "todo-list";

    fn render(&self) -> El<TodoPage> {
        let problem = TodoPage::problem();
        ul().css(styles::list())
            .children(self.items.iter().map(|todo| {
                let title = span().css(styles::title(todo.done)).child(
                    a().css(styles::link())
                        .href(TODO.link_with(todo.id.0))
                        .text(todo.title.as_str()),
                );
                li().id(format!("todo-{}", todo.id.0))
                    .css(styles::item())
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr("aria-label", "toggle")
                            .on_click(TOGGLE.call_with(&todo.id).error(problem))
                            .text(if todo.done { "✓" } else { "○" }),
                    )
                    .child(title)
                    .child(
                        button()
                            .css(styles::icon_button())
                            .attr("aria-label", "remove")
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
            .child(
                p().id("problem")
                    .attr("role", "alert")
                    .css(styles::problem())
                    .show(!problem.is_empty())
                    .bind_text(problem),
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
                .child(p().id("status").show(done).text("Done."))
                .child(p().id("status-open").show(!done).text("Still to do."))
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
            .write(sqlx::query("INSERT INTO todos (title) VALUES ($1)").bind(title))
            .await?
    };
    Ok(patches.signals(TodoPage::draft().patch(String::new())))
}

async fn toggle(state: AppState, id: TodoId) -> Result<Patches<TodoPage>, DbError> {
    state
        .write(sqlx::query("UPDATE todos SET done = NOT done WHERE id = $1").bind(id.0))
        .await
}

async fn remove(state: AppState, id: TodoId) -> Result<Patches<TodoPage>, DbError> {
    state
        .write(sqlx::query("DELETE FROM todos WHERE id = $1").bind(id.0))
        .await
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
    sqlx::raw_sql(SCHEMA).execute(&pool).await?;
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(CHANGED).await?;
    let (changed, _) = broadcast::channel(16);
    tokio::spawn(listen(listener, changed.clone()));

    let state = AppState {
        db: Db(pool),
        changed,
    };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(ADD, add)
        .action(TOGGLE, toggle)
        .action(REMOVE, remove)
        .stream(UPDATES, updates)
        .with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3011".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
