//! Serving actions and the runtime from an axum [`Router`].

use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

use axum::Router;
use axum::body::Bytes;
use axum::extract::{FromRequestParts, MatchedPath, Query, RawPathParams, State};
use axum::http::request::Parts;
use axum::http::{StatusCode, header};
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use axum::response::{Html, IntoResponse, IntoResponseParts, Redirect, Response};
use axum::routing::{MethodRouter, get, post};
use futures::{Stream, StreamExt as _, stream};
use serde::de::DeserializeOwned;

use crate::action::{Action, Method};
use crate::page::Page;
use crate::patch::{PatchEvent, Patches, SignalPatch};
use crate::route::{Params, Route, Search, Url};
use crate::signal::Signals;

/// Adds hypersig routes to an axum [`Router`].
///
/// Each handler takes the router state and its arguments, and may take one
/// [`FromRequestParts`] extractor between them, such as a `HeaderMap` or the
/// application's own signed-in user. A tuple of extractors counts as one.
/// See [`Handler`].
pub trait RouterExt<St>: Sized {
    /// Serves the browser runtime at [`RUNTIME_PATH`](crate::RUNTIME_PATH).
    #[must_use]
    fn hypersig_runtime(self) -> Self;

    /// Serves the page at `route` on `GET`. `routes!` calls it for each page it declares.
    #[doc(hidden)]
    #[must_use]
    fn page<P, Q, H, M, Fut, R>(self, route: Route<P, Q>, handler: H) -> Self
    where
        P: Params + Send + 'static,
        Q: Search + DeserializeOwned + Send + 'static,
        H: Handler<St, Q::Args<P>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = R> + Send + 'static,
        R: Into<PageResponse>;

    /// Serves `action` with a handler that returns one batch of patches.
    ///
    /// The handler receives the router state and the arguments the call
    /// passed, already deserialized, and returns anything that converts into
    /// an [`ActionResponse`]. A request whose arguments do not deserialize
    /// gets a 400 before the handler runs.
    #[must_use]
    fn action<S, A, H, M, Fut, P>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, A, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: Into<ActionResponse<S>>;

    /// Serves `action` with a handler that returns a stream of patches.
    ///
    /// The response stays open until the stream ends or the browser leaves
    /// the page. Use it with [`El::on_load`](crate::html::El::on_load) to push
    /// changes made by other users. The request is refused as for
    /// [`RouterExt::action`].
    #[must_use]
    fn stream<S, A, H, M, Str>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, A, M, Output = Str>,
        M: 'static,
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

    fn page<P, Q, H, M, Fut, R>(self, route: Route<P, Q>, handler: H) -> Self
    where
        P: Params + Send + 'static,
        Q: Search + DeserializeOwned + Send + 'static,
        H: Handler<St, Q::Args<P>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = R> + Send + 'static,
        R: Into<PageResponse>,
    {
        let path = route.path();
        let route_handler = move |State(state): State<St>,
                                  matched: MatchedPath,
                                  params: RawPathParams,
                                  request: Parts| {
            let handler = handler.clone();
            let nested = matched.as_str() != path;
            let segments: Vec<&str> = params.iter().map(|(_, value)| value).collect();
            let params = P::from_segments(&segments);
            let search = Q::unit().map_or_else(
                || Query::<Q>::try_from_uri(&request.uri).map(|Query(search)| search),
                Ok,
            );
            let served_at = matched.as_str().to_owned();
            async move {
                if nested {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!(
                            "the page at `{path}` is registered on a nested router, which serves it at `{served_at}`"
                        ),
                    )
                        .into_response();
                }
                let Some(params) = params else {
                    return StatusCode::NOT_FOUND.into_response();
                };
                let search = match search {
                    Ok(search) => search,
                    Err(rejection) => return rejection.into_response(),
                };
                match handler.call(state, request, Q::join(params, search)).await {
                    Ok(page) => page.await.into().into_response(),
                    Err(rejection) => rejection,
                }
            }
        };
        self.route(route.path(), get(route_handler))
    }

    fn action<S, A, H, M, Fut, P>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, A, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: Into<ActionResponse<S>>,
    {
        let method = action.method();
        let route = move |State(state): State<St>,
                          Query(query): Query<HashMap<String, String>>,
                          request: Parts,
                          body: Bytes| {
            let handler = handler.clone();
            async move {
                let args = read_args::<A>(method, &query, &body)?;
                Ok::<_, RequestError>(match handler.call(state, request, args).await {
                    Ok(reply) => reply.await.into().into_response(),
                    Err(rejection) => rejection,
                })
            }
        };
        self.route(action.path(), method_router(method, route))
    }

    fn stream<S, A, H, M, Str>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, A, M, Output = Str>,
        M: 'static,
        Str: Stream<Item = Patches<S>> + Send + 'static,
    {
        let method = action.method();
        let route = move |State(state): State<St>,
                          Query(query): Query<HashMap<String, String>>,
                          request: Parts,
                          body: Bytes| {
            let handler = handler.clone();
            async move {
                let args = read_args::<A>(method, &query, &body)?;
                let patches = match handler.call(state, request, args).await {
                    Ok(patches) => patches,
                    Err(rejection) => return Ok(rejection),
                };
                let events = patches
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

/// A function that serves a page, an action, or a stream.
///
/// It is implemented for `Fn(St, A) -> O` and for `Fn(St, E, A) -> O`, where
/// `St` is the router state, `A` the page's parameters or the action's
/// arguments, and `E` a [`FromRequestParts`] extractor that runs before the
/// function. When the extractor fails, its rejection is the response and the
/// function does not run. `M` tells the two forms apart, and the compiler
/// infers it.
///
/// An extractor that guards an action should refuse with a status outside
/// 2xx, which the page's [`Call::error`](crate::Call::error) shows. The
/// browser's `fetch` follows a redirect, so an action refused with one hands
/// the page the HTML it points to, and the page shows nothing.
///
/// ```
/// use axum::Router;
/// use axum::http::HeaderMap;
/// use hypersig::{Action, Patches, RouterExt, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Greeting {
///     text: String,
/// }
///
/// const GREET: Action<Greeting> = Action::post("/greet");
///
/// async fn greet((): (), headers: HeaderMap, (): ()) -> Patches<Greeting> {
///     let agent = headers.get("user-agent").and_then(|value| value.to_str().ok());
///     Greeting::text().patch(format!("Hello, {}", agent.unwrap_or("stranger"))).into()
/// }
///
/// let _app: Router = Router::new().action(GREET, greet);
/// ```
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot serve this page or action",
    label = "not a handler of the router's state and the arguments `{A}`",
    note = "a handler is `fn(state, args)` or `fn(state, extractor, args)`, where the state is the router's and the extractor implements `FromRequestParts` for it"
)]
pub trait Handler<St, A, M>: Clone + Send + Sync + 'static + sealed::Sealed<St, A, M> {
    /// What the function returns: a future for a page or an action, a stream for a stream.
    type Output;

    /// Runs the extractor, if the function takes one, then the function.
    ///
    /// # Errors
    ///
    /// Returns the extractor's rejection as a response when it fails.
    fn call(
        self,
        state: St,
        request: Parts,
        args: A,
    ) -> impl Future<Output = Result<Self::Output, Response>> + Send;
}

mod sealed {
    pub trait Sealed<St, A, M> {}
}

impl<St, A, O, F> sealed::Sealed<St, A, (St, A)> for F where F: Fn(St, A) -> O {}

impl<St, A, O, F> Handler<St, A, (St, A)> for F
where
    F: Fn(St, A) -> O + Clone + Send + Sync + 'static,
    O: Send,
    St: Send + 'static,
    A: Send + 'static,
{
    type Output = O;

    fn call(
        self,
        state: St,
        _request: Parts,
        args: A,
    ) -> impl Future<Output = Result<O, Response>> + Send {
        std::future::ready(Ok(self(state, args)))
    }
}

impl<St, E, A, O, F> sealed::Sealed<St, A, (St, E, A)> for F where F: Fn(St, E, A) -> O {}

// The arguments come after the extractor because they are read from the body
// or the URL, as an axum handler takes its body last.
impl<St, E, A, O, F> Handler<St, A, (St, E, A)> for F
where
    F: Fn(St, E, A) -> O + Clone + Send + Sync + 'static,
    E: FromRequestParts<St> + Send,
    St: Send + Sync + 'static,
    A: Send + 'static,
{
    type Output = O;

    async fn call(self, state: St, mut request: Parts, args: A) -> Result<O, Response> {
        let extracted = E::from_request_parts(&mut request, &state)
            .await
            .map_err(IntoResponse::into_response)?;
        Ok(self(state, extracted, args))
    }
}

/// What a page handler returns: a page, a redirect, or a 404.
///
/// A handler that only ever renders returns `Page` or `Option<Page>`, which
/// convert into this. One that sometimes sends the browser elsewhere returns
/// this type, built from either. A pair of [`IntoResponseParts`] and one of
/// these adds headers, such as a `Set-Cookie`. A `Result` whose error is
/// [`IntoResponse`] answers with the error, so a handler can use `?`.
///
/// ```
/// use hypersig::html::h1;
/// use hypersig::{Page, PageResponse, Route, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Profile {
///     name: String,
/// }
///
/// # async fn login((): (), (): ()) -> Option<Page<Profile>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const LOGIN: Route = "/login" => login;
/// }
///
/// fn profile(signed_in: bool) -> PageResponse {
///     if !signed_in {
///         return PageResponse::redirect(LOGIN.url(()));
///     }
///     let signals = Profile { name: "Ada".to_owned() };
///     Page::new("Profile", signals).child(h1().bind_text(Profile::name())).into()
/// }
/// # let _ = (profile(true), pages);
/// ```
#[derive(Debug)]
pub struct PageResponse(Response);

impl PageResponse {
    /// Sends the browser to `url` with a `303 See Other`.
    #[must_use]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "callers build the URL in the call, as `redirect(LOGIN.url(()))`"
    )]
    pub fn redirect(url: Url) -> Self {
        Self(Redirect::to(url.as_str()).into_response())
    }

    /// Answers with a `404 Not Found`.
    #[must_use]
    pub fn not_found() -> Self {
        Self(StatusCode::NOT_FOUND.into_response())
    }
}

impl<S: Signals> From<Page<S>> for PageResponse {
    fn from(page: Page<S>) -> Self {
        Self(Html(page.render()).into_response())
    }
}

impl<S: Signals> From<Option<Page<S>>> for PageResponse {
    fn from(page: Option<Page<S>>) -> Self {
        page.map_or_else(Self::not_found, Self::from)
    }
}

impl<H: IntoResponseParts, R: Into<PageResponse>> From<(H, R)> for PageResponse {
    fn from((parts, page): (H, R)) -> Self {
        Self((parts, page.into().0).into_response())
    }
}

impl<R: Into<PageResponse>, E: IntoResponse> From<Result<R, E>> for PageResponse {
    fn from(result: Result<R, E>) -> Self {
        result.map_or_else(|error| Self(error.into_response()), Into::into)
    }
}

impl IntoResponse for PageResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

/// What an action handler returns: patches for the page whose signals are `S`.
///
/// [`Patches`] and [`SignalPatch`] convert into this and go out as
/// Server-Sent Events. A pair of [`IntoResponseParts`] and one of these adds
/// headers, such as a `Set-Cookie`. A `Result` whose error is
/// [`IntoResponse`] answers with the error, so a handler can use `?`. The
/// runtime treats a status outside 2xx as a failure and writes the status
/// and the body to the signal the page gave
/// [`Call::error`](crate::Call::error). An error response with a 2xx status
/// reaches the page as an answer with no patches.
///
/// ```
/// use axum::http::StatusCode;
/// use hypersig::{Action, Patches, RouterExt, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Notes {
///     count: i32,
/// }
///
/// const ADD: Action<Notes, i32> = Action::post("/add");
///
/// async fn add((): (), count: i32) -> Result<Patches<Notes>, (StatusCode, String)> {
///     let full = || (StatusCode::CONFLICT, "Too many notes.".to_owned());
///     let next = count.checked_add(1).ok_or_else(full)?;
///     Ok(Notes::count().patch(next).into())
/// }
///
/// let _app: axum::Router = axum::Router::new().action(ADD, add);
/// ```
pub struct ActionResponse<S> {
    response: Response,
    marker: PhantomData<fn() -> S>,
}

impl<S> ActionResponse<S> {
    fn new(response: Response) -> Self {
        Self {
            response,
            marker: PhantomData,
        }
    }
}

impl<S> fmt::Debug for ActionResponse<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ActionResponse")
            .field(&self.response)
            .finish()
    }
}

impl<S: Signals> From<Patches<S>> for ActionResponse<S> {
    fn from(patches: Patches<S>) -> Self {
        let events =
            stream::iter(patches.into_events()).map(|event| Ok::<_, Infallible>(to_sse(&event)));
        Self::new(Sse::new(events).into_response())
    }
}

impl<S: Signals> From<SignalPatch<S>> for ActionResponse<S> {
    fn from(patch: SignalPatch<S>) -> Self {
        Patches::from(patch).into()
    }
}

impl<S, H: IntoResponseParts, R: Into<ActionResponse<S>>> From<(H, R)> for ActionResponse<S> {
    fn from((parts, reply): (H, R)) -> Self {
        Self::new((parts, reply.into().response).into_response())
    }
}

impl<S, R: Into<ActionResponse<S>>, E: IntoResponse> From<Result<R, E>> for ActionResponse<S> {
    fn from(result: Result<R, E>) -> Self {
        result.map_or_else(|error| Self::new(error.into_response()), Into::into)
    }
}

impl<S> IntoResponse for ActionResponse<S> {
    fn into_response(self) -> Response {
        self.response
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
