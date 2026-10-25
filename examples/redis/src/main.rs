//! The todo list of the `todo` example, kept in Redis. Every server process
//! started on the same Redis serves the same list, and a change made through
//! one process reaches the open tabs of the others over Redis's `PUBLISH`
//! and `SUBSCRIBE`.
//!
//! Start Redis, then run `cargo run -p example-redis` and open
//! <http://localhost:3014>. `REDIS_URL` names the server,
//! `redis://127.0.0.1/` by default. A hosted Redis takes its own URL there,
//! and one that starts with `rediss://`, for TLS, needs redis-rs's
//! `tokio-rustls-comp` feature, which this example leaves off.

use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{El, Id, Role, a, attr, button, div, form, h1, input, li, p, span, ul};
use hypersig::{Action, Fragment, Page, Patches, Route, RouterExt, Signals, StreamAction};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands as _, Client, Script};
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

/// The id of a todo, the number `ADD_TODO` takes from `NEXT_ID`.
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
const TODO_ROW: Id = Id::new("todo");

/// The counter the next todo's id comes from.
const NEXT_ID: &str = "todos:next";
/// A hash from each todo's id to its title.
const TITLES: &str = "todos:titles";
/// The set of the ids of the todos that are done.
const DONE: &str = "todos:done";
/// The channel each write publishes on and each process subscribes to.
const CHANGED: &str = "todos:changed";

/// The keys each script below receives, in this order.
const KEYS: [&str; 4] = [NEXT_ID, TITLES, DONE, CHANGED];

/// The scripts below are the only writes. Each changes the keys and
/// publishes on `CHANGED` in one script, which Redis runs with no other
/// command in between, so no write can leave the other processes unaware of
/// it.
const ADD_TODO: &str = "
local id = redis.call('INCR', KEYS[1])
redis.call('HSET', KEYS[2], id, ARGV[1])
redis.call('PUBLISH', KEYS[4], '')
";

const TOGGLE_TODO: &str = "
if redis.call('HEXISTS', KEYS[2], ARGV[1]) == 1 then
  if redis.call('SREM', KEYS[3], ARGV[1]) == 0 then
    redis.call('SADD', KEYS[3], ARGV[1])
  end
  redis.call('PUBLISH', KEYS[4], '')
end
";

const REMOVE_TODO: &str = "
redis.call('HDEL', KEYS[2], ARGV[1])
redis.call('SREM', KEYS[3], ARGV[1])
redis.call('PUBLISH', KEYS[4], '')
";

struct Todo {
    id: TodoId,
    title: String,
    done: bool,
}

/// A command that failed.
struct DbError(redis::RedisError);

impl From<redis::RedisError> for DbError {
    fn from(error: redis::RedisError) -> Self {
        Self(error)
    }
}

/// The visitor reads this response's body under the form, so it says what
/// failed without the server's address, which the server's log keeps.
impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        eprintln!("redis: {}", self.0);
        let message = "The todo list could not be read or saved. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

/// A connection that redis-rs opens again after it breaks. Each call works
/// on a clone, which shares the one connection. The command that finds the
/// connection broken fails, and the next one waits for the new connection.
/// A failed write is not sent again, because Redis may have run it before
/// the connection broke, and `ADD_TODO` would then add the todo twice, so
/// the visitor reads the problem and tries again.
#[derive(Clone)]
struct Db(ConnectionManager);

impl Db {
    async fn list(&self) -> Result<Vec<Todo>, DbError> {
        let mut con = self.0.clone();
        let titles: Vec<(i64, String)> = con.hgetall(TITLES).await?;
        let done: Vec<i64> = con.smembers(DONE).await?;
        let mut todos: Vec<Todo> = titles
            .into_iter()
            .map(|(id, title)| Todo {
                id: TodoId(id),
                title,
                done: done.contains(&id),
            })
            .collect();
        todos.sort_by_key(|todo| todo.id.0);
        Ok(todos)
    }

    async fn find(&self, id: TodoId) -> Result<Option<Todo>, DbError> {
        let mut con = self.0.clone();
        let title: Option<String> = con.hget(TITLES, id.0).await?;
        let Some(title) = title else {
            return Ok(None);
        };
        let done: bool = con.sismember(DONE, id.0).await?;
        Ok(Some(Todo { id, title, done }))
    }

    async fn add(&self, title: &str) -> Result<(), DbError> {
        self.write(ADD_TODO, title).await
    }

    async fn toggle(&self, id: TodoId) -> Result<(), DbError> {
        self.write(TOGGLE_TODO, id.0).await
    }

    async fn remove(&self, id: TodoId) -> Result<(), DbError> {
        self.write(REMOVE_TODO, id.0).await
    }

    /// Runs one of the scripts above with `KEYS` and its one argument.
    async fn write(&self, script: &str, arg: impl redis::ToRedisArgs) -> Result<(), DbError> {
        let mut con = self.0.clone();
        Script::new(script)
            .key(&KEYS[..])
            .arg(arg)
            .invoke_async::<()>(&mut con)
            .await?;
        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    db: Db,
    /// Tells this process's update streams that a todo changed. `listen`
    /// sends on it for each message on `CHANGED`, which every write by any
    /// process on the same Redis publishes, this one's included.
    changed: broadcast::Sender<()>,
}

/// The patch an action answers with once its write succeeded. The write
/// published on `CHANGED`, so the list arrives over the streams.
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
        "hypersig redis",
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
                    eprintln!("redis: {}", error.0);
                    None
                }
            }
        })
}

/// Forwards each message on `CHANGED` to `changed`. Redis keeps no message
/// for a subscriber that is not connected, so after the connection breaks,
/// this subscribes again and tells the streams to read the list again, for
/// the writes published in between.
async fn listen(client: Client, changed: broadcast::Sender<()>) {
    loop {
        match subscribe(&client).await {
            Ok(mut pubsub) => {
                let _ = changed.send(());
                let mut messages = pubsub.on_message();
                while messages.next().await.is_some() {
                    let _ = changed.send(());
                }
                eprintln!("redis: the subscription closed");
            }
            Err(error) => eprintln!("redis: {error}"),
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn subscribe(client: &Client) -> redis::RedisResult<redis::aio::PubSub> {
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe(CHANGED).await?;
    Ok(pubsub)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_owned());
    let client = Client::open(url)?;
    let db = Db(client.get_connection_manager().await?);
    let (changed, _) = broadcast::channel(16);
    tokio::spawn(listen(client, changed.clone()));

    let state = AppState { db, changed };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(ADD, add)
        .action(TOGGLE, toggle)
        .action(REMOVE, remove)
        .stream(UPDATES, updates)
        .with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3014".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
