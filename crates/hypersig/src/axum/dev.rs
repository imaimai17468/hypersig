//! The stream that tells a page served by a debug build to reload once the
//! server that served it has stopped.
//!
//! A page whose server restarts with new code is shown by a process that no
//! longer exists, so the page reloads to get the new one. Each process
//! answers at the same path with a token of its own, and the page names the
//! token of the process that rendered it. The runtime imports the code that
//! reads the stream from a script served beside it, so the runtime a release
//! build serves carries none of that code. Beside the URL, the page carries
//! its signal struct's [`SHAPE`](crate::Signals::SHAPE), so the reloaded
//! page restores the signals the visitor changed only into a struct with
//! the same fields. A release build, and a build for
//! wasm32-unknown-unknown, where the host restarts nothing in front of an
//! open tab, serve no stream and write no URL.

#[cfg(all(
    debug_assertions,
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod served {
    use std::collections::HashMap;
    use std::convert::Infallible;
    use std::sync::LazyLock;
    use std::time::{SystemTime, UNIX_EPOCH};

    use axum::Router;
    use axum::extract::Query;
    use axum::http::header;
    use axum::response::sse::{Event, Sse};
    use axum::response::{IntoResponse, Response};
    use axum::routing::get;
    use futures::{StreamExt as _, stream};

    use crate::route::Base;

    /// The path of the stream, beside the runtime's.
    const PATH: &str = "/_hypersig/dev";

    /// The path of the dev script. The runtime imports it from `dev.js`
    /// resolved against the stream's URL, so it sits beside `PATH`.
    const SCRIPT_PATH: &str = "/_hypersig/dev.js";

    /// The code a page served by this build runs, which `pnpm build` writes
    /// from `runtime/src/dev.ts` apart from the runtime.
    const SCRIPT: &str = include_str!("../../assets/dev.js");

    /// This process's token, from the time it first rendered a page or
    /// answered the stream and its process id, which two processes started
    /// one after the other do not share.
    static TOKEN: LazyLock<String> = LazyLock::new(|| {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos());
        format!("{nanos:x}{:x}", std::process::id())
    });

    /// The URL a page rendered under `base` reads the stream from.
    #[expect(
        clippy::unnecessary_wraps,
        reason = "a release build's `url` answers `None` through the same signature"
    )]
    pub(crate) fn url(base: Base) -> Option<String> {
        Some(format!("{}?since={}", base.join(PATH), *TOKEN))
    }

    /// Serves the stream and the dev script on `router`.
    pub(crate) fn route<St: Clone + Send + Sync + 'static>(router: Router<St>) -> Router<St> {
        router
            .route(PATH, get(answer))
            .route(SCRIPT_PATH, get(script))
    }

    /// Serves the dev script. Its path names no hash, so the browser asks
    /// for it again on each load rather than run a copy from an older build.
    async fn script() -> Response {
        let head = [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache"),
        ];
        (head, SCRIPT).into_response()
    }

    /// Sends `hs-reload` at once to a page another process rendered, and
    /// holds the stream open for a page this one rendered until it stops.
    async fn answer(Query(query): Query<HashMap<String, String>>) -> Response {
        let events = if query.get("since") == Some(&*TOKEN) {
            stream::pending().boxed()
        } else {
            // Data is `{}` because axum writes no `data` field for an empty
            // string, and an event without one is not dispatched.
            let reload = Event::default().event("hs-reload").data("{}");
            stream::iter([Ok::<_, Infallible>(reload)]).boxed()
        };
        super::super::keep_alive(Sse::new(events)).into_response()
    }
}

#[cfg(all(
    debug_assertions,
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub(crate) use served::{route, url};

/// The URL of the stream, which a release build does not serve.
#[cfg(not(all(
    debug_assertions,
    not(all(target_arch = "wasm32", target_os = "unknown"))
)))]
pub(crate) fn url(_base: crate::route::Base) -> Option<String> {
    None
}

/// Leaves `router` as it is, because a release build serves no stream.
#[cfg(not(all(
    debug_assertions,
    not(all(target_arch = "wasm32", target_os = "unknown"))
)))]
pub(crate) fn route<St>(router: axum::Router<St>) -> axum::Router<St> {
    router
}
