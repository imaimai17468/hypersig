//! Serving actions and the runtime from an axum [`Router`].

mod dev;

use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::Router;
use axum::body::{Body, Bytes};
use axum::extract::rejection::RawPathParamsRejection;
use axum::extract::{FromRequestParts, MatchedPath, Query, RawPathParams, Request, State};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::sse::{Event as SseEvent, Sse};
use axum::response::{Html, IntoResponse, IntoResponseParts, Redirect, Response};
use axum::routing::{MethodRouter, get, post};
use futures::{Stream, StreamExt as _, stream};
use serde::de::DeserializeOwned;

use crate::action::{Action, Method, StreamAction};
use crate::exact;
use crate::page::{Framing, Page};
use crate::patch::{ListPatch, PartMismatch, PatchEvent, Patches, SignalPatch};
use crate::route::{Base, Params, Route, Search, Url};
use crate::signal::{Signals, current_part, in_part};
use crate::upload::{ARGS_PART, FILES_PART, Upload, UploadStream, Uploaded, UploadedFile};

/// Adds the browser runtime to an axum [`Router`].
///
/// The pages, actions, streams, and uploads are registered by the function
/// that [`routes!`](crate::routes) generates, which calls this trait's
/// hidden methods.
pub trait RouterExt<St>: Sized {
    /// Serves the browser runtime at [`RUNTIME_PATH`](crate::RUNTIME_PATH).
    ///
    /// The response tells the browser to keep the file for a year without
    /// asking again, which holds because the path changes with the file. A
    /// request whose `Accept-Encoding` names `br` receives the file
    /// compressed with Brotli at build time, so the server compresses nothing.
    /// On wasm32-unknown-unknown the file goes out uncompressed, because the
    /// Cloudflare Workers runtime compresses a body that names a
    /// `Content-Encoding` again.
    ///
    /// A debug build also serves a stream at `/_hypersig/dev` that each page
    /// it serves keeps open. When the server stops and another starts in
    /// its place, as `cargo hypersig dev` restarts it after a change, the
    /// page reloads from the new one. The page closes each request to the
    /// stream after two seconds and opens another, so a server that shuts
    /// down gracefully waits for it no longer than that. A release build
    /// serves no stream, and its pages open none.
    #[must_use]
    fn hypersig_runtime(self) -> Self;

    /// Serves the page at `route` on `GET`. `routes!` calls it for each page it declares.
    #[doc(hidden)]
    #[must_use]
    fn page<P, Q, H, M, Fut, R>(
        self,
        route: Route<P, Q>,
        handler: H,
        unserved: Option<UnservedPage<St>>,
    ) -> Self
    where
        P: Params + Send + 'static,
        Q: Search + DeserializeOwned + Send + 'static,
        Q::Args<P>: Send,
        H: Handler<St, Q::Args<P>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = R> + Send + 'static,
        R: Into<PageResponse>;

    /// Serves `action` with `handler`. `routes!` calls it for each `action` line.
    #[doc(hidden)]
    #[must_use]
    fn action<S, A, H, M, Fut, P>(self, action: Action<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, A, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: IntoActionResponse<S>;

    /// Serves `stream` on `GET` with `handler`. `routes!` calls it for each `stream` line.
    #[doc(hidden)]
    #[must_use]
    fn stream<S, A, H, M, Str>(self, stream: StreamAction<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, A, M, Output = Str>,
        M: 'static,
        Str: PatchStream<S>;

    /// Serves `upload` with `handler`. `routes!` calls it for each `upload` line.
    #[doc(hidden)]
    #[must_use]
    fn upload<S, A, H, M, Fut, P>(self, upload: Upload<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, Uploaded<A>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: IntoActionResponse<S>;

    /// Serves `upload` with `handler`. `routes!` calls it for each `upload` line with `stream`.
    #[doc(hidden)]
    #[must_use]
    fn upload_stream<S, A, H, M, Fut, P>(self, upload: Upload<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, UploadStream<A>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: IntoActionResponse<S>;
}

/// The runtime's file, compressed with Brotli where `headers` accept it.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn runtime_file(headers: &HeaderMap) -> Response {
    let head = [
        (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
        // The path names a hash of the file, so a changed runtime is served
        // at another path.
        (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        (header::VARY, "accept-encoding"),
    ];
    if accepts_brotli(headers) {
        (head, [(header::CONTENT_ENCODING, "br")], crate::RUNTIME_BR).into_response()
    } else {
        (head, crate::RUNTIME_JS).into_response()
    }
}

/// The runtime's file, uncompressed.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn runtime_file(_headers: &HeaderMap) -> Response {
    let head = [
        (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
        // The path names a hash of the file, so a changed runtime is served
        // at another path.
        (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
    ];
    (head, crate::RUNTIME_JS).into_response()
}

/// Whether `Accept-Encoding` names `br` with a weight above zero.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn accepts_brotli(headers: &HeaderMap) -> bool {
    headers
        .get_all(header::ACCEPT_ENCODING)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(','))
        .any(|coding| {
            let mut params = coding.split(';');
            let name = params.next().unwrap_or("").trim();
            name.eq_ignore_ascii_case("br")
                && params.all(|param| {
                    let (key, weight) = param.split_once('=').unwrap_or((param, ""));
                    !key.trim().eq_ignore_ascii_case("q")
                        || weight.trim().parse::<f32>().is_ok_and(|q| q > 0.0)
                })
        })
}

impl<St: Clone + Send + Sync + 'static> RouterExt<St> for Router<St> {
    fn hypersig_runtime(self) -> Self {
        dev::route(self.route(
            crate::RUNTIME_PATH,
            get(|headers: HeaderMap| async move { runtime_file(&headers) }),
        ))
    }

    fn page<P, Q, H, M, Fut, R>(
        self,
        route: Route<P, Q>,
        handler: H,
        unserved: Option<UnservedPage<St>>,
    ) -> Self
    where
        P: Params + Send + 'static,
        Q: Search + DeserializeOwned + Send + 'static,
        Q::Args<P>: Send,
        H: Handler<St, Q::Args<P>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = R> + Send + 'static,
        R: Into<PageResponse>,
    {
        let path = route.path();
        let base = route.base();
        // The page is served at its path behind a proxy that strips the
        // base, and at this one inside a router nested at the base.
        let under_base = base.join(path);
        let route_handler = move |State(state): State<St>,
                                  matched: MatchedPath,
                                  params: Result<RawPathParams, RawPathParamsRejection>,
                                  request: Parts| {
            let handler = handler.clone();
            let unserved = unserved.clone();
            let nested = matched.as_str() != path && matched.as_str() != under_base;
            // A segment that does not decode to UTF-8 names no page, so the
            // router's page for a 404 answers it, as it answers a segment
            // that does not parse.
            let params = params.ok().and_then(|params| {
                let segments: Vec<&str> = params.iter().map(|(_, value)| value).collect();
                P::from_segments(&segments)
            });
            let search = Q::unit().map_or_else(
                || crate::query::parse_query::<Q>(request.uri.query().unwrap_or_default()),
                Ok,
            );
            let served_at = matched.as_str().to_owned();
            async move {
                if nested {
                    let links = match base.as_str() {
                        "" => format!("`{path}`"),
                        base => format!("`{base}` and `{path}`"),
                    };
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!(
                            "the page whose links point to {links} is registered on a nested router, which serves it at `{served_at}`"
                        ),
                    )
                        .into_response();
                }
                let refused = match (params, search) {
                    (None, _) => Err(Reason::Segments),
                    (Some(_), Err(rejection)) => Err(Reason::Query(format!(
                        "Failed to deserialize query string: {rejection}"
                    ))),
                    (Some(params), Ok(search)) => Ok(Q::join(params, search)),
                };
                let args = match (refused, &unserved) {
                    (Ok(args), _) => args,
                    (Err(reason), Some(page)) => {
                        let unserved = Unserved::new(reason, &request);
                        return page.answer(state, request, unserved).await;
                    }
                    (Err(Reason::Query(text)), None) => {
                        return (StatusCode::BAD_REQUEST, text).into_response();
                    }
                    (Err(_), None) => return StatusCode::NOT_FOUND.into_response(),
                };
                // The router's page for a 404 reads the request, which the
                // handler takes, so it keeps a copy only where there is one.
                let kept = unserved
                    .as_ref()
                    .map(|page| (page, state.clone(), request.clone()));
                match handler.call(state, request, ready(args)).await {
                    Ok(page) => {
                        let kept = kept.map(|(page, state, request)| Unanswered {
                            page,
                            state,
                            request,
                        });
                        page.await.into().finish(base, kept).await
                    }
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
        P: IntoActionResponse<S>,
    {
        let method = action.method();
        let route = move |State(state): State<St>,
                          Query(query): Query<HashMap<String, String>>,
                          request: Parts,
                          body: Bytes| {
            let handler = handler.clone();
            in_called_part(called_part(&request.headers), async move {
                let args = read_args::<A>(method, &request.headers, &query, &body)?;
                Ok::<_, RequestError>(match handler.call(state, request, ready(args)).await {
                    Ok(reply) => reply.await.into_action_response().into_response(),
                    Err(rejection) => rejection,
                })
            })
        };
        self.route(action.path(), method_router(method, route))
    }

    fn stream<S, A, H, M, Str>(self, stream: StreamAction<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, A, M, Output = Str>,
        M: 'static,
        Str: PatchStream<S>,
    {
        let method = Method::Get;
        let route = move |State(state): State<St>,
                          Query(query): Query<HashMap<String, String>>,
                          request: Parts,
                          body: Bytes| {
            let handler = handler.clone();
            in_called_part(called_part(&request.headers), async move {
                let args = read_args::<A>(method, &request.headers, &query, &body)?;
                let patches = match handler.call(state, request, ready(args)).await {
                    Ok(patches) => patches,
                    Err(rejection) => return Ok(rejection),
                };
                // The stream is read after this future returns, so it
                // builds and writes its patches under the part on its own.
                // An event it cannot write ends the response with an error,
                // which closes the stream before `hs-end`.
                let events = InPart {
                    part: current_part(),
                    inner: Box::pin(
                        patches
                            .flat_map(|patches| stream::iter(patches.into_events()))
                            .map(|event| to_sse(&event))
                            .chain(stream::iter([Ok(end_of_stream())])),
                    ),
                };
                Ok::<_, RequestError>(keep_alive(Sse::new(events)).into_response())
            })
        };
        self.route(stream.path(), method_router(method, route))
    }

    fn upload<S, A, H, M, Fut, P>(self, upload: Upload<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, Uploaded<A>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: IntoActionResponse<S>,
    {
        let limit = upload.limit();
        let route = move |State(state): State<St>, request: Request| {
            let handler = handler.clone();
            in_called_part(called_part(request.headers()), async move {
                let (request, body) = request.into_parts();
                let multipart = open_upload_stream(&request.headers, body, limit)?;
                let uploaded = async move {
                    read_upload::<A>(multipart)
                        .await
                        .map_err(IntoResponse::into_response)
                };
                Ok::<_, RequestError>(match handler.call(state, request, uploaded).await {
                    Ok(reply) => reply.await.into_action_response().into_response(),
                    Err(rejection) => rejection,
                })
            })
        };
        // `open_upload_stream` holds the limit, as it does for `upload_stream`.
        self.route(upload.path(), post(route))
    }

    fn upload_stream<S, A, H, M, Fut, P>(self, upload: Upload<S, A>, handler: H) -> Self
    where
        S: Signals,
        A: DeserializeOwned + Send + 'static,
        H: Handler<St, UploadStream<A>, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = P> + Send + 'static,
        P: IntoActionResponse<S>,
    {
        let limit = upload.limit();
        let route = move |State(state): State<St>, request: Request| {
            let handler = handler.clone();
            in_called_part(called_part(request.headers()), async move {
                let (request, body) = request.into_parts();
                let mut multipart = open_upload_stream(&request.headers, body, limit)?;
                let uploaded = async move {
                    let args = read_args_part::<A>(&mut multipart)
                        .await
                        .map_err(IntoResponse::into_response)?;
                    Ok(UploadStream::new(args, multipart))
                };
                Ok::<_, RequestError>(match handler.call(state, request, uploaded).await {
                    Ok(reply) => reply.await.into_action_response().into_response(),
                    Err(rejection) => rejection,
                })
            })
        };
        // The body is read as a stream, which `DefaultBodyLimit` does not
        // reach, so the multer constraint in `open_upload_stream` holds the
        // limit.
        self.route(upload.path(), post(route))
    }
}

/// A function that serves a page, an action, or a stream.
///
/// Name it after `=>` on a line of [`routes!`](crate::routes).
///
/// It is implemented for `Fn(St, A) -> O` and for `Fn(St, E, A) -> O`, where
/// `St` is the router state, `A` the page's parameters or the action's
/// arguments, and `E` a [`FromRequestParts`] extractor that runs before the
/// function, such as [`HeaderMap`] or the application's own signed-in user.
/// A tuple of extractors counts as one. When the extractor fails, its
/// rejection is the response and the function does not run. `M` tells the
/// two forms apart, and the compiler infers it.
///
/// An extractor that guards an action should refuse with a status outside
/// 2xx, which the page's [`Call::error`](crate::Call::error) shows. The
/// browser's `fetch` follows a redirect, so an action refused with one hands
/// the page the HTML it points to, and the page shows nothing.
///
/// ```
/// use axum::Router;
/// use axum::http::HeaderMap;
/// use hypersig::{Action, Patches, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Greeting {
///     text: String,
/// }
///
/// hypersig::routes! {
///     fn actions(Router<()>);
///     action GREET: Action<Greeting> = post "/greet" => greet;
/// }
///
/// async fn greet((): (), headers: HeaderMap, (): ()) -> Patches<Greeting> {
///     let agent = headers.get("user-agent").and_then(|value| value.to_str().ok());
///     Greeting::text().patch(format!("Hello, {}", agent.unwrap_or("stranger"))).into()
/// }
///
/// let _app: Router = actions(Router::new());
/// ```
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot serve this page or action",
    label = "not a handler of the router's state and the arguments `{A}`",
    note = "a handler is `fn(state, args)` or `fn(state, extractor, args)`, where the state is the router's and the extractor implements `FromRequestParts` for it"
)]
pub trait Handler<St, A, M>: Clone + Send + Sync + 'static + sealed::Sealed<St, A, M> {
    /// What the function returns: a future for a page or an action, a stream for a stream.
    type Output;

    /// Runs the extractor, if the function takes one, then awaits `args` and
    /// runs the function.
    ///
    /// `args` is awaited after the extractor, so an upload whose extractor
    /// refuses the request is answered before its body is read.
    ///
    /// # Errors
    ///
    /// Returns the extractor's rejection, or the error `args` ends with, as a
    /// response.
    fn call(
        self,
        state: St,
        request: Parts,
        args: impl Future<Output = Result<A, Response>> + Send,
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

    async fn call(
        self,
        state: St,
        _request: Parts,
        args: impl Future<Output = Result<A, Response>> + Send,
    ) -> Result<O, Response> {
        Ok(self(state, args.await?))
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

    async fn call(
        self,
        state: St,
        mut request: Parts,
        args: impl Future<Output = Result<A, Response>> + Send,
    ) -> Result<O, Response> {
        let extracted = E::from_request_parts(&mut request, &state)
            .await
            .map_err(IntoResponse::into_response)?;
        Ok(self(state, extracted, args.await?))
    }
}

/// What a page handler returns: a page, a page with a status, a redirect, or a 404.
///
/// A handler that only ever renders returns `Page` or `Option<Page>`, which
/// convert into this. One that sometimes sends the browser elsewhere returns
/// this type, built from either. A pair of [`IntoResponseParts`] and one of
/// these adds headers, such as a `Set-Cookie`. A `Result` whose error is
/// [`IntoResponse`] answers with the error, so a handler can use `?`.
///
/// A 404 from `None` or [`PageResponse::not_found`] shows the page of the
/// handler [`routes!`](crate::routes) names after `else`, and has an empty
/// body where it names none.
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
pub struct PageResponse {
    reply: Reply,
    /// Header parts added around the reply, innermost first.
    parts: Vec<AddParts>,
}

/// Adds the header parts of a pair to the response the reply becomes.
type AddParts = Box<dyn FnOnce(Response) -> Response + Send>;

/// Renders a page under a base, with the URL of the stream that reloads it, if any.
type Render = Box<dyn FnOnce(Base, Option<&str>) -> String + Send>;

enum Reply {
    /// A page, rendered once the router knows where it is served.
    Page {
        render: Render,
        status: Option<StatusCode>,
        framing: Framing,
    },
    /// A 404 that shows the router's page for a request it serves no page for.
    NotFound,
    Response(Response),
}

impl fmt::Debug for PageResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("PageResponse");
        match &self.reply {
            Reply::Page { status, .. } => tuple.field(&format_args!("Page({status:?})")),
            Reply::NotFound => tuple.field(&format_args!("NotFound")),
            Reply::Response(response) => tuple.field(response),
        };
        tuple.finish()
    }
}

impl PageResponse {
    fn new(reply: Reply) -> Self {
        Self {
            reply,
            parts: Vec::new(),
        }
    }

    /// Sends the browser to `url` with a `303 See Other`.
    #[must_use]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "callers build the URL in the call, as `redirect(LOGIN.url(()))`"
    )]
    pub fn redirect(url: Url) -> Self {
        Self::new(Reply::Response(Redirect::to(url.as_str()).into_response()))
    }

    /// Answers with a `404 Not Found`.
    #[must_use]
    pub fn not_found() -> Self {
        Self::new(Reply::NotFound)
    }

    /// Answers with `page` and the status `status`, such as a styled `404 Not Found`.
    ///
    /// ```
    /// use axum::http::StatusCode;
    /// use hypersig::html::p;
    /// use hypersig::{Page, PageResponse, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Missing {}
    ///
    /// let page = Page::new("Not found", Missing {}).child(p().text("No such order."));
    /// let _response = PageResponse::with_status(StatusCode::NOT_FOUND, page);
    /// ```
    #[must_use]
    pub fn with_status<S: Signals + Send + 'static>(status: StatusCode, page: Page<S>) -> Self {
        Self::page(page, Some(status))
    }

    /// The reply that renders `page`, answering with `status` where there is one.
    fn page<S: Signals + Send + 'static>(page: Page<S>, status: Option<StatusCode>) -> Self {
        Self::new(Reply::Page {
            framing: page.frame_policy().clone(),
            render: Box::new(move |base, dev| page.render_under(base, dev)),
            status,
        })
    }

    /// Gives a page without a status of its own the status `status`.
    fn or_status(mut self, status: StatusCode) -> Self {
        if let Reply::Page { status: own, .. } = &mut self.reply {
            own.get_or_insert(status);
        }
        self
    }

    /// The response of a page under `base`, with `unserved` answering a 404
    /// where the router names a page for it.
    async fn finish<St: Clone + Send + Sync + 'static>(
        self,
        base: Base,
        unserved: Option<Unanswered<'_, St>>,
    ) -> Response {
        let Self { reply, parts } = self;
        let response = match (reply, unserved) {
            (
                Reply::NotFound,
                Some(Unanswered {
                    page,
                    state,
                    request,
                }),
            ) => {
                let unserved = Unserved::new(Reason::NoPage, &request);
                Box::pin(page.answer(state, request, unserved)).await
            }
            (reply, _) => reply.into_response(base),
        };
        parts
            .into_iter()
            .fold(response, |response, add| add(response))
    }
}

/// What a 404 needs to show the page the router names for it.
struct Unanswered<'a, St> {
    page: &'a UnservedPage<St>,
    state: St,
    request: Parts,
}

impl Reply {
    /// The response of a page under `base`, where a 404 has an empty body.
    fn into_response(self, base: Base) -> Response {
        match self {
            Self::Page {
                render,
                status,
                framing,
            } => {
                let html = render(base, dev::url(base).as_deref());
                let mut response = (status.unwrap_or(StatusCode::OK), Html(html)).into_response();
                let headers = response.headers_mut();
                if let Some(value) = framing.x_frame_options() {
                    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static(value));
                }
                if let Some(policy) = framing.content_security_policy() {
                    // `Origin` holds only the bytes of a host and a port, which a header can hold.
                    let policy = HeaderValue::try_from(policy).expect("an origin is header text");
                    headers.insert(header::CONTENT_SECURITY_POLICY, policy);
                }
                response
            }
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            Self::Response(response) => response,
        }
    }
}

impl<S: Signals + Send + 'static> From<Page<S>> for PageResponse {
    fn from(page: Page<S>) -> Self {
        Self::page(page, None)
    }
}

impl<S: Signals + Send + 'static> From<Option<Page<S>>> for PageResponse {
    fn from(page: Option<Page<S>>) -> Self {
        page.map_or_else(Self::not_found, Self::from)
    }
}

impl<H: IntoResponseParts + Send + 'static, R: Into<PageResponse>> From<(H, R)> for PageResponse {
    fn from((parts, page): (H, R)) -> Self {
        let mut response = page.into();
        response
            .parts
            .push(Box::new(move |inner| (parts, inner).into_response()));
        response
    }
}

impl<R: Into<PageResponse>, E: IntoResponse> From<Result<R, E>> for PageResponse {
    fn from(result: Result<R, E>) -> Self {
        result.map_or_else(
            |error| Self::new(Reply::Response(error.into_response())),
            Into::into,
        )
    }
}

impl IntoResponse for PageResponse {
    fn into_response(self) -> Response {
        let Self { reply, parts } = self;
        parts
            .into_iter()
            .fold(reply.into_response(Base::ROOT), |response, add| {
                add(response)
            })
    }
}

/// A page request that reached no page, handed to the `else` handler of [`routes!`](crate::routes).
///
/// Its [`status`](Unserved::status) is `404 Not Found` for a path no route
/// matches, a path whose segments do not parse into its route's
/// parameters, and a handler that answers `None`, and `400 Bad Request` for
/// a query string that does not deserialize into the route's search
/// parameters. A page the handler returns is sent with that status, unless
/// it comes from [`PageResponse::with_status`].
#[derive(Debug)]
pub struct Unserved {
    reason: Reason,
    path: String,
}

#[derive(Debug)]
enum Reason {
    NoRoute,
    Segments,
    NoPage,
    Query(String),
}

impl Unserved {
    fn new(reason: Reason, request: &Parts) -> Self {
        Self {
            reason,
            path: request.uri.path().to_owned(),
        }
    }

    /// The status the page for this request is sent with: 404 or 400.
    #[must_use]
    pub fn status(&self) -> StatusCode {
        match self.reason {
            Reason::NoRoute | Reason::Segments | Reason::NoPage => StatusCode::NOT_FOUND,
            Reason::Query(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// The parameters of `route`, where the request's path starts with that route's path.
    ///
    /// A `{name}` segment of `route` takes the request's segment at its
    /// place, percent-decoded, and a literal segment must be equal. A page
    /// for a missing path under `/{lang}` reads its language this way.
    ///
    /// ```
    /// use hypersig::html::p;
    /// use hypersig::{IntlLocale, Page, Route, Signals, Unserved};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Missing {}
    ///
    /// # async fn home((): (), _: String) -> Option<Page<Missing>> { None }
    /// hypersig::routes! {
    ///     fn pages(Router<()>) else missing;
    ///     const HOME: Route<String> = "/{lang}" => home;
    /// }
    ///
    /// async fn missing((): (), unserved: Unserved) -> Page<Missing> {
    ///     let lang = unserved.within(HOME).unwrap_or_else(|| "en".to_owned());
    ///     let (locale, text) = if lang == "ja" {
    ///         (IntlLocale::Ja, "ページが見つかりません。")
    ///     } else {
    ///         (IntlLocale::En, "No such page.")
    ///     };
    ///     Page::new(text, Missing {}).lang(locale).child(p().text(text))
    /// }
    /// # let _ = pages;
    /// ```
    #[must_use]
    pub fn within<P: Params, Q: Search>(&self, route: Route<P, Q>) -> Option<P> {
        route.prefix_of(&self.path)
    }
}

impl fmt::Display for Unserved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = &self.path;
        match &self.reason {
            Reason::NoRoute => write!(f, "no page is served at `{path}`"),
            Reason::Segments => write!(
                f,
                "a segment of `{path}` does not parse into its route's parameters"
            ),
            Reason::NoPage => write!(f, "the handler of `{path}` answered with no page"),
            Reason::Query(rejection) => f.write_str(rejection),
        }
    }
}

/// The handler [`routes!`](crate::routes) names after `else`, which each page and the router's fallback keep.
#[doc(hidden)]
pub struct UnservedPage<St> {
    handler: Arc<UnservedFn<St>>,
    /// The base of the routes, which the page's runtime path starts with.
    base: Base,
}

/// The handler behind an [`UnservedPage`], with its types erased.
type UnservedFn<St> = dyn Fn(St, Parts, Unserved) -> Pin<Box<dyn Future<Output = Result<PageResponse, Response>> + Send>>
    + Send
    + Sync;

impl<St> Clone for UnservedPage<St> {
    fn clone(&self) -> Self {
        Self {
            handler: Arc::clone(&self.handler),
            base: self.base,
        }
    }
}

impl<St> fmt::Debug for UnservedPage<St> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UnservedPage")
    }
}

impl<St: Clone + Send + Sync + 'static> UnservedPage<St> {
    /// Keeps `handler` to answer a page request that reached no page.
    #[doc(hidden)]
    pub fn new<H, M, Fut, R>(base: Base, handler: H) -> Self
    where
        H: Handler<St, Unserved, M, Output = Fut>,
        M: 'static,
        Fut: Future<Output = R> + Send + 'static,
        R: Into<PageResponse>,
    {
        let handler: Arc<UnservedFn<St>> = Arc::new(move |state, request, unserved| {
            let handler = handler.clone();
            Box::pin(async move {
                let page = handler.call(state, request, ready(unserved)).await?;
                Ok(page.await.into())
            })
        });
        Self { handler, base }
    }

    /// The handler's page, sent with the status of `unserved` unless it sets its own.
    async fn answer(&self, state: St, request: Parts, unserved: Unserved) -> Response {
        let status = unserved.status();
        match (self.handler)(state, request, unserved).await {
            // A handler here that answers `None` has no page to fall back on.
            Ok(page) => page.or_status(status).finish::<St>(self.base, None).await,
            Err(rejection) => rejection,
        }
    }

    /// Serves the handler's page as the fallback of `router`, for a path no route matches.
    #[doc(hidden)]
    pub fn fallback(self, router: Router<St>) -> Router<St> {
        router.fallback(move |State(state): State<St>, request: Parts| {
            let page = self.clone();
            async move {
                // The runtime could not apply a page, so a call to an action
                // that does not exist gets the bare status.
                if RequestKind::of(&request) == RequestKind::Runtime {
                    return StatusCode::NOT_FOUND.into_response();
                }
                let unserved = Unserved::new(Reason::NoRoute, &request);
                page.answer(state, request, unserved).await
            }
        })
    }
}

/// What sent a request: the browser loading a page, or the runtime calling an action.
///
/// The runtime asks for `text/event-stream` when it calls an action, a
/// stream, or an upload, and a browser loading a page asks for HTML. An
/// extractor that refuses a visitor reads it to choose the refusal: a page
/// can send the browser to a sign-in page, and an action answers with a
/// status the page shows through [`Call::error`](crate::Call::error),
/// because `fetch` would follow a redirect and hand the page HTML it cannot
/// apply.
///
/// ```
/// use axum::http::{Request, header};
/// use hypersig::RequestKind;
///
/// let (parts, ()) = Request::post("/notes")
///     .header(header::ACCEPT, "text/event-stream")
///     .body(())
///     .expect("request")
///     .into_parts();
/// assert_eq!(RequestKind::of(&parts), RequestKind::Runtime);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestKind {
    /// The browser loading a page, by following a link or opening a URL.
    Navigation,
    /// The runtime calling an action, a stream, or an upload.
    Runtime,
}

impl RequestKind {
    /// The kind of the request whose head is `request`.
    #[must_use]
    pub fn of(request: &Parts) -> Self {
        if from_runtime(&request.headers) {
            Self::Runtime
        } else {
            Self::Navigation
        }
    }
}

/// Whether `headers` are those of a request the runtime or `EventSource`
/// sent, which accept `text/event-stream` and nothing else.
fn from_runtime(headers: &HeaderMap) -> bool {
    headers
        .get_all(header::ACCEPT)
        .iter()
        .any(|accept| accept.as_bytes() == b"text/event-stream")
}

impl<St: Sync> FromRequestParts<St> for RequestKind {
    type Rejection = Infallible;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &St,
    ) -> impl Future<Output = Result<Self, Infallible>> + Send {
        std::future::ready(Ok(Self::of(parts)))
    }
}

/// What an action handler returns: patches for the page whose signals are `S`.
///
/// [`Patches`], [`SignalPatch`], and [`ListPatch`] convert into this and go out as
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
/// use hypersig::{Action, Patches, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Notes {
///     count: i32,
/// }
///
/// hypersig::routes! {
///     fn actions(Router<()>);
///     action ADD: Action<Notes, i32> = post "/add" => add;
/// }
///
/// async fn add((): (), count: i32) -> Result<Patches<Notes>, (StatusCode, String)> {
///     let full = || (StatusCode::CONFLICT, "Too many notes.".to_owned());
///     let next = count.checked_add(1).ok_or_else(full)?;
///     Ok(Notes::count().patch(next).into())
/// }
///
/// let _app: axum::Router = actions(axum::Router::new());
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
    /// The response is `500 Internal Server Error` for a fragment of a part
    /// rendered outside the handler answering that part, as
    /// [`Patches::element`] describes.
    fn from(patches: Patches<S>) -> Self {
        // The events are written now, while the handler's part is current,
        // rather than when the body is read.
        let events: Result<Vec<_>, _> = patches
            .into_events()
            .iter()
            .map(|event| to_sse(event).map(Ok::<_, Infallible>))
            .collect();
        Self::new(match events {
            Ok(events) => Sse::new(stream::iter(events)).into_response(),
            Err(mismatch) => {
                (StatusCode::INTERNAL_SERVER_ERROR, mismatch.to_string()).into_response()
            }
        })
    }
}

impl<S: Signals> From<SignalPatch<S>> for ActionResponse<S> {
    fn from(patch: SignalPatch<S>) -> Self {
        Patches::from(patch).into()
    }
}

impl<S: Signals> From<ListPatch<S>> for ActionResponse<S> {
    fn from(patch: ListPatch<S>) -> Self {
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

/// A value an action's handler returns for the page whose signals are `S`.
///
/// It is implemented for every type that converts into an
/// [`ActionResponse<S>`]: [`Patches`], a [`SignalPatch`], a [`ListPatch`], a
/// tuple of response parts and one of them, and a `Result` of one of them
/// whose error implements [`IntoResponse`].
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a reply of an action on the page whose signals are `{S}`",
    note = "an action's handler returns `Patches<{S}>`, a `SignalPatch<{S}>`, a `ListPatch<{S}>`, a tuple of response parts and one of those, or a `Result` of one of those whose error implements `IntoResponse`"
)]
pub trait IntoActionResponse<S> {
    /// Converts `self` into the response the action answers with.
    fn into_action_response(self) -> ActionResponse<S>;
}

// Without `do_not_recommend`, rustc reports the failed `From` bound below
// this impl, naming `ActionResponse` in place of the message above.
#[diagnostic::do_not_recommend]
impl<S, P: Into<ActionResponse<S>>> IntoActionResponse<S> for P {
    fn into_action_response(self) -> ActionResponse<S> {
        self.into()
    }
}

/// A stream of patch batches for the page whose signals are `S`.
///
/// It is implemented for every `Stream` of [`Patches<S>`] that is `Send` and
/// `'static`. A stream's handler is a plain `fn` that returns the stream,
/// such as one `futures::stream::unfold` builds. An `async fn` returns a
/// future of the stream, which does not implement it.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a stream of `Patches<{S}>` a stream's handler returns",
    note = "a stream's handler is a plain `fn` returning a `Stream<Item = Patches<{S}>> + Send + 'static`; an `async fn` returns a future, which is not a stream"
)]
pub trait PatchStream<S>: Stream<Item = Patches<S>> + Send + 'static {}

// Without `do_not_recommend`, rustc reports the failed `Stream` bound below
// this impl in place of the message above.
#[diagnostic::do_not_recommend]
impl<S, T: Stream<Item = Patches<S>> + Send + 'static> PatchStream<S> for T {}

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

/// Refuses a request from another site, a POST whose body is not JSON, and a
/// GET the runtime did not send, then reads the arguments from the body of a POST and from the `args`
/// query parameter of a GET.
fn read_args<A: DeserializeOwned>(
    method: Method,
    headers: &HeaderMap,
    query: &HashMap<String, String>,
    body: &[u8],
) -> Result<A, RequestError> {
    // A browser that sends no `Sec-Fetch-Site` still sends a preflight before
    // a cross-origin POST of JSON, and this server answers no preflight.
    refuse_cross_site(headers)?;
    match method {
        Method::Post => {
            let json = headers
                .get(header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.split(';').next())
                .is_some_and(|media| media.trim().eq_ignore_ascii_case("application/json"));
            if !json {
                return Err(RequestError::new(Refusal::NotJson));
            }
            exact::from_slice(if body.is_empty() { b"null" } else { body })
        }
        // An `<img>`, a link, or a form sends a GET with another `Accept`, so
        // they cannot call the action from a browser without
        // `Sec-Fetch-Site`. An `EventSource` on another site still sends this
        // one, which is why a GET action changes nothing.
        Method::Get if !from_runtime(headers) => {
            return Err(RequestError::new(Refusal::NotFromRuntime));
        }
        Method::Get => {
            exact::from_slice(query.get("args").map_or("null", String::as_str).as_bytes())
        }
    }
    .map_err(|source| RequestError::new(Refusal::Arguments(source)))
}

/// Refuses a request whose `Sec-Fetch-Site` names another origin.
fn refuse_cross_site(headers: &HeaderMap) -> Result<(), RequestError> {
    // `same-site` is refused too, because a cookie with `SameSite=Lax` rides
    // along on a request from a sibling subdomain.
    let site = headers.get("sec-fetch-site");
    if site.is_some_and(|site| site.as_bytes() != b"same-origin") {
        return Err(RequestError::new(Refusal::CrossSite));
    }
    Ok(())
}

/// The header the runtime adds to an upload. Its value is not read.
const UPLOAD_HEADER: &str = "hypersig-upload";

/// The header the runtime adds to a call made inside a [`Part`](crate::Part),
/// holding the dotted path of the page field the part sits at.
const PART_HEADER: &str = "hypersig-part";

/// The part a call came from, read from [`PART_HEADER`].
///
/// The path only places the answer in the caller's own page, so any field
/// names pass, and only a value that is not a dotted path is refused.
fn called_part(headers: &HeaderMap) -> Result<Option<Arc<str>>, RequestError> {
    let Some(value) = headers.get(PART_HEADER) else {
        return Ok(None);
    };
    let path = value
        .to_str()
        .ok()
        .filter(|path| {
            path.split('.').all(|field| {
                !field.is_empty()
                    && field
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
            })
        })
        .ok_or_else(|| RequestError::new(Refusal::Part))?;
    Ok(Some(path.into()))
}

/// Runs `future`, the response to one request, with `part`, the part the
/// request came from, current, or answers with the refusal of a malformed part.
fn in_called_part<F>(
    part: Result<Option<Arc<str>>, RequestError>,
    future: F,
) -> InPart<dyn Future<Output = Response> + Send>
where
    F: Future<Output: IntoResponse> + Send + 'static,
{
    let (part, inner): (_, Pin<Box<dyn Future<Output = Response> + Send>>) = match part {
        Ok(part) => (part, Box::pin(async move { future.await.into_response() })),
        Err(refusal) => (None, Box::pin(std::future::ready(refusal.into_response()))),
    };
    InPart { part, inner }
}

/// A future or a stream that runs with the paths of `part` current each
/// time it is polled, so the patches it builds and writes land under it.
///
/// The part is set and put back around every poll, because a task may move
/// between threads between two polls.
struct InPart<T: ?Sized> {
    part: Option<Arc<str>>,
    inner: Pin<Box<T>>,
}

impl<T: Future + ?Sized> Future for InPart<T> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T::Output> {
        let this = &mut *self;
        in_part(this.part.clone(), || this.inner.as_mut().poll(cx))
    }
}

impl<T: Stream + ?Sized> Stream for InPart<T> {
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T::Item>> {
        let this = &mut *self;
        in_part(this.part.clone(), || this.inner.as_mut().poll_next(cx))
    }
}

/// Arguments already read, as the future [`Handler::call`] awaits.
fn ready<A>(args: A) -> std::future::Ready<Result<A, Response>> {
    std::future::ready(Ok(args))
}

/// Reads the arguments from the first part of `multipart` and the files
/// from the `files` parts into memory, skipping any other part.
async fn read_upload<A: DeserializeOwned>(
    mut multipart: multer::Multipart<'static>,
) -> Result<Uploaded<A>, RequestError> {
    let args = read_args_part::<A>(&mut multipart).await?;
    let mut files = Vec::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(RequestError::from_multer)?
    {
        if field.name() != Some(FILES_PART) {
            continue;
        }
        let file_name = field.file_name().map(str::to_owned);
        let content_type = field.content_type().map(ToString::to_string);
        let bytes = field.bytes().await.map_err(RequestError::from_multer)?;
        files.push(UploadedFile::new(file_name, content_type, bytes.to_vec()));
    }
    Ok(Uploaded::new(args, files))
}

/// Reads the arguments from the first part of `multipart`, which the runtime
/// sends before the files.
async fn read_args_part<A: DeserializeOwned>(
    multipart: &mut multer::Multipart<'static>,
) -> Result<A, RequestError> {
    let field = multipart
        .next_field()
        .await
        .map_err(RequestError::from_multer)?
        .ok_or_else(RequestError::unexpected_part)?;
    if field.name() != Some(ARGS_PART) {
        return Err(RequestError::unexpected_part());
    }
    let args = field.bytes().await.map_err(RequestError::from_multer)?;
    exact::from_slice(&args).map_err(|source| RequestError::new(Refusal::Arguments(source)))
}

/// Refuses an upload from another site, one without [`UPLOAD_HEADER`], one
/// that says it is longer than `limit`, and one that is not multipart, and
/// reads `body` as multipart parts as it arrives, ending with a 413 once it
/// passes `limit`.
fn open_upload_stream(
    headers: &HeaderMap,
    body: Body,
    limit: usize,
) -> Result<multer::Multipart<'static>, RequestError> {
    refuse_foreign_upload(headers, limit)?;
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let boundary = multer::parse_boundary(content_type)
        .map_err(|source| RequestError::new(Refusal::NotMultipart(Box::new(source))))?;
    let limit = u64::try_from(limit).unwrap_or(u64::MAX);
    let constraints =
        multer::Constraints::new().size_limit(multer::SizeLimit::new().whole_stream(limit));
    Ok(multer::Multipart::with_constraints(
        body.into_data_stream(),
        boundary,
        constraints,
    ))
}

/// Refuses an upload from another site, one without [`UPLOAD_HEADER`], and
/// one that says it is longer than `limit`.
fn refuse_foreign_upload(headers: &HeaderMap, limit: usize) -> Result<(), RequestError> {
    refuse_cross_site(headers)?;
    // A `multipart/form-data` body is one a form on another site can send
    // without a preflight, but no form can add a header. A browser sends a
    // preflight before a cross-origin request with one, and this server
    // answers no preflight.
    if !headers.contains_key(UPLOAD_HEADER) {
        return Err(RequestError::new(Refusal::NotFromPage));
    }
    let declared = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok());
    if declared.is_some_and(|length| length > limit) {
        return Err(RequestError::new(Refusal::TooLarge));
    }
    Ok(())
}

/// Sends a comment on a quiet stream, so that a proxy does not close it.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn keep_alive<S>(sse: Sse<S>) -> Sse<axum::response::sse::KeepAliveStream<S>> {
    sse.keep_alive(axum::response::sse::KeepAlive::default())
}

/// Leaves the stream as it is, because axum times keep-alive comments with
/// tokio, which has no timer on wasm32-unknown-unknown.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn keep_alive<S>(sse: Sse<S>) -> Sse<S> {
    sse
}

/// The last event of a stream whose handler's stream finished, as the wire
/// format in `patch.rs` describes. Data is `{}` because axum writes no `data`
/// field for an empty string, and an event without one is not dispatched.
fn end_of_stream() -> SseEvent {
    SseEvent::default().event("hs-end").data("{}")
}

fn to_sse(event: &PatchEvent) -> Result<SseEvent, PartMismatch> {
    Ok(SseEvent::default()
        .event(event.name())
        .data(event.try_data()?))
}

/// A request to an action or an upload that was refused, before its handler
/// ran or while a streaming upload's handler read the files.
///
/// Its response is `403 Forbidden` for a request from another site, a GET
/// action call or an upload without the header the runtime adds, `415 Unsupported Media Type`
/// for a POST whose body is not JSON or an upload whose body is not
/// `multipart/form-data`, `413 Payload Too Large` for an upload longer than
/// [`Upload::limit`], and `400 Bad Request` for a malformed multipart body,
/// an upload whose first part is not the arguments, a call whose
/// `hypersig-part` header is not a dotted field path,
/// or arguments that do not deserialize, with the serde message.
#[derive(Debug)]
pub struct RequestError {
    refusal: Refusal,
    backtrace: Backtrace,
}

#[derive(Debug)]
enum Refusal {
    CrossSite,
    NotJson,
    NotFromRuntime,
    NotFromPage,
    NotMultipart(Box<dyn StdError + Send + Sync>),
    TooLarge,
    Multipart(Box<dyn StdError + Send + Sync>),
    PartOrder,
    Arguments(serde_json::Error),
    Part,
}

impl RequestError {
    fn new(refusal: Refusal) -> Self {
        Self {
            refusal,
            backtrace: Backtrace::capture(),
        }
    }

    /// The refusal for `error`, met while reading an upload's body.
    pub(crate) fn from_multer(error: multer::Error) -> Self {
        Self::new(match error {
            multer::Error::StreamSizeExceeded { .. } | multer::Error::FieldSizeExceeded { .. } => {
                Refusal::TooLarge
            }
            error => Refusal::Multipart(Box::new(error)),
        })
    }

    /// The refusal for an upload whose first part is not the arguments.
    pub(crate) fn unexpected_part() -> Self {
        Self::new(Refusal::PartOrder)
    }

    /// The backtrace captured where the error was created.
    #[must_use]
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    /// The status of the response the error answers with.
    #[must_use]
    pub fn status(&self) -> StatusCode {
        match self.refusal {
            Refusal::CrossSite | Refusal::NotFromRuntime | Refusal::NotFromPage => {
                StatusCode::FORBIDDEN
            }
            Refusal::NotJson | Refusal::NotMultipart(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Refusal::TooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Refusal::Multipart(_) | Refusal::PartOrder | Refusal::Arguments(_) | Refusal::Part => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.refusal {
            Refusal::CrossSite => f.write_str("the action was called from another site"),
            Refusal::NotJson => f.write_str("the action's body is not `application/json`"),
            Refusal::NotFromRuntime => {
                f.write_str("the call lacks the `Accept` header the hypersig runtime sends")
            }
            Refusal::NotFromPage => {
                f.write_str("the upload lacks the header the hypersig runtime adds")
            }
            Refusal::NotMultipart(_) => {
                f.write_str("the upload's body is not `multipart/form-data`")
            }
            Refusal::TooLarge => f.write_str("the upload is longer than its limit"),
            Refusal::Multipart(source) => write!(f, "the upload's body is malformed: {source}"),
            Refusal::PartOrder => f.write_str("the upload's first part is not the arguments"),
            Refusal::Arguments(source) => {
                write!(f, "the action's arguments did not deserialize: {source}")
            }
            Refusal::Part => f.write_str("the call's part header is not a dotted field path"),
        }
    }
}

impl StdError for RequestError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.refusal {
            Refusal::Arguments(source) => Some(source),
            Refusal::Multipart(source) | Refusal::NotMultipart(source) => Some(source.as_ref()),
            Refusal::CrossSite
            | Refusal::NotJson
            | Refusal::NotFromRuntime
            | Refusal::NotFromPage
            | Refusal::TooLarge
            | Refusal::PartOrder
            | Refusal::Part => None,
        }
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        (self.status(), self.to_string()).into_response()
    }
}
