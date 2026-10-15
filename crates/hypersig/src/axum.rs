//! Serving actions and the runtime from an axum [`Router`].

use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;

use axum::Router;
use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::{StatusCode, header};
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{MethodRouter, get, post};
use futures::{Stream, StreamExt as _, stream};
use serde::de::DeserializeOwned;

use crate::action::{Action, Method};
use crate::patch::{PatchEvent, Patches};
use crate::signal::Signals;

/// Adds hypersig routes to an axum [`Router`].
pub trait RouterExt<St>: Sized {
    /// Serves the browser runtime at [`RUNTIME_PATH`](crate::RUNTIME_PATH).
    #[must_use]
    fn hypersig_runtime(self) -> Self;

    /// Serves `action` with a handler that answers with one batch of patches.
    ///
    /// The handler receives the router state and the arguments the call
    /// passed, already deserialized. A request whose arguments do not
    /// deserialize gets a 400 before the handler runs.
    #[must_use]
    fn action<S, A, H, Fut, P>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Fn(St, A) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: Into<Patches<S>>;

    /// Serves `action` with a handler that answers with a stream of patches.
    ///
    /// The response stays open until the stream ends or the browser leaves
    /// the page. Use it with [`El::on_load`](crate::html::El::on_load) to push
    /// changes made by other users.
    #[must_use]
    fn stream<S, A, H, Str>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Fn(St, A) -> Str + Clone + Send + Sync + 'static,
        Str: Stream<Item = Patches<S>> + Send + 'static;
}

impl<St: Clone + Send + Sync + 'static> RouterExt<St> for Router<St> {
    fn hypersig_runtime(self) -> Self {
        self.route(
            crate::RUNTIME_PATH,
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
                    crate::RUNTIME_JS,
                )
            }),
        )
    }

    fn action<S, A, H, Fut, P>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Fn(St, A) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: Into<Patches<S>>,
    {
        let method = action.method();
        let route = move |State(state): State<St>,
                          Query(query): Query<HashMap<String, String>>,
                          body: Bytes| {
            let handler = handler.clone();
            async move {
                let args = read_args::<A>(method, &query, &body)?;
                let patches: Patches<S> = handler(state, args).await.into();
                let events = stream::iter(patches.into_events())
                    .map(|event| Ok::<_, Infallible>(to_sse(&event)));
                Ok::<_, RequestError>(Sse::new(events).into_response())
            }
        };
        self.route(action.path(), method_router(method, route))
    }

    fn stream<S, A, H, Str>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Fn(St, A) -> Str + Clone + Send + Sync + 'static,
        Str: Stream<Item = Patches<S>> + Send + 'static,
    {
        let method = action.method();
        let route = move |State(state): State<St>,
                          Query(query): Query<HashMap<String, String>>,
                          body: Bytes| {
            let handler = handler.clone();
            async move {
                let args = read_args::<A>(method, &query, &body)?;
                let events = handler(state, args)
                    .flat_map(|patches| stream::iter(patches.into_events()))
                    .map(|event| Ok::<_, Infallible>(to_sse(&event)));
                Ok::<_, RequestError>(
                    Sse::new(events)
                        .keep_alive(KeepAlive::default())
                        .into_response(),
                )
            }
        };
        self.route(action.path(), method_router(method, route))
    }
}

fn method_router<St, H, T>(method: Method, handler: H) -> MethodRouter<St>
where
    St: Clone + Send + Sync + 'static,
    H: axum::handler::Handler<T, St>,
    T: 'static,
{
    match method {
        Method::Get => get(handler),
        Method::Post => post(handler),
    }
}

/// Reads the arguments from the body of a POST and from the `args` query parameter of a GET.
fn read_args<A: DeserializeOwned>(
    method: Method,
    query: &HashMap<String, String>,
    body: &[u8],
) -> Result<A, RequestError> {
    match method {
        Method::Post if !body.is_empty() => serde_json::from_slice(body),
        Method::Post => serde_json::from_str("null"),
        Method::Get => serde_json::from_str(query.get("args").map_or("null", String::as_str)),
    }
    .map_err(RequestError::new)
}

fn to_sse(event: &PatchEvent) -> SseEvent {
    SseEvent::default().event(event.name()).data(event.data())
}

/// A request to an action whose arguments did not deserialize.
///
/// Its response is `400 Bad Request` with the serde message as the body.
#[derive(Debug)]
pub struct RequestError {
    source: serde_json::Error,
    backtrace: Backtrace,
}

impl RequestError {
    fn new(source: serde_json::Error) -> Self {
        Self {
            source,
            backtrace: Backtrace::capture(),
        }
    }

    /// The backtrace captured where the error was created.
    #[must_use]
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the action's arguments did not deserialize: {}",
            self.source
        )
    }
}

impl StdError for RequestError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
