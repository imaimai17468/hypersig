//! Server-driven hypermedia with typed signals.
//!
//! The server owns the state, renders HTML, and answers browser events with
//! patches over Server-Sent Events. A small runtime in the browser keeps a
//! store of signals for local reactivity and applies the patches. Signal
//! names, attribute expressions, action endpoints, page links, and patch
//! targets are all produced from Rust types, so a misspelled signal, a type
//! mismatch in an expression, an action called from the wrong page, or a link
//! whose parameters do not fit its route is a compile error.
//! [`Page::render`] evaluates the expressions with the initial signals, so the
//! first HTML already shows what the runtime would compute.
//!
//! ```
//! use hypersig::html::{button, div, span};
//! use hypersig::{Action, Page, Patches, Signals};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Signals, Serialize, Deserialize)]
//! struct Counter {
//!     count: i32,
//! }
//!
//! const RESET: Action<Counter> = Action::post("/reset");
//!
//! let count = Counter::count();
//! let page = Page::new("Counter", Counter { count: 0 }).child(
//!     div()
//!         .child(span().bind_text(count.to_text()))
//!         .child(button().on_click(count.set(count + 1)).text("+"))
//!         .child(button().show(count.gt(0)).on_click(RESET.call()).text("reset")),
//! );
//! assert!(page.render().contains("data-hs-on-click"));
//!
//! // The handler for RESET answers with a patch built from the same types.
//! let reply: Patches<Counter> = count.patch(0).into();
//! assert!(reply.to_event_stream().starts_with("event: hs-signals"));
//! ```
//!
//! With the `axum` feature (on by default), [`routes!`] declares the pages
//! at their [`Route`]s together with their handlers, and `RouterExt` serves
//! the runtime and the actions.

extern crate self as hypersig;

mod action;
#[cfg(feature = "axum")]
mod axum;
mod expr;
mod form;
pub mod html;
mod page;
mod patch;
mod route;
mod signal;
mod ssr;

#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
struct Readme;

#[cfg(doctest)]
#[doc = include_str!("../../../README.ja.md")]
struct ReadmeJa;

pub use action::{Action, Call, Method, NodeRef};
#[cfg(feature = "axum")]
pub use axum::{PageResponse, RequestError, RouterExt};
pub use expr::{
    Comparable, Expr, IntoExpr, IntoStmt, Number, NumberFormat, Stmt, Textual, if_else,
};
pub use form::{FieldErrors, Form, Rules, Validate};
pub use hypersig_proc::{Search, Segment, Signals};
pub use page::Page;
pub use patch::{Fragment, Patches, SignalPatch, fragment};
pub use route::{Extends, Link, Params, Rest, Route, Search, SearchValue, Segment, Url};
pub use signal::{Item, Local, MAX_DEPTH, Root, Sig, Signals};

/// Typed atomic CSS, from the `hypersig_style` crate.
///
/// The crate is split off because its code is generated from MDN's CSS data
/// and is larger than the rest of hypersig. Reach it through this path.
#[doc(inline)]
pub use hypersig_style as style;

/// The path [`Page`] loads the runtime from.
pub const RUNTIME_PATH: &str = "/_hypersig/runtime.js";

/// The browser runtime, built from `runtime/` in the repository.
pub const RUNTIME_JS: &str = include_str!("../assets/hypersig.js");

#[doc(hidden)]
pub mod _private {
    #[cfg(feature = "axum")]
    pub use ::axum::Router;

    pub use crate::route::SearchFields;
    pub use crate::route::sealed::Sealed;
    use crate::route::{Extends, Params, Route, Search};
    use crate::signal::{Path, Sig};

    /// Called by `routes!` for a page at `path`.
    #[must_use]
    pub const fn route<P: Params, Q: Search>(path: &'static str) -> Route<P, Q> {
        Route::new(path)
    }

    /// Called by `routes!` for a page at `path` under `parent`.
    #[must_use]
    pub const fn child<P: Params, Q: Search, C: Params + Extends<P>, R: Search>(
        parent: &Route<P, Q>,
        path: &'static str,
    ) -> Route<C, R> {
        parent.child(path)
    }

    /// Called by `#[derive(Signals)]` with a field name of `S`.
    #[must_use]
    pub fn sig<S, T>(field: &'static str) -> Sig<S, T> {
        Sig::from_path(Path::field(field))
    }
}
