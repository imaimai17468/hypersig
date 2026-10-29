# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::html::{Role, button, form, input, p};
# use hypersig::{Action, Page, Patches, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Clone)]
# struct AppState {
#     client: reqwest::Client,
#     base: String,
#     token: String,
# }
#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    draft: String,
    problem: String,
}

#[derive(Serialize)]
struct NewNote<'a> {
    text: &'a str,
}

/// The body of a request the service refused.
#[derive(Deserialize)]
struct Refusal {
    error: String,
}

enum ApiError {
    /// The service answered 422 about the visitor's input, which the
    /// visitor can fix.
    Refused(String),
    /// Anything else. The visitor can only try again.
    Unavailable(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Unavailable(error.to_string())
    }
}

/// The runtime writes this body to the signal given to `Call::error`.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Refused(message) => (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
            Self::Unavailable(detail) => {
                eprintln!("notes service: {detail}");
                let message = "The notes service did not answer. Try again.";
                (StatusCode::BAD_GATEWAY, message).into_response()
            }
        }
    }
}

async fn add(state: AppState, draft: String) -> Result<Patches<Notes>, ApiError> {
    let response = state
        .client
        .post(format!("{}/notes", state.base))
        .bearer_auth(&state.token)
        .json(&NewNote { text: &draft })
        .send()
        .await?;
    if response.status() == StatusCode::UNPROCESSABLE_ENTITY {
        let refusal: Refusal = response.json().await?;
        return Err(ApiError::Refused(refusal.error));
    }
    response.error_for_status()?;
    Ok(Notes::draft().patch(String::new()).into())
}

async fn index(_: AppState, (): ()) -> Page<Notes> {
    let signals = Notes { draft: String::new(), problem: String::new() };
    Page::new("Notes", signals).child(
        form()
            .on_submit(ADD.call_with(Notes::draft()).error(Notes::problem()))
            .child(input().bind(Notes::draft()))
            .child(button().text("Add"))
            .child(p().role(Role::Alert).bind_text(Notes::problem())),
    )
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    action ADD: Action<Notes, String> = post "/notes" => add;
}
# let _: Router<AppState> = pages(Router::new());
