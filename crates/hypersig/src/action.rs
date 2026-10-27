//! Server endpoints that the browser calls with typed arguments.

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use serde_json::{Map, Value, json};

use crate::expr::{Expr, IntoExpr, IntoStmt, Stmt, in_let_scope};
use crate::html::{El, Id};
use crate::route::Base;
use crate::signal::{Scopes, Sig, current_part};

/// The HTTP method an [`Action`] is served on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Method {
    /// `GET`. The arguments travel as JSON in the `args` query parameter.
    Get,
    /// `POST`. The arguments travel as the JSON body.
    Post,
}

impl Method {
    /// The method name as it appears in a request line.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
        }
    }
}

/// A server endpoint on the page whose signals are `S`, which receives arguments `A`.
///
/// Declare one on an `action` line of [`routes!`](crate::routes), which
/// names its handler, and call it from the page with [`Action::call`] or
/// [`Action::call_with`]. The function `routes!` generates registers the
/// handler, so both sides agree on the path, the method, the argument type,
/// and the signal struct its patches target, and an action without a
/// handler cannot be declared.
///
/// The request carries the arguments and nothing else. A handler that needs
/// several values takes a tuple, which the page passes as a tuple of expressions.
///
/// The type of `A` holds, and its values do not: the visitor's browser can
/// send any value of `A`, whatever the page's expressions would compute, and
/// anyone can call the action's URL without loading the page. A handler that
/// guards the page does not guard its actions, so each handler checks that
/// the visitor may act on the arguments it receives, such as the owner of an
/// order whose id it takes.
///
/// ```
/// use hypersig::{Action, Patches, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Counter {
///     count: i32,
/// }
///
/// hypersig::routes! {
///     fn actions(Router<()>);
///     action INCREMENT: Action<Counter> = post "/increment" => increment;
///     action SET: Action<Counter, (i32, String)> = post "/set" => set;
/// }
///
/// async fn increment((): (), (): ()) -> Patches<Counter> {
///     Counter::count().patch(1).into()
/// }
///
/// async fn set((): (), (count, _note): (i32, String)) -> Patches<Counter> {
///     Counter::count().patch(count).into()
/// }
///
/// let _click = INCREMENT.call();
/// let _save = SET.call_with((Counter::count(), "typed"));
/// let _app: axum::Router = actions(axum::Router::new());
/// ```
pub struct Action<S, A = ()> {
    method: Method,
    base: Base,
    path: &'static str,
    marker: PhantomData<fn(S, A)>,
}

impl<S, A> Action<S, A> {
    /// An action served on `method path` under `base`, which the page's
    /// calls put before the path.
    ///
    /// [`routes!`](crate::routes) checks `path` with [`Action::check_path`]
    /// in a `const _` beside the constant, so a path that does not start
    /// with `/` is a compile error.
    pub(crate) const fn at(method: Method, base: Base, path: &'static str) -> Self {
        Self {
            method,
            base,
            path,
            marker: PhantomData,
        }
    }

    /// Why `path` cannot be an action's path, if it cannot.
    pub(crate) const fn check_path(path: &'static str) -> Result<(), &'static str> {
        if path.is_empty() || path.as_bytes()[0] != b'/' {
            return Err("an action path starts with `/`");
        }
        Ok(())
    }

    /// The path the action is served on, without its [`Base`].
    #[must_use]
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// The method the action is served on.
    #[must_use]
    pub fn method(&self) -> Method {
        self.method
    }

    /// The base the routes are declared under.
    #[cfg(feature = "axum")]
    pub(crate) fn base(&self) -> Base {
        self.base
    }

    /// The path a call to this action names, with its [`Base`].
    pub(crate) fn call_path(&self) -> String {
        self.base.join(self.path)
    }
}

#[cfg(feature = "axum")]
impl<S, A: serde::Serialize> Action<S, A> {
    /// The request the runtime sends to call this action with `args`.
    ///
    /// A test hands it to the router with `tower::ServiceExt::oneshot`. It
    /// carries `Accept: text/event-stream`, which the runtime sends with
    /// every call and [`RequestKind::of`](crate::RequestKind::of) reads, and
    /// the `hypersig-call` header the runtime adds, which the router requires
    /// of a `GET`. A `POST` sends `args` as its JSON body with `Content-Type:
    /// application/json`, and a `GET` sends them as JSON in the `args` query
    /// parameter. The path is [`Action::path`], without the [`Base`], as the
    /// router registers it.
    ///
    /// # Examples
    ///
    /// ```
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Votes {
    ///     likes: i32,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn actions(Router<()>);
    ///     action LIKE: Action<Votes, i32> = post "/like" => like;
    /// }
    /// # async fn like((): (), _: i32) -> hypersig::Patches<Votes> { hypersig::Patches::new() }
    /// let request = LIKE.request(&3);
    /// assert_eq!(request.uri(), "/like");
    /// assert_eq!(request.headers()["content-type"], "application/json");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `args` does not serialize to JSON, such as a map whose keys
    /// are not strings.
    #[must_use]
    pub fn request(&self, args: &A) -> axum::http::Request<axum::body::Body> {
        runtime_request(self.method, self.path, args)
    }
}

impl<S> Action<S, ()> {
    /// A call to this action, which takes no arguments.
    #[must_use]
    pub fn call(&self) -> Call<S> {
        Call::new(
            self.method,
            self.base,
            self.path,
            Expr::<S, ()>::build(|_| Value::Null),
        )
    }
}

impl<S, A> Action<S, A> {
    /// A call to this action with `args`.
    ///
    /// `args` is evaluated in the browser when the call runs, so it can be a
    /// literal such as `&id` or an expression such as `item.at(Todo::id())`.
    #[must_use]
    pub fn call_with(&self, args: impl IntoExpr<S, A>) -> Call<S> {
        Call::new(self.method, self.base, self.path, args.into_expr())
    }
}

impl<S, A> Clone for Action<S, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, A> Copy for Action<S, A> {}

impl<S, A> fmt::Debug for Action<S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Action")
            .field("method", &self.method)
            .field("base", &self.base.as_str())
            .field("path", &self.path)
            .finish()
    }
}

/// A call to an [`Action`], usable wherever a [`Stmt`] is.
///
/// ```
/// use hypersig::{Action, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Form {
///     saving: bool,
///     error: String,
/// }
///
/// hypersig::routes! {
///     fn actions(Router<()>);
///     action SAVE: Action<Form> = post "/save" => save;
/// }
/// # async fn save((): (), (): ()) -> hypersig::Patches<Form> { hypersig::Patches::new() }
/// let _click = SAVE.call().indicator(Form::saving()).error(Form::error());
/// ```
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool is one independent flag of the call's wire form, which the runtime reads by name"
)]
pub struct Call<S> {
    method: Method,
    base: Base,
    path: &'static str,
    args: Value,
    indicator: Value,
    error: Value,
    latest: bool,
    resends: bool,
    /// Whether `indicator` says that a stream's answer is open, rather than that an answer is awaited.
    connected: bool,
    exclusive: bool,
    /// The milliseconds after which an answer that has not ended counts as failed.
    timeout: Option<u32>,
    /// The name of the [`FileInput`](crate::FileInput) whose files an upload sends.
    files: Option<&'static str>,
    /// The path of the signal that holds the status of a failed answer.
    status: Option<Value>,
    /// The `let` depth the error text is named at, and the statement that reads it.
    on_error: Option<(u8, Value)>,
    on_success: Option<Value>,
    /// The page field of the [`Part`](crate::Part) the call was built in.
    part: Option<Arc<str>>,
    /// The list item scopes the arguments and the statements read.
    scopes: Scopes,
    marker: PhantomData<fn() -> S>,
}

impl<S> Call<S> {
    fn new<A>(method: Method, base: Base, path: &'static str, args: Expr<S, A>) -> Self {
        let mut scopes = Scopes::default();
        Self {
            method,
            base,
            path,
            args: args.take(&mut scopes),
            indicator: Value::Null,
            error: Value::Null,
            latest: false,
            resends: false,
            connected: false,
            exclusive: false,
            timeout: None,
            files: None,
            status: None,
            on_error: None,
            on_success: None,
            part: current_part(),
            scopes,
            marker: PhantomData,
        }
    }

    /// A `POST` to `path` that sends `args` with the files picked in the
    /// file input named `files`.
    pub(crate) fn upload<A>(
        base: Base,
        path: &'static str,
        args: Expr<S, A>,
        files: &'static str,
    ) -> Self {
        Self {
            files: Some(files),
            ..Self::new(Method::Post, base, path, args)
        }
    }

    /// Sets `flag` to true while the response is awaited and to false after.
    ///
    /// Several calls may set one flag. It turns false once every one of them
    /// has its answer.
    #[must_use]
    pub fn indicator(mut self, flag: Sig<S, bool>) -> Self {
        self.indicator = Value::from(flag.path());
        self
    }

    /// Sets `message` to the error text when the call fails.
    ///
    /// It sets `message` to `""` when the call is sent.
    ///
    /// Emptying it on each send lets a handler of
    /// [`El::on_signal_change`](crate::html::El::on_signal_change) see a
    /// failure that repeats the last one. The text is the `text/plain` body
    /// of an answer whose status is outside 2xx, which axum sends for a
    /// `String`, so a handler that fails writes the sentence the page shows.
    /// When that body is empty or of another type, such as a proxy's HTML
    /// error page, it is the status and its reason, such as `502 Bad
    /// Gateway`, and when no answer arrives, it is the browser's message. A
    /// 2xx answer fails too, with a message of the runtime's own, when it
    /// was redirected, is not an event stream, or closes before the event
    /// `hs-end` that ends every answer of the router, as does a call whose
    /// [`Call::timeout`] passed. The status also goes to the console and to
    /// the `hs-error` event.
    #[must_use]
    pub fn error(mut self, message: Sig<S, String>) -> Self {
        self.error = Value::from(message.path());
        self
    }

    /// Sets `status` to the HTTP status of a failed answer.
    ///
    /// It sets `status` to `0` when the call is sent.
    ///
    /// It stays `0` when no answer the runtime can read arrives: the
    /// connection fails, the answer was redirected, is not an event stream,
    /// or closes before the server ended it, or the [`Call::timeout`]
    /// passed. A page compares it with a status the server answers with,
    /// written from `StatusCode` rather than as a number, to tell one
    /// failure from another without reading the text [`Call::error`] holds.
    ///
    /// # Examples
    ///
    /// ```
    /// use axum::http::StatusCode;
    /// use hypersig::html::{a, button, div};
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Notes {
    ///     failure: i32,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn actions(Router<()>);
    ///     action SAVE: Action<Notes> = post "/save" => save;
    /// }
    /// # async fn save((): (), (): ()) -> hypersig::Patches<Notes> { hypersig::Patches::new() }
    /// let signed_out = Notes::failure().equals(i32::from(StatusCode::UNAUTHORIZED.as_u16()));
    /// let _page = div()
    ///     .child(button().on_click(SAVE.call().status(Notes::failure())).text("Save"))
    ///     .child(a().show(signed_out).text("Sign in again"));
    /// ```
    #[must_use]
    pub fn status(mut self, status: Sig<S, i32>) -> Self {
        self.status = Some(Value::from(status.path()));
        self
    }

    /// Aborts the unfinished earlier `latest` call to the same action.
    ///
    /// A slow answer to an earlier call then cannot overwrite the answer to
    /// this one. The calls of every element to the action share the one
    /// latest call, because the handler decides which signals they patch. A
    /// call made inside a [`Part`](crate::Part) shares it with the calls of
    /// that part only.
    ///
    /// Use it for a read whose answer replaces the previous one, such as a
    /// search that runs as the visitor types. The patches of the aborted call
    /// that have not arrived are dropped, and the server may stop its handler
    /// at an `.await`, so leave it off a call that changes data. A later call
    /// never aborts a call without `latest`. For a `POST`, the runtime drops
    /// the events of an earlier call that arrive after a later call to the
    /// same action has had its complete answer, with or without `latest`.
    ///
    /// ```
    /// use hypersig::html::input;
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Search {
    ///     query: String,
    ///     results: Vec<String>,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn actions(Router<()>);
    ///     action FIND: Action<Search, String> = get "/find" => find;
    /// }
    /// # async fn find((): (), _: String) -> hypersig::Patches<Search> { hypersig::Patches::new() }
    /// let query = Search::query();
    /// let _field = input().bind(query).on_input(FIND.call_with(query).latest());
    /// ```
    #[must_use]
    pub fn latest(mut self) -> Self {
        self.latest = true;
        self
    }

    /// Skips this call while another call to the same action awaits its answer.
    ///
    /// A second click on a button that places an order then sends nothing
    /// while the first order is on its way. The skipped call sets no
    /// indicator and no error, and runs neither [`Call::on_error`] nor
    /// [`Call::on_success`]. A call made inside a [`Part`](crate::Part)
    /// waits only for the calls of that part.
    ///
    /// While a `POST` awaits its answer, with or without `exclusive`, the
    /// runtime sets `aria-busy="true"` on the element whose handler sent it,
    /// unless the page gave that element an `aria-busy` of its own.
    ///
    /// ```
    /// use hypersig::html::button;
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Cart {
    ///     ordering: bool,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn actions(Router<()>);
    ///     action ORDER: Action<Cart> = post "/order" => order;
    /// }
    /// # async fn order((): (), (): ()) -> hypersig::Patches<Cart> { hypersig::Patches::new() }
    /// let _order = button()
    ///     .on_click(ORDER.call().indicator(Cart::ordering()).exclusive())
    ///     .text("Order");
    /// ```
    #[must_use]
    pub fn exclusive(mut self) -> Self {
        self.exclusive = true;
        self
    }

    /// Fails this call when its answer has not ended `limit` after it was sent.
    ///
    /// The runtime then aborts the request, drops the events still to come,
    /// sets the [`Call::status`] signal to `0`, and writes the text the
    /// [`Call::error`] signal shows, as for a connection that failed. Without
    /// it a call waits for as long as the server takes, which suits an
    /// upload of a large file. A [`StreamCall`] has no timeout, because a
    /// stream stays open until its handler ends it.
    ///
    /// The server may still finish the handler after the browser stopped
    /// waiting, so a call that changes data can have taken effect.
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// use hypersig::html::button;
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Report {
    ///     problem: String,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn actions(Router<()>);
    ///     action BUILD: Action<Report> = post "/build" => build;
    /// }
    /// # async fn build((): (), (): ()) -> hypersig::Patches<Report> { hypersig::Patches::new() }
    /// let call = BUILD.call().error(Report::problem()).timeout(Duration::from_secs(20));
    /// let _build = button().on_click(call).text("Build the report");
    /// ```
    #[must_use]
    pub fn timeout(mut self, limit: Duration) -> Self {
        // A browser fires a timer of more than `i32::MAX` milliseconds at
        // once, so a longer limit is held at that, a little under 25 days.
        let millis = limit.as_millis().min(i32::MAX.unsigned_abs().into());
        self.timeout = Some(u32::try_from(millis).unwrap_or(u32::MAX));
        self
    }

    /// Runs the statement `handle` builds when this call fails.
    ///
    /// `handle` receives the error text, the same text [`Call::error`] writes,
    /// so a page with several calls in flight can mark the one that failed
    /// rather than all of them. The statement runs where the call was made:
    /// inside a list item it reaches that item, and inside
    /// [`Expr::let_in_stmt`] it reads the values named when the call was sent.
    ///
    /// The statement does not run for a `latest` call that a later one
    /// aborted, nor once the page navigates away.
    ///
    /// ```
    /// use hypersig::html::{button, each, li};
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Line {
    ///     id: i32,
    ///     problem: String,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Cart {
    ///     lines: Vec<Line>,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn actions(Router<()>);
    ///     action ORDER: Action<Cart, i32> = post "/order" => order;
    /// }
    /// # async fn order((): (), _: i32) -> hypersig::Patches<Cart> { hypersig::Patches::new() }
    /// let _lines = each(Cart::lines(), |line| {
    ///     li().child(button().text("Order").on_click(
    ///         ORDER
    ///             .call_with(line.at(Line::id()))
    ///             .on_error(|message| line.at(Line::problem()).set(message))
    ///             .on_success(line.at(Line::problem()).set("")),
    ///     ))
    /// });
    /// ```
    #[must_use]
    pub fn on_error<B: IntoStmt<S>>(mut self, handle: impl FnOnce(Expr<S, String>) -> B) -> Self {
        self.on_error = Some(in_let_scope(|depth| {
            let message = Expr::build(|_| json!(["var", depth]));
            (depth, handle(message).into_stmt().take(&mut self.scopes))
        }));
        self
    }

    /// Runs `next` once the server's answer has arrived and its patches are applied.
    ///
    /// It runs where the call was made, as [`Call::on_error`] does, and not
    /// for an aborted `latest` call or once the page navigates away.
    #[must_use]
    pub fn on_success(mut self, next: impl IntoStmt<S>) -> Self {
        self.on_success = Some(next.into_stmt().take(&mut self.scopes));
        self
    }

    /// Sends this call, then runs `next` without waiting for the answer.
    ///
    /// The arguments are evaluated when the call is sent, so `next` can
    /// change what they read. The patches the server answers with apply when
    /// they arrive.
    #[must_use]
    pub fn then(self, next: impl IntoStmt<S>) -> Stmt<S> {
        self.into_stmt().then(next)
    }
}

impl<S> IntoStmt<S> for Call<S> {
    fn into_stmt(self) -> Stmt<S> {
        let call = self;
        let path = call.base.join(call.path);
        let mut ast = match call.files {
            Some(files) => json!(["upload", path, call.args, call.indicator, call.error, files]),
            None => json!([
                "call",
                call.method.as_str(),
                path,
                call.args,
                call.indicator,
                call.error
            ]),
        };
        let flags: Vec<&str> = [
            ("latest", call.latest),
            ("reconnect", call.resends),
            ("connected", call.connected),
            ("exclusive", call.exclusive),
        ]
        .into_iter()
        .filter_map(|(flag, on)| on.then_some(flag))
        .collect();
        let mut outcomes = Map::new();
        if let Some(timeout) = call.timeout {
            outcomes.insert("timeout".to_owned(), Value::from(timeout));
        }
        if let Some((depth, stmt)) = call.on_error {
            outcomes.insert("error".to_owned(), json!([depth, stmt]));
        }
        if let Some(status) = call.status {
            outcomes.insert("status".to_owned(), status);
        }
        if let Some(stmt) = call.on_success {
            outcomes.insert("ok".to_owned(), stmt);
        }
        if let Some(part) = call.part {
            outcomes.insert("part".to_owned(), Value::from(&*part));
        }
        // A call without flags or outcomes keeps the six-element form, and
        // one with outcomes writes its flags, empty or not, before them.
        if let Value::Array(items) = &mut ast {
            if !flags.is_empty() || !outcomes.is_empty() {
                items.push(json!(flags));
            }
            if !outcomes.is_empty() {
                items.push(Value::Object(outcomes));
            }
        }
        Stmt::build(|scopes| {
            scopes.extend(call.scopes);
            ast
        })
    }
}

impl<S> fmt::Debug for Call<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Call")
            .field("method", &self.method)
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

/// A stream of patches on the page whose signals are `S`.
///
/// Its arguments are `A`.
///
/// Declare one on a `stream` line of [`routes!`](crate::routes), which
/// serves it on `GET` with a handler that returns a stream of patches. A call
/// to it can [`reconnect`](StreamCall::reconnect), which a call to an
/// [`Action`] cannot, because sending a `POST` again could repeat a change
/// the server already made.
///
/// A stream that an element's statement opened closes once the element
/// leaves the page, and the runtime applies none of its later events. A
/// patch that replaces the element then leaves one stream open, the one the
/// new element's statement opens.
///
/// ```
/// use hypersig::html::div;
/// use hypersig::{Signals, StreamAction};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Feed {
///     items: Vec<String>,
/// }
///
/// hypersig::routes! {
///     fn streams(Router<()>);
///     stream UPDATES: StreamAction<Feed> = "/updates" => |(), ()| {
///         futures::stream::empty::<hypersig::Patches<Feed>>()
///     };
/// }
/// let _feed = div().on_load(UPDATES.call().reconnect());
/// ```
pub struct StreamAction<S, A = ()> {
    base: Base,
    path: &'static str,
    marker: PhantomData<fn(S, A)>,
}

impl<S, A> StreamAction<S, A> {
    /// A stream served on `GET path` under `base`, which the page's calls
    /// put before the path.
    ///
    /// [`routes!`](crate::routes) checks `path` with
    /// [`StreamAction::check_path`] in a `const _` beside the constant, so a
    /// path that does not start with `/` is a compile error.
    pub(crate) const fn at(base: Base, path: &'static str) -> Self {
        Self {
            base,
            path,
            marker: PhantomData,
        }
    }

    /// Why `path` cannot be a stream's path, if it cannot.
    pub(crate) const fn check_path(path: &'static str) -> Result<(), &'static str> {
        if path.is_empty() || path.as_bytes()[0] != b'/' {
            return Err("a stream path starts with `/`");
        }
        Ok(())
    }

    /// The path the stream is served on, without its [`Base`].
    #[must_use]
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// The base the routes are declared under.
    #[cfg(feature = "axum")]
    pub(crate) fn base(&self) -> Base {
        self.base
    }

    /// The path a call to this stream names, with its [`Base`].
    pub(crate) fn call_path(&self) -> String {
        self.base.join(self.path)
    }

    /// The request the runtime sends to open this stream with `args`.
    ///
    /// It is the `GET` that [`Action::request`] builds: `Accept:
    /// text/event-stream` and the `hypersig-call` header, with `args` as JSON
    /// in the `args` query parameter.
    ///
    /// # Panics
    ///
    /// Panics if `args` does not serialize to JSON.
    #[cfg(feature = "axum")]
    #[must_use]
    pub fn request(&self, args: &A) -> axum::http::Request<axum::body::Body>
    where
        A: serde::Serialize,
    {
        runtime_request(Method::Get, self.path, args)
    }

    /// A call to this stream with `args`.
    ///
    /// The browser evaluates `args` when the call runs.
    ///
    /// A call that [reconnects](StreamCall::reconnect) evaluates `args`
    /// again before each attempt.
    #[must_use]
    pub fn call_with(&self, args: impl IntoExpr<S, A>) -> StreamCall<S> {
        StreamCall(Call::new(
            Method::Get,
            self.base,
            self.path,
            args.into_expr(),
        ))
    }
}

impl<S> StreamAction<S, ()> {
    /// A call to this stream, which takes no arguments.
    #[must_use]
    pub fn call(&self) -> StreamCall<S> {
        StreamCall(Call::new(
            Method::Get,
            self.base,
            self.path,
            Expr::<S, ()>::build(|_| Value::Null),
        ))
    }
}

impl<S, A> Clone for StreamAction<S, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, A> Copy for StreamAction<S, A> {}

impl<S, A> fmt::Debug for StreamAction<S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamAction")
            .field("base", &self.base.as_str())
            .field("path", &self.path)
            .finish()
    }
}

/// A call to a [`StreamAction`], usable wherever a [`Stmt`] is.
pub struct StreamCall<S>(Call<S>);

impl<S> StreamCall<S> {
    /// Sets `flag` to true while the server's answer is open, and to false otherwise.
    ///
    /// The flag turns true when the server answers with a 2xx status and
    /// false when the stream ends or breaks off. It stays false while the
    /// stream connects and while [`reconnect`](StreamCall::reconnect) waits to
    /// open it again, so a page can show whether it is receiving changes.
    /// Several streams may set one flag, which is true while any of them is open.
    #[must_use]
    pub fn connected(mut self, flag: Sig<S, bool>) -> Self {
        self.0 = self.0.indicator(flag);
        self.0.connected = true;
        self
    }

    /// Sets `message` as [`Call::error`] does, and to `""` when the stream connects.
    #[must_use]
    pub fn error(self, message: Sig<S, String>) -> Self {
        Self(self.0.error(message))
    }

    /// Sets `status` as [`Call::status`] does, and to `0` when the stream connects.
    #[must_use]
    pub fn status(self, status: Sig<S, i32>) -> Self {
        Self(self.0.status(status))
    }

    /// Opens the stream again when its connection fails, breaks off, or answers 5xx.
    ///
    /// The first attempt waits up to 1 second, and each later one up to twice
    /// as long, up to 30 seconds. Each wait is at least half its length, at a
    /// random point, so pages that lost the server together do not all
    /// return at the same moment. The wait ends early when the browser comes
    /// back online or the page becomes visible. A stream that delivers an
    /// event starts the wait over, so a server that answers 2xx and closes
    /// before sending anything is asked again less often each time. While
    /// the stream is failing, the [`error`](StreamCall::error) signal holds
    /// the last failure.
    ///
    /// The server sends the event `hs-end` last once the handler's stream
    /// finishes, and a stream that ended with it is not opened again. A
    /// response that closes before it, as a host closes one at its time
    /// limit, counts as broken off. A stream whose element has left the page
    /// is not opened again either. Each attempt evaluates the arguments again
    /// and runs the handler with them. Pass a signal the stream's patches
    /// keep up to date, such as the id of the last message received, and the
    /// handler can send only what came after it. A stream whose arguments
    /// do not change sends the current values first, so the page catches up
    /// with what it missed.
    ///
    /// ```
    /// use hypersig::html::div;
    /// use hypersig::{Signals, StreamAction};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Chat {
    ///     messages: Vec<String>,
    ///     last_id: i32,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn streams(Router<()>);
    ///     // The handler sends the messages after `last_id` and patches `last_id`.
    ///     stream SINCE: StreamAction<Chat, i32> = "/messages" => since;
    /// }
    /// # fn since((): (), _: i32) -> futures::stream::Empty<hypersig::Patches<Chat>> { futures::stream::empty() }
    /// let _feed = div().on_load(SINCE.call_with(Chat::last_id()).reconnect());
    /// ```
    #[must_use]
    pub fn reconnect(mut self) -> Self {
        self.0.resends = true;
        self
    }

    /// Opens this stream, then runs `next` without waiting for an answer.
    #[must_use]
    pub fn then(self, next: impl IntoStmt<S>) -> Stmt<S> {
        self.0.then(next)
    }
}

impl<S> IntoStmt<S> for StreamCall<S> {
    fn into_stmt(self) -> Stmt<S> {
        self.0.into_stmt()
    }
}

impl<S> fmt::Debug for StreamCall<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("StreamCall").field(&self.0).finish()
    }
}

/// A named element on a page whose signals are `S`, for imperative DOM calls.
///
/// Declare one with [`node_ref!`](crate::node_ref), attach it with
/// [`El::node_ref`](crate::html::El::node_ref), and use its statements in
/// handlers. Inside a list item the browser looks for the element in the same
/// item first, so one `NodeRef` serves every item.
///
/// Its name is the path of its `const`, which `node_ref!` writes from
/// `module_path!()` and the constant's identifier, so two constants never
/// share a name. One constant attached to two elements the browser could
/// both reach outside a list template is the mistake left:
/// [`Page::render`](crate::Page::render) and
/// [`El::render`](crate::html::El::render) panic on it, which a test that
/// renders the page catches. A [`Fragment`](crate::Fragment) sent again is
/// rendered on its own, so the server cannot compare it with the rest of the
/// page. The browser does, and writes a warning to the console when the patch
/// adds a name or an id that another element it can reach already carries. A
/// [`FileInput`](crate::FileInput) name is looked up apart from these, so it
/// never collides with one.
///
/// ```
/// use hypersig::html::{button, div, input};
/// use hypersig::{NodeRef, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Search {
///     query: String,
/// }
///
/// hypersig::node_ref!(const FIELD: NodeRef<Search>);
///
/// # fn main() {
/// let html = div()
///     .child(input().node_ref(FIELD).bind(Search::query()))
///     .child(button().on_click(FIELD.focus()).text("Search"))
///     .render();
/// assert!(html.contains(&format!(r#"data-hs-ref="{}::FIELD""#, module_path!())));
/// # }
/// ```
pub struct NodeRef<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> NodeRef<S> {
    /// A reference named `name`, which [`node_ref!`](crate::node_ref) writes
    /// from the path of its constant.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty. In a `const`, that is a compile error.
    pub(crate) const fn new(name: &'static str) -> Self {
        assert!(!name.is_empty(), "a node ref name is never empty");
        Self {
            name,
            marker: PhantomData,
        }
    }

    pub(crate) const fn name(self) -> &'static str {
        self.name
    }

    /// An element id spelled as this reference's name.
    ///
    /// Two constants never share a name, so their ids differ too. A
    /// component that names its element by a `NodeRef` uses it for the
    /// `id` that `aria-*` attributes point at.
    ///
    /// ```
    /// use hypersig::NodeRef;
    ///
    /// hypersig::node_ref!(const FIELD: NodeRef<()>);
    ///
    /// # fn main() {
    /// assert_eq!(FIELD.id().as_str(), format!("{}::FIELD", module_path!()));
    /// # }
    /// ```
    #[must_use]
    pub const fn id(self) -> Id {
        Id::new(self.name)
    }

    fn dom(self, op: &str) -> Stmt<S> {
        Stmt::build(|_| json!(["dom", op, self.name]))
    }

    /// A statement that focuses the element after the next render.
    #[must_use]
    pub fn focus(self) -> Stmt<S> {
        self.dom("focus")
    }

    /// A statement that removes focus from the element.
    #[must_use]
    pub fn blur(self) -> Stmt<S> {
        self.dom("blur")
    }

    /// A statement that selects the element's text, for an `<input>`.
    #[must_use]
    pub fn select(self) -> Stmt<S> {
        self.dom("select")
    }

    /// A statement that scrolls the element into view.
    #[must_use]
    pub fn scroll_into_view(self) -> Stmt<S> {
        self.dom("scroll")
    }

    /// A statement that hides the element if it is an open popover.
    ///
    /// The browser then returns focus to the element that had it before the
    /// popover opened, where focus was inside the popover.
    #[must_use]
    pub fn hide_popover(self) -> Stmt<S> {
        self.dom("hide-popover")
    }

    /// A statement that shows the element if it is a closed popover.
    #[must_use]
    pub fn show_popover(self) -> Stmt<S> {
        self.dom("show-popover")
    }
}

impl<S> Clone for NodeRef<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for NodeRef<S> {}

impl<S> fmt::Debug for NodeRef<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NodeRef").field(&self.name).finish()
    }
}

/// A named `<dialog>` on a page whose signals are `S`.
///
/// Statements open and close it.
///
/// Declare one with [`dialog_ref!`](crate::dialog_ref).
/// [`DialogRef::dialog`] is the only way to attach one, and it builds the
/// `<dialog>` element itself, so [`show_modal`](DialogRef::show_modal) and
/// [`close`](DialogRef::close) always name a dialog. A [`NodeRef`] has
/// neither, since it may sit on any element.
///
/// Its name is the path of its `const`, as a [`NodeRef`]'s is, in the one
/// namespace both share. Rendering panics where the dialog of one constant
/// sits twice where the browser can reach both.
///
/// ```
/// use hypersig::html::{button, div, input};
/// use hypersig::{DialogRef, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Palette {
///     query: String,
/// }
///
/// hypersig::dialog_ref!(const PALETTE: DialogRef<Palette>);
///
/// # fn main() {
/// let page = div()
///     .child(button().on_click(PALETTE.show_modal()).text("Commands"))
///     .child(
///         PALETTE
///             .dialog()
///             .child(input().bind(Palette::query()))
///             .child(button().on_click(PALETTE.close()).text("Close")),
///     );
/// assert!(page.render().contains(&format!(r#"<dialog data-hs-ref="{}::PALETTE">"#, module_path!())));
/// # }
/// ```
pub struct DialogRef<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> DialogRef<S> {
    /// A reference named `name`, which [`dialog_ref!`](crate::dialog_ref)
    /// writes from the path of its constant.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty. In a `const`, that is a compile error.
    pub(crate) const fn new(name: &'static str) -> Self {
        assert!(!name.is_empty(), "a dialog ref name is never empty");
        Self {
            name,
            marker: PhantomData,
        }
    }

    pub(crate) const fn name(self) -> &'static str {
        self.name
    }

    /// An element id spelled as this reference's name, as [`NodeRef::id`] is.
    #[must_use]
    pub const fn id(self) -> Id {
        Id::new(self.name)
    }

    /// An empty `<dialog>` that this reference names.
    #[must_use]
    pub fn dialog(self) -> El<S> {
        crate::html::dialog().node_ref(NodeRef::new(self.name))
    }

    /// A statement that opens the dialog with `showModal()`, unless it is open.
    ///
    /// The browser makes the rest of the page inert, moves focus into the
    /// dialog, and closes it on Escape.
    #[must_use]
    pub fn show_modal(self) -> Stmt<S> {
        NodeRef::new(self.name).dom("show-modal")
    }

    /// A statement that closes the dialog, if it is open.
    ///
    /// The browser then returns focus to the element that had it before the
    /// dialog opened.
    #[must_use]
    pub fn close(self) -> Stmt<S> {
        NodeRef::new(self.name).dom("close")
    }
}

impl<S> Clone for DialogRef<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for DialogRef<S> {}

impl<S> fmt::Debug for DialogRef<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DialogRef").field(&self.name).finish()
    }
}

/// The request the runtime's `call` sends to `path` with `args`, as
/// `runtime/src/runtime/action/call.ts` builds it.
#[cfg(feature = "axum")]
fn runtime_request(
    method: Method,
    path: &str,
    args: &impl serde::Serialize,
) -> axum::http::Request<axum::body::Body> {
    use axum::body::Body;
    use axum::http::{Request, header};

    let json = serde_json::to_string(args).expect("the arguments serialize to JSON");
    let request = Request::builder()
        .method(method.as_str())
        .header(header::ACCEPT, "text/event-stream")
        .header(crate::axum::CALL_HEADER, "1");
    let request = match method {
        // The runtime leaves the parameter out for arguments that are `null`,
        // such as `()`.
        Method::Get if json == "null" => request.uri(path).body(Body::empty()),
        Method::Get => {
            let query = crate::route::encode_uri_component(&json);
            request
                .uri(format!("{path}?args={query}"))
                .body(Body::empty())
        }
        Method::Post => request
            .uri(path)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json)),
    };
    request.expect("a path that starts with `/` and these headers make a request")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    enum Page {}

    const FIND: Action<Page, String> = Action::at(Method::Get, Base::ROOT, "/find");
    const SAVE: Action<Page> = Action::at(Method::Post, Base::ROOT, "/save");
    const FEED: StreamAction<Page> = StreamAction::at(Base::ROOT, "/feed");

    #[test]
    fn a_call_without_flags_keeps_six_elements() {
        assert_eq!(
            SAVE.call().into_stmt().as_json(),
            &json!(["call", "POST", "/save", null, null, null])
        );
    }

    #[test]
    fn the_flags_follow_the_error_signal() {
        assert_eq!(
            FIND.call_with("tea").latest().into_stmt().as_json(),
            &json!([
                "call",
                "GET",
                "/find",
                ["lit", "tea"],
                null,
                null,
                ["latest"]
            ])
        );
    }

    #[test]
    fn a_stream_call_marks_its_flag_as_a_connection() {
        let live = Sig::<Page, bool>::from_path(crate::signal::Path::field("live"));
        assert_eq!(
            FEED.call().connected(live).into_stmt().as_json(),
            &json!(["call", "GET", "/feed", null, "live", null, ["connected"]])
        );
    }

    #[test]
    fn the_outcomes_follow_the_flags() {
        let saved: Sig<Page, String> = Sig::from_path(crate::signal::Path::field("saved"));
        assert_eq!(
            SAVE.call()
                .on_error(|message| saved.set(message))
                .on_success(saved.set("yes"))
                .into_stmt()
                .as_json(),
            &json!([
                "call",
                "POST",
                "/save",
                null,
                null,
                null,
                [],
                {
                    "error": [0, ["set", "saved", ["var", 0]]],
                    "ok": ["set", "saved", ["lit", "yes"]]
                }
            ])
        );
    }

    #[test]
    fn an_exclusive_call_with_a_timeout_writes_a_flag_and_the_milliseconds() {
        assert_eq!(
            SAVE.call()
                .exclusive()
                .timeout(Duration::from_secs(5))
                .into_stmt()
                .as_json(),
            &json!([
                "call",
                "POST",
                "/save",
                null,
                null,
                null,
                ["exclusive"],
                { "timeout": 5000 }
            ])
        );
    }

    #[test]
    fn a_timeout_longer_than_a_browser_timer_holds_is_cut_to_it() {
        let stmt = SAVE.call().timeout(Duration::MAX).into_stmt();
        assert_eq!(stmt.as_json()[7]["timeout"], json!(2_147_483_647));
    }

    #[test]
    fn a_stream_call_is_a_get_that_can_reconnect() {
        assert_eq!(
            FEED.call().reconnect().into_stmt().as_json(),
            &json!(["call", "GET", "/feed", null, null, null, ["reconnect"]])
        );
    }
}
