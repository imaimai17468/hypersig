use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

#[derive(Clone)]
struct Admin;

struct CurrentUser;

// The extractor reads the state of an admin router, but serves a router whose state is `()`.
impl FromRequestParts<Admin> for CurrentUser {
    type Rejection = StatusCode;

    async fn from_request_parts(_: &mut Parts, _: &Admin) -> Result<Self, StatusCode> {
        Ok(Self)
    }
}

hypersig::routes! {
    fn actions(Router<()>);
    action ADD: Action<Counter> = post "/add" => add;
}

async fn add((): (), _: CurrentUser, (): ()) -> Patches<Counter> {
    Counter::count().patch(1).into()
}

fn main() {
    let _ = actions;
}
