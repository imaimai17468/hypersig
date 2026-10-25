# use axum::http::StatusCode;
# use hypersig::{Action, Patches, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

# #[derive(Signals, Serialize, Deserialize)]
# struct Profile {
#     saved: String,
# }
const SAVE: Action<Profile, String> = Action::post("/save");

async fn save((): (), name: String) -> Result<Patches<Profile>, (StatusCode, &'static str)> {
    if name.is_empty() {
        // An event of the application's own, at the level it picks.
        tracing::warn!("refused an empty name");
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Write a name."));
    }
    Ok(Profile::saved().patch(name).into())
}

// Once, at the start of `main`: write each event at INFO or above to stderr.
tracing_subscriber::fmt().with_max_level(Level::INFO).init();

// Each request gets a span with its method and URI, and one line with the
// answer's status and how long it took. Both are DEBUG unless set here.
let trace = TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO));
let _app: Router = Router::new().action(SAVE, save).layer(trace);
