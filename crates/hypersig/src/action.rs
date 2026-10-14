//! Server endpoints that the browser calls with the page's signals.

use std::fmt;
use std::marker::PhantomData;

use serde::Serialize;
use serde_json::Value;

use crate::expr::Stmt;

/// The HTTP method an [`Action`] is served on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Method {
    /// `GET`. Signals and arguments travel in the query string.
    Get,
    /// `POST`. Signals travel in the JSON body and arguments in the query string.
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

/// A server endpoint that receives the signals `S` and arguments `A`.
///
/// Declare one as a `const` and use the same value on both sides: the page
/// calls it with [`Action::call`] or [`Action::call_with`], and the router
/// registers its handler with `RouterExt::action`. Both sides then agree on the
/// path, the method, the signal struct, and the argument type.
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
/// let _click = INCREMENT.call();
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
    /// A statement that calls this action with the page's current signals.
    #[must_use]
    pub fn call(&self) -> Stmt<S> {
        Stmt::call(self.method.as_str(), self.path, Value::Null)
    }
}

impl<S, A: Serialize> Action<S, A> {
    /// A statement that calls this action with `args` and the current signals.
    ///
    /// # Panics
    ///
    /// Panics if `args` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn call_with(&self, args: &A) -> Stmt<S> {
        let args = serde_json::to_value(args).expect("action arguments serialize to JSON");
        Stmt::call(self.method.as_str(), self.path, args)
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
