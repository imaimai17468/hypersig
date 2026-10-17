//! Server endpoints that the browser calls with typed arguments.

use std::fmt;
use std::marker::PhantomData;

use serde_json::{Value, json};

use crate::expr::{IntoExpr, IntoStmt, Stmt};
use crate::signal::Sig;

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

    /// An action served on `GET path`. Use it for a long-lived stream of patches.
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
            path,
            marker: PhantomData,
        }
    }

    /// The path the action is served on.
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
        Call::new(self.method, self.path, Value::Null)
    }
}

impl<S, A> Action<S, A> {
    /// A call to this action with `args`.
    ///
    /// `args` is evaluated in the browser when the call runs, so it can be a
    /// literal such as `&id` or an expression such as `item.at(Todo::id())`.
    #[must_use]
    pub fn call_with(&self, args: impl IntoExpr<S, A>) -> Call<S> {
        Call::new(self.method, self.path, args.into_expr().into_json())
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
    path: &'static str,
    args: Value,
    indicator: Value,
    error: Value,
    marker: PhantomData<fn() -> S>,
}

impl<S> Call<S> {
    fn new(method: Method, path: &'static str, args: Value) -> Self {
        Self {
            method,
            path,
            args,
            indicator: Value::Null,
            error: Value::Null,
            marker: PhantomData,
        }
    }

    /// Sets `flag` to true while the request is in flight and to false after.
    #[must_use]
    pub fn indicator(mut self, flag: Sig<S, bool>) -> Self {
        self.indicator = Value::from(flag.path());
        self
    }

    /// Sets `message` to the error text when the call fails, and to `""` when it succeeds.
    #[must_use]
    pub fn error(mut self, message: Sig<S, String>) -> Self {
        self.error = Value::from(message.path());
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
        Stmt::new(json!([
            "call",
            call.method.as_str(),
            call.path,
            call.args,
            call.indicator,
            call.error
        ]))
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

/// A named element on a page whose signals are `S`, for imperative DOM calls.
///
/// Declare one as a `const`, attach it with
/// [`El::node_ref`](crate::html::El::node_ref), and use its statements in
/// handlers. Inside a list item the browser looks for the element in the same
/// item first, so one `NodeRef` serves every item.
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
