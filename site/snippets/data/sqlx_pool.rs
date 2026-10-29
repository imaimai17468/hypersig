# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::html::{li, ul};
# use hypersig::{Page, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Todos {
#     count: i32,
# }
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug)]
struct DbError(sqlx::Error);

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        Self(error)
    }
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        eprintln!("database: {}", self.0);
        let message = "The list could not be read. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

async fn index(state: AppState, (): ()) -> Result<Page<Todos>, DbError> {
    let titles: Vec<String> = sqlx::query_scalar("SELECT title FROM todos ORDER BY id")
        .fetch_all(&state.db)
        .await?;
    let count = i32::try_from(titles.len()).unwrap_or(i32::MAX);
    let list = ul().children(titles.into_iter().map(|title| li().text(title)));
    Ok(Page::new("Todos", Todos { count }).child(list))
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

# async fn run() -> Result<(), sqlx::Error> {
let url = std::env::var("DATABASE_URL").expect("DATABASE_URL names the database");
let db = PgPoolOptions::new().max_connections(5).connect(&url).await?;
let _app: Router = pages(Router::new()).hypersig_runtime().with_state(AppState { db });
# Ok(())
# }
# let _ = run;
