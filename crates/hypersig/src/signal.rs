//! Signal structs and typed references to their fields.

use std::fmt;
use std::marker::PhantomData;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::expr::{Expr, IntoExpr, Stmt};
use crate::patch::SignalPatch;

/// A struct whose fields are the signals of one page.
///
/// Derive it with `#[derive(Signals)]` next to serde's `Serialize` and
/// `Deserialize`. The derive adds one associated function per field, named
/// after the field, that returns a [`Sig`] for it:
///
/// ```
/// use hypersig::{Sig, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Counter {
///     count: i32,
/// }
///
/// let count: Sig<Counter, i32> = Counter::count();
/// assert_eq!(count.path(), "count");
/// ```
///
/// The browser keeps the signals as one JSON object and sends the whole object
/// with every action, which the server deserializes back into this struct.
pub trait Signals: Serialize + DeserializeOwned + Send + 'static {}

/// A typed reference to the signal at one field of the signal struct `S`.
///
/// `T` is the field's type. Values come only from the accessor functions that
/// `#[derive(Signals)]` generates, so the path always names a real field.
pub struct Sig<S, T> {
    path: &'static str,
    marker: PhantomData<fn() -> (S, T)>,
}

impl<S, T> Sig<S, T> {
    pub(crate) fn from_path(path: &'static str) -> Self {
        assert!(!path.is_empty(), "a signal path is never empty");
        Self {
            path,
            marker: PhantomData,
        }
    }

    /// The field name this signal is stored under.
    #[must_use]
    pub fn path(self) -> &'static str {
        self.path
    }

    /// A statement that assigns `value` to this signal in the browser.
    #[must_use]
    pub fn set(self, value: impl IntoExpr<S, T>) -> Stmt<S> {
        Stmt::set(self.path, value.into_expr())
    }

    /// A patch that sets this signal to `value` when the server sends it.
    ///
    /// # Panics
    ///
    /// Panics if `value` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn patch(self, value: T) -> SignalPatch<S>
    where
        T: Serialize,
    {
        SignalPatch::single(self.path, &value)
    }

    pub(crate) fn expr(self) -> Expr<S, T> {
        Expr::sig(self.path)
    }
}

impl<S, T> Clone for Sig<S, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T> Copy for Sig<S, T> {}

impl<S, T> fmt::Debug for Sig<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Sig").field(&self.path).finish()
    }
}
