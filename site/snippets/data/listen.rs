# use axum::Router;
# use hypersig::html::{each, li, ul};
# use hypersig::{Page, Patches, Route, RouterExt, Signals, StreamAction};
# use serde::{Deserialize, Serialize};
use std::time::Duration;

use futures::{Stream, StreamExt as _, stream};
use sqlx::PgPool;
use sqlx::postgres::{PgListener, PgPoolOptions};
use tokio::sync::broadcast::{self, error::RecvError};

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    titles: Vec<String>,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    /// Tells this process's streams that the table changed.
    changed: broadcast::Sender<()>,
}

async fn titles(db: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT title FROM todos ORDER BY id")
        .fetch_all(db)
        .await
}

async fn index(state: AppState, (): ()) -> Page<Todos> {
    let titles = titles(&state.db).await.unwrap_or_else(|error| {
        // The stream sends the list again once it opens.
        eprintln!("database: {error}");
        Vec::new()
    });
    Page::new("Todos", Todos { titles }).child(
        ul().on_load(UPDATES.call().reconnect())
            .child(each(Todos::titles(), |title| li().bind_text(title.value()))),
    )
}

/// Sends the list once, then again after each change any process makes.
fn updates(state: AppState, (): ()) -> impl Stream<Item = Patches<Todos>> {
    let changes = stream::unfold(state.changed.subscribe(), |mut changes| async move {
        match changes.recv().await {
            // A receiver that fell behind missed some changes, and reading
            // the list again covers them.
            Ok(()) | Err(RecvError::Lagged(_)) => Some(((), changes)),
            Err(RecvError::Closed) => None,
        }
    });
    stream::once(async {}).chain(changes).filter_map(move |()| {
        let db = state.db.clone();
        async move {
            match titles(&db).await {
                Ok(titles) => Some(Todos::titles().patch(titles).into()),
                Err(error) => {
                    eprintln!("database: {error}");
                    None
                }
            }
        }
    })
}

/// Forwards each notification on `todos_changed` to this process's streams.
async fn listen(mut listener: PgListener, changed: broadcast::Sender<()>) {
    loop {
        // After an error, the next `recv` connects again. A notification
        // sent in between is lost, so the streams read the list again.
        if let Err(error) = listener.recv().await {
            eprintln!("database: {error}");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        let _ = changed.send(());
    }
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    stream UPDATES: StreamAction<Todos> = "/todos/updates" => updates;
}

# async fn run() -> Result<(), sqlx::Error> {
let url = std::env::var("DATABASE_URL").expect("DATABASE_URL names the database");
let db = PgPoolOptions::new().max_connections(5).connect(&url).await?;
let mut listener = PgListener::connect_with(&db).await?;
listener.listen("todos_changed").await?;
let (changed, _) = broadcast::channel(16);
tokio::spawn(listen(listener, changed.clone()));
let _app: Router = pages(Router::new())
    .hypersig_runtime()
    .with_state(AppState { db, changed });
# Ok(())
# }
# let _ = run;
