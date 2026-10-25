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
#![doc(
    html_logo_url = "https://imaimai17468.github.io/hypersig/icon.svg",
    html_favicon_url = "https://imaimai17468.github.io/hypersig/icon.svg"
)]

extern crate self as hypersig;

mod action;
#[cfg(feature = "axum")]
mod axum;
mod date;
mod exact;
mod expr;
mod finite;
mod form;
mod head;
mod href;
pub mod html;
mod intl;
mod label;
mod page;
mod patch;
#[cfg(feature = "axum")]
mod query;
mod route;
mod signal;
mod ssr;
mod tape;
mod upload;

#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
struct Readme;

#[cfg(doctest)]
#[doc = include_str!("../../../README.ja.md")]
struct ReadmeJa;

pub use action::{Action, Call, DialogRef, Method, NodeRef, StreamAction, StreamCall};
#[cfg(feature = "axum")]
pub use axum::{
    ActionResponse, Handler, PageResponse, RequestError, RequestKind, RouterExt, Unserved,
};
pub use date::{DateFormat, DateStyle, TimeStyle, TimeZone, Timestamp};
pub use expr::{
    CharClass, Comparable, Expr, IntoExpr, IntoStmt, Number, NumberFormat, Stmt, Textual, if_else,
};
pub use form::{FieldErrors, Form, Rules, Validate};
pub use head::{OpenGraph, OpenGraphLocale, OpenGraphType, Origin, OriginError};
pub use href::{IntoHref, SafeUrl, UnsafeUrl};
pub use hypersig_proc::{Search, Segment, Signals};
pub use intl::{IntlLocale, PluralCategory};
pub use page::Page;
pub use patch::{Fragment, ListPatch, Patches, SignalPatch, fragment};
pub use route::{
    Base, Extends, Link, Params, Rest, Route, Search, SearchScalar, SearchValue, Segment, Url,
    Variants,
};
pub use signal::{Item, Local, MAX_DEPTH, Part, Root, Sig, Signals};
pub use upload::{FileInput, Upload};
#[cfg(feature = "axum")]
pub use upload::{StreamedFile, UploadStream, Uploaded, UploadedFile};

/// Typed atomic CSS, from the `hypersig_style` crate.
///
/// The crate is split off because its code is generated from MDN's CSS data
/// and is larger than the rest of hypersig. Reach it through this path.
#[doc(inline)]
pub use hypersig_style as style;

/// The path [`Page`] loads the runtime from, which names a hash of [`RUNTIME_JS`].
///
/// It reads `/_hypersig/runtime.` followed by 16 hex digits and `.js`. The
/// digits change whenever the runtime does, so a browser may keep the file
/// for as long as it likes, and `RouterExt::hypersig_runtime` tells it to.
pub const RUNTIME_PATH: &str = match std::str::from_utf8(&RUNTIME_PATH_BYTES) {
    Ok(path) => path,
    Err(_) => panic!("the runtime path is ASCII"),
};

/// The browser runtime, built from `runtime/` in the repository.
pub const RUNTIME_JS: &str = include_str!("../assets/hypersig.js");

/// [`RUNTIME_JS`] compressed with Brotli by the same `pnpm build`, for a browser that accepts it.
#[cfg(all(
    feature = "axum",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
const RUNTIME_BR: &[u8] = include_bytes!("../assets/hypersig.js.br");

/// [`RUNTIME_PATH`] as bytes, with the FNV-1a hash of [`RUNTIME_JS`] in hex.
///
/// The compiler computes it, so the path needs no build script and changes
/// in the same build as the runtime.
const RUNTIME_PATH_BYTES: [u8; RUNTIME_PREFIX.len() + HASH_DIGITS + RUNTIME_SUFFIX.len()] = {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let js = RUNTIME_JS.as_bytes();
    let mut hash = FNV_OFFSET_BASIS;
    let mut i = 0;
    while i < js.len() {
        hash ^= js[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    let mut out = [0u8; RUNTIME_PREFIX.len() + HASH_DIGITS + RUNTIME_SUFFIX.len()];
    let mut i = 0;
    while i < RUNTIME_PREFIX.len() {
        out[i] = RUNTIME_PREFIX[i];
        i += 1;
    }
    let mut digit = 0;
    while digit < HASH_DIGITS {
        let shift = 4 * (HASH_DIGITS - 1 - digit);
        out[RUNTIME_PREFIX.len() + digit] = HEX[((hash >> shift) & 0xf) as usize];
        digit += 1;
    }
    let mut i = 0;
    while i < RUNTIME_SUFFIX.len() {
        out[RUNTIME_PREFIX.len() + HASH_DIGITS + i] = RUNTIME_SUFFIX[i];
        i += 1;
    }
    out
};

/// What [`RUNTIME_PATH`] holds before the hash.
const RUNTIME_PREFIX: &[u8] = b"/_hypersig/runtime.";

/// What [`RUNTIME_PATH`] holds after the hash.
const RUNTIME_SUFFIX: &[u8] = b".js";

/// The number of hex digits that write a 64-bit hash.
const HASH_DIGITS: usize = 16;

/// The 64-bit offset basis of FNV-1a, from the FNV specification.
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

/// The 64-bit prime of FNV-1a, from the FNV specification.
const FNV_PRIME: u64 = 0x0100_0000_01b3;

#[doc(hidden)]
pub mod _private {
    #[cfg(feature = "axum")]
    pub use ::axum::Router;

    #[cfg(feature = "axum")]
    pub use crate::axum::UnservedPage;

    pub use crate::route::SearchFields;
    pub use crate::route::sealed::Sealed;
    use crate::route::{Base, Extends, Params, Route, Search};
    use crate::signal::{Path, Sig};

    /// Called by `routes!` for a page at `path` under `base`.
    #[must_use]
    pub const fn route<P: Params, Q: Search>(base: Base, path: &'static str) -> Route<P, Q> {
        Route::new(path).under(base)
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
