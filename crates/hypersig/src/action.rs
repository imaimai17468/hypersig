//! Server endpoints that the browser calls with typed arguments.

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use serde_json::{Map, Value, json};

use crate::expr::{Expr, IntoExpr, IntoStmt, Stmt, in_let_scope};
use crate::html::El;
use crate::route::Base;
use crate::signal::{Sig, current_part};

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
/// Declare one as a `const` and use the same value on both sides: the page
/// calls it with [`Action::call`] or [`Action::call_with`], and the router
/// registers its handler with `RouterExt::action`. Both sides then agree on the
/// path, the method, the argument type, and the signal struct its patches target.
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
/// use hypersig::{Action, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Counter {
///     count: i32,
/// }
///
/// const INCREMENT: Action<Counter> = Action::post("/increment");
/// const SET: Action<Counter, (i32, String)> = Action::post("/set");
/// let _click = INCREMENT.call();
/// let _save = SET.call_with((Counter::count(), "typed"));
/// ```
pub struct Action<S, A = ()> {
    method: Method,
    base: Base,
    path: &'static str,
    marker: PhantomData<fn(S, A)>,
}

impl<S, A> Action<S, A> {
    /// An action served on `POST path`.
    ///
    /// # Panics
    ///
    /// Panics if `path` does not start with `/`. In a `const`, that is a
    /// compile error.
    #[must_use]
    pub const fn post(path: &'static str) -> Self {
        Self::new(Method::Post, path)
    }

    /// An action served on `GET path`, for a read that changes nothing.
    ///
    /// A long-lived stream of patches is a [`StreamAction`].
    ///
    /// # Panics
    ///
    /// Panics if `path` does not start with `/`. In a `const`, that is a
    /// compile error.
    #[must_use]
    pub const fn get(path: &'static str) -> Self {
        Self::new(Method::Get, path)
    }

    const fn new(method: Method, path: &'static str) -> Self {
        assert!(
            !path.is_empty() && path.as_bytes()[0] == b'/',
            "an action path starts with `/`"
        );
        Self {
            method,
            base: Base::ROOT,
            path,
            marker: PhantomData,
        }
    }

    /// The same action under `base`, which the page's calls put before the path.
    ///
    /// The router registers [`Action::path`] without it, as it does a
    /// route's path. See [`Base`].
    #[must_use]
    pub const fn under(self, base: Base) -> Self {
        Self { base, ..self }
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
}

impl<S> Action<S, ()> {
    /// A call to this action, which takes no arguments.
    #[must_use]
    pub fn call(&self) -> Call<S> {
        Call::new(self.method, self.base, self.path, Value::Null)
    }
}

impl<S, A> Action<S, A> {
    /// A call to this action with `args`.
    ///
    /// `args` is evaluated in the browser when the call runs, so it can be a
    /// literal such as `&id` or an expression such as `item.at(Todo::id())`.
    #[must_use]
    pub fn call_with(&self, args: impl IntoExpr<S, A>) -> Call<S> {
        Call::new(
            self.method,
            self.base,
            self.path,
            args.into_expr().into_json(),
        )
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
/// const SAVE: Action<Form> = Action::post("/save");
/// let _click = SAVE.call().indicator(Form::saving()).error(Form::error());
/// ```
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
    /// The name of the [`FileInput`](crate::FileInput) whose files an upload sends.
    files: Option<&'static str>,
    /// The `let` depth the error text is named at, and the statement that reads it.
    on_error: Option<(u8, Value)>,
    on_success: Option<Value>,
    /// The page field of the [`Part`](crate::Part) the call was built in.
    part: Option<Arc<str>>,
    marker: PhantomData<fn() -> S>,
}

impl<S> Call<S> {
    fn new(method: Method, base: Base, path: &'static str, args: Value) -> Self {
        Self {
            method,
            base,
            path,
            args,
            indicator: Value::Null,
            error: Value::Null,
            latest: false,
            resends: false,
            connected: false,
            files: None,
            on_error: None,
            on_success: None,
            part: current_part(),
            marker: PhantomData,
        }
    }

    /// A `POST` to `path` that sends `args` with the files picked in the
    /// file input named `files`.
    pub(crate) fn upload(base: Base, path: &'static str, args: Value, files: &'static str) -> Self {
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

    /// Sets `message` to the error text when the call fails, and to `""` when it is sent.
    ///
    /// Emptying it on each send lets a handler of
    /// [`El::on_signal_change`](crate::html::El::on_signal_change) see a
    /// failure that repeats the last one. The text is the body of an answer whose status is outside 2xx, so a
    /// handler that fails writes the sentence the page shows. When that body
    /// is empty, it is the status and its reason, such as `502 Bad Gateway`,
    /// and when no answer arrives, it is the browser's message. The status
    /// also goes to the console and to the `hs-error` event.
    #[must_use]
    pub fn error(mut self, message: Sig<S, String>) -> Self {
        self.error = Value::from(message.path());
        self
    }

    /// Aborts the unfinished earlier `latest` call to the same action.
    ///
    /// A slow answer to an earlier call then cannot overwrite the answer to
    /// this one. The calls of every element to the action share the one
    /// latest call, because the handler decides which signals they patch.
    ///
    /// Use it for a read whose answer replaces the previous one, such as a
    /// search that runs as the visitor types. The patches of the aborted call
    /// that have not arrived are dropped, and the server may stop its handler
    /// at an `.await`, so leave it off a call that changes data. A call
    /// without `latest` is never aborted.
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
    /// const FIND: Action<Search, String> = Action::get("/find");
    /// let query = Search::query();
    /// let _field = input().bind(query).on_input(FIND.call_with(query).latest());
    /// ```
    #[must_use]
    pub fn latest(mut self) -> Self {
        self.latest = true;
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
    /// const ORDER: Action<Cart, i32> = Action::post("/order");
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
            let message = Expr::from_json(json!(["var", depth]));
            (depth, handle(message).into_stmt().into_json())
        }));
        self
    }

    /// Runs `next` once the server's answer has arrived and its patches are applied.
    ///
    /// It runs where the call was made, as [`Call::on_error`] does, and not
    /// for an aborted `latest` call or once the page navigates away.
    #[must_use]
    pub fn on_success(mut self, next: impl IntoStmt<S>) -> Self {
        self.on_success = Some(next.into_stmt().into_json());
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
        ]
        .into_iter()
        .filter_map(|(flag, on)| on.then_some(flag))
        .collect();
        let mut outcomes = Map::new();
        if let Some((depth, stmt)) = call.on_error {
            outcomes.insert("error".to_owned(), json!([depth, stmt]));
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
        Stmt::new(ast)
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

/// A stream of patches on the page whose signals are `S`, which receives arguments `A`.
///
/// It is served on `GET` with `RouterExt::stream`, which accepts only this
/// type, and an [`Action`] goes to `RouterExt::action`. A call to it can
/// [`reconnect`](StreamCall::reconnect), which a call to an `Action` cannot,
/// because sending a `POST` again could repeat a change the server already made.
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
/// const UPDATES: StreamAction<Feed> = StreamAction::new("/updates");
/// let _feed = div().on_load(UPDATES.call().reconnect());
/// ```
pub struct StreamAction<S, A = ()> {
    base: Base,
    path: &'static str,
    marker: PhantomData<fn(S, A)>,
}

impl<S, A> StreamAction<S, A> {
    /// A stream served on `GET path`.
    ///
    /// # Panics
    ///
    /// Panics if `path` does not start with `/`. In a `const`, that is a
    /// compile error.
    #[must_use]
    pub const fn new(path: &'static str) -> Self {
        assert!(
            !path.is_empty() && path.as_bytes()[0] == b'/',
            "a stream path starts with `/`"
        );
        Self {
            base: Base::ROOT,
            path,
            marker: PhantomData,
        }
    }

    /// The same stream under `base`, which the page's calls put before the path.
    ///
    /// The router registers [`StreamAction::path`] without it. See [`Base`].
    #[must_use]
    pub const fn under(self, base: Base) -> Self {
        Self { base, ..self }
    }

    /// The path the stream is served on, without its [`Base`].
    #[must_use]
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// A call to this stream with `args`, evaluated in the browser when the call runs.
    ///
    /// A call that [reconnects](StreamCall::reconnect) evaluates `args`
    /// again before each attempt.
    #[must_use]
    pub fn call_with(&self, args: impl IntoExpr<S, A>) -> StreamCall<S> {
        StreamCall(Call::new(
            Method::Get,
            self.base,
            self.path,
            args.into_expr().into_json(),
        ))
    }
}

impl<S> StreamAction<S, ()> {
    /// A call to this stream, which takes no arguments.
    #[must_use]
    pub fn call(&self) -> StreamCall<S> {
        StreamCall(Call::new(Method::Get, self.base, self.path, Value::Null))
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

    /// Opens the stream again when its connection fails, breaks off, or answers 5xx.
    ///
    /// The first attempt waits 1 second, and each later one twice as long,
    /// up to 30 seconds. A connection that the server answered with a 2xx
    /// status starts the wait over. While the stream is failing, the
    /// [`error`](StreamCall::error) signal holds the last failure.
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
    /// // The handler sends the messages after `last_id` and patches `last_id`.
    /// const SINCE: StreamAction<Chat, i32> = StreamAction::new("/messages");
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
/// Declare one as a `const`, attach it with
/// [`El::node_ref`](crate::html::El::node_ref), and use its statements in
/// handlers. Inside a list item the browser looks for the element in the same
/// item first, so one `NodeRef` serves every item.
///
/// Outside a list, one name sits on one element. [`Page::render`](crate::Page::render)
/// and [`El::render`](crate::html::El::render) panic when two elements the
/// browser could both reach carry one name, which a test that renders the
/// page catches. A [`Fragment`](crate::Fragment) sent again is rendered on
/// its own, so the server cannot compare it with the rest of the page. The
/// browser does, and writes a warning to the console when the patch adds a
/// name or an id that another element it can reach already carries. A
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
/// const FIELD: NodeRef<Search> = NodeRef::new("search");
/// let _ = div()
///     .child(input().node_ref(FIELD).bind(Search::query()))
///     .child(button().on_click(FIELD.focus()).text("Search"));
/// ```
pub struct NodeRef<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> NodeRef<S> {
    /// A reference named `name`.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty. In a `const`, that is a compile error.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        assert!(!name.is_empty(), "a node ref name is never empty");
        Self {
            name,
            marker: PhantomData,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        self.name
    }

    fn dom(self, op: &str) -> Stmt<S> {
        Stmt::new(json!(["dom", op, self.name]))
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

    /// A statement that shows the element if it is a popover that is not open.
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

/// A named `<dialog>` on a page whose signals are `S`, which statements open and close.
///
/// [`DialogRef::dialog`] is the only way to attach one, and it builds the
/// `<dialog>` element itself, so [`show_modal`](DialogRef::show_modal) and
/// [`close`](DialogRef::close) always name a dialog. A [`NodeRef`] has
/// neither, since it may sit on any element.
///
/// The name shares one namespace with the names of [`NodeRef`]s, and
/// rendering panics where two elements the browser can both reach carry one.
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
/// const PALETTE: DialogRef<Palette> = DialogRef::new("palette");
/// let page = div()
///     .child(button().on_click(PALETTE.show_modal()).text("Commands"))
///     .child(
///         PALETTE
///             .dialog()
///             .child(input().bind(Palette::query()))
///             .child(button().on_click(PALETTE.close()).text("Close")),
///     );
/// assert!(page.render().contains(r#"<dialog data-hs-ref="palette">"#));
/// ```
pub struct DialogRef<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> DialogRef<S> {
    /// A reference named `name`.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty. In a `const`, that is a compile error.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        assert!(!name.is_empty(), "a dialog ref name is never empty");
        Self {
            name,
            marker: PhantomData,
        }
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    enum Page {}

    const FIND: Action<Page, String> = Action::get("/find");
    const SAVE: Action<Page> = Action::post("/save");
    const FEED: StreamAction<Page> = StreamAction::new("/feed");

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
    fn a_stream_call_is_a_get_that_can_reconnect() {
        assert_eq!(
            FEED.call().reconnect().into_stmt().as_json(),
            &json!(["call", "GET", "/feed", null, null, null, ["reconnect"]])
        );
    }
}
