//! A counter that runs on Cloudflare Workers, saves its count in D1, and
//! shows a clock the worker streams.
//!
//! The page, its actions, and its stream are the same hypersig code an axum
//! server runs. The worker hands each request to the axum `Router`.
//!
//! Run `npx wrangler dev` and open <http://localhost:8787>, and publish it
//! with `npx wrangler deploy`. Each request may run in a different isolate,
//! so a value kept in memory does not reach the next request, and the saved
//! count lives in the D1 database that `wrangler.toml` binds as `DB`.

use std::rc::Rc;
use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures::{Stream, stream};
use hypersig::html::{Stylesheet, button, div, h1, p, span};
use hypersig::{
    Action, DateFormat, DateStyle, IntlLocale, Page, Patches, Route, RouterExt, Signals,
    StreamAction, TimeStyle, Timestamp,
};
use serde::{Deserialize, Serialize};
use tower_service::Service as _;
use worker::send::{SendFuture, SendWrapper};
use worker::{Context, D1Database, Delay, Env, HttpRequest, console_error, event};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
    now: Timestamp,
    problem: String,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

const DOUBLE: Action<Counter, i32> = Action::post("/double");
const SAVE: Action<Counter, i32> = Action::post("/save");
const CLOCK: StreamAction<Counter> = StreamAction::new("/clock");

const TIME: DateFormat =
    DateFormat::date_time(IntlLocale::En, DateStyle::Medium, TimeStyle::Medium);

const STYLE: Stylesheet = Stylesheet::new(
    "
body { font-family: system-ui, sans-serif; max-width: 32rem; margin: 3rem auto; padding: 0 1rem; }
button { font-size: 1rem; padding: .4rem .9rem; }
",
);

/// The binding of the D1 database. A D1 handle is a JavaScript object, which
/// is not `Send`, and a worker runs on one thread, so the wrapper never moves
/// it to another.
#[derive(Clone)]
struct AppState {
    db: SendWrapper<Rc<D1Database>>,
}

/// Runs before each query. `IF NOT EXISTS` leaves the table in place once the
/// first request has made it, so a new database needs no migration first.
const SCHEMA: &str =
    "CREATE TABLE IF NOT EXISTS counter (id INTEGER PRIMARY KEY, count INTEGER NOT NULL)";

/// A D1 call that failed. The visitor reads the body, and the worker's log
/// keeps D1's own message.
struct DbError(worker::Error);

impl From<worker::Error> for DbError {
    fn from(error: worker::Error) -> Self {
        Self(error)
    }
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        console_error!("D1: {}", self.0);
        let message = "The count could not be read or saved. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

impl AppState {
    async fn saved(&self) -> Result<i32, DbError> {
        let db = Rc::clone(&self.db);
        // D1 answers through a JavaScript promise, which is not `Send` either.
        SendFuture::new(async move {
            db.exec(SCHEMA).await?;
            let count = db
                .prepare("SELECT count FROM counter WHERE id = 1")
                .first::<i32>(Some("count"))
                .await?;
            Ok::<_, DbError>(count.unwrap_or(0))
        })
        .await
    }

    async fn save(&self, count: i32) -> Result<(), DbError> {
        let db = Rc::clone(&self.db);
        SendFuture::new(async move {
            db.exec(SCHEMA).await?;
            db.prepare(
                "INSERT INTO counter (id, count) VALUES (1, ?1) \
                 ON CONFLICT (id) DO UPDATE SET count = excluded.count",
            )
            .bind(&[count.into()])?
            .run()
            .await?;
            Ok::<_, DbError>(())
        })
        .await
    }
}

async fn index(state: AppState, (): ()) -> Result<Page<Counter>, DbError> {
    let count = Counter::count();
    let problem = Counter::problem();
    let initial = Counter {
        count: state.saved().await?,
        now: Timestamp::now(),
        problem: String::new(),
    };
    Ok(Page::new("hypersig on Cloudflare Workers", initial)
        .style(STYLE)
        .child(
            div()
                .on_load(CLOCK.call().reconnect())
                .child(h1().text("Counter"))
                .child(
                    p().child(button().on_click(count.set(count + 1)).text("+1"))
                        .child(span().bind_text(count.to_text()))
                        .child(
                            button()
                                .on_click(DOUBLE.call_with(count))
                                .text("Double on the worker"),
                        )
                        .child(
                            button()
                                .on_click(SAVE.call_with(count).error(problem))
                                .text("Save in D1"),
                        ),
                )
                .child(
                    p().attr("role", "alert")
                        .show(!problem.is_empty())
                        .bind_text(problem),
                )
                .child(p().bind_text(Counter::now().format_date(&TIME))),
        ))
}

async fn double(_: AppState, count: i32) -> Patches<Counter> {
    Counter::count().patch(count.saturating_mul(2)).into()
}

async fn save(state: AppState, count: i32) -> Result<Patches<Counter>, DbError> {
    state.save(count).await?;
    Ok(Counter::problem().patch(String::new()).into())
}

/// The worker's time, once a second.
fn clock(_: AppState, (): ()) -> impl Stream<Item = Patches<Counter>> {
    stream::unfold((), |()| async {
        // `Delay` waits on a JavaScript timer, which is not `Send`. A worker
        // runs on one thread, so the wrapper never moves it to another.
        SendFuture::new(Delay::from(Duration::from_secs(1))).await;
        Some((Counter::now().patch(Timestamp::now()).into(), ()))
    })
}

#[event(fetch)]
async fn fetch(
    request: HttpRequest,
    env: Env,
    _context: Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let state = AppState {
        db: SendWrapper::new(Rc::new(env.d1("DB")?)),
    };
    let mut app = pages(Router::new())
        .hypersig_runtime()
        .action(DOUBLE, double)
        .action(SAVE, save)
        .stream(CLOCK, clock)
        .with_state(state);
    Ok(app.call(request).await?)
}
