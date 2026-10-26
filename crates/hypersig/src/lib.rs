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
//! hypersig::routes! {
//!     fn actions(Router<()>);
//!     action RESET: Action<Counter> = post "/reset" => reset;
//! }
//!
//! // The handler for RESET answers with a patch built from the same types.
//! async fn reset((): (), (): ()) -> Patches<Counter> {
//!     Counter::count().patch(0).into()
//! }
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
//! let reply: Patches<Counter> = count.patch(0).into();
//! assert!(reply.to_event_stream().starts_with("event: hs-signals"));
//! let _app: axum::Router = actions(axum::Router::new());
//! ```
//!
//! With the `axum` feature (on by default), [`routes!`] declares the pages,
//! actions, streams, and uploads together with their handlers, and
//! `RouterExt` serves the runtime.
#![doc(
    html_logo_url = "https://imaimai17468.github.io/hypersig/icon.svg",
    html_favicon_url = "https://imaimai17468.github.io/hypersig/icon.svg"
)]

extern crate self as hypersig;

mod action;
mod asset;
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
pub use asset::Asset;
#[cfg(all(
    feature = "axum",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub use axum::serve;
#[cfg(feature = "axum")]
pub use axum::{
    ActionResponse, Handler, IntoActionResponse, PageResponse, PatchStream, RequestError,
    RequestKind, RouterExt, Unserved,
};
pub use date::{DateFormat, DateStyle, TimeStyle, TimeZone, Timestamp};
pub use expr::{
    CharClass, Comparable, Expr, IntoExpr, IntoStmt, Number, NumberFormat, Stmt, Textual, if_else,
};
pub use form::{FieldErrors, Form, Rules, Validate};
pub use head::{
    LanguageTag, LanguageTagError, OpenGraph, OpenGraphLocale, OpenGraphType, Origin, OriginError,
};
pub use href::{IntoHref, SafeUrl, UnsafeUrl};
pub use hypersig_proc::{Search, Segment, Signals};
pub use intl::{IntlLocale, PluralCategory};
pub use page::{Framing, Page};
pub use patch::{Fragment, ListPatch, Patches, SignalPatch, fragment};
pub use route::{
    Base, Extends, Link, Params, Rest, Route, Search, SearchScalar, SearchValue, Segment, Url,
    Variants,
};
pub use signal::{
    Computed, FromPage, IsComputed, Item, Local, MAX_DEPTH, Part, Root, Sig, Signals, Writable,
};
pub use upload::{FileInput, Upload};
#[cfg(feature = "axum")]
pub use upload::{FileName, StreamedFile, UploadStream, Uploaded, UploadedFile};

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

    /// Called by `routes!` for an action served on `method path` under `base`.
    #[must_use]
    pub const fn action<S, A>(
        base: Base,
        method: crate::Method,
        path: &'static str,
    ) -> crate::Action<S, A> {
        crate::Action::at(method, base, path)
    }

    /// Called by `routes!` for a stream served on `GET path` under `base`.
    #[must_use]
    pub const fn stream<S, A>(base: Base, path: &'static str) -> crate::StreamAction<S, A> {
        crate::StreamAction::at(base, path)
    }

    /// Called by `routes!` for an upload served on `POST path` under `base`.
    #[must_use]
    pub const fn upload<S, A>(
        base: Base,
        path: &'static str,
        max_bytes: usize,
    ) -> crate::Upload<S, A> {
        crate::Upload::at(base, path, max_bytes)
    }

    /// The limit `routes!` gives an upload line without `max_bytes`.
    pub const UPLOAD_MAX_BYTES: usize = crate::upload::DEFAULT_MAX_BYTES;

    /// Called by `#[derive(Signals)]` with a field name of `S`.
    #[must_use]
    pub fn sig<S, T>(field: &'static str) -> Sig<S, T> {
        Sig::from_path(Path::field(field))
    }

    /// Called by `#[derive(Signals)]` for a field marked `#[signals(computed)]`.
    #[must_use]
    pub fn computed_sig<S, T>(field: &'static str) -> Sig<S, T, crate::Computed> {
        Sig::from_path(Path::field(field))
    }

    /// Called by `attr!` inside a `const` block, so a name `Attr` refuses
    /// is a compile error.
    #[must_use]
    pub const fn attr(name: &'static str) -> crate::html::Attr {
        crate::html::Attr::new(name)
    }

    /// Called by `tag!` inside a `const` block, so a name `Tag` refuses is
    /// a compile error.
    #[must_use]
    pub const fn tag(name: &'static str) -> crate::html::Tag {
        crate::html::Tag::new(name)
    }

    /// Called by `node_ref!` with the path of its constant.
    #[must_use]
    pub const fn node_ref<S>(name: &'static str) -> crate::NodeRef<S> {
        crate::NodeRef::new(name)
    }

    /// Called by `dialog_ref!` with the path of its constant.
    #[must_use]
    pub const fn dialog_ref<S>(name: &'static str) -> crate::DialogRef<S> {
        crate::DialogRef::new(name)
    }

    /// Whether `a` and `b` carry one name, for a check in a `const`, such as
    /// `hypersig_ui`'s refusal of one constant as both parts of a combobox.
    #[must_use]
    pub const fn same_node_ref<S>(a: crate::NodeRef<S>, b: crate::NodeRef<S>) -> bool {
        let (a, b) = (a.name().as_bytes(), b.name().as_bytes());
        if a.len() != b.len() {
            return false;
        }
        let mut i = 0;
        while i < a.len() {
            if a[i] != b[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

/// Declares a [`NodeRef`] constant named after its own path.
///
/// The input is the constant's declaration without a value. The macro writes
/// that constant, with the name `module_path!()` and the identifier give it,
/// and a `const _` that reads it back through `self::`. The second item
/// stops the build where the declaration sits in a function body, because
/// two functions in one module could otherwise each declare a constant of
/// one identifier and give two constants one name. Attributes go on both
/// items.
///
/// ```
/// use hypersig::html::input;
/// use hypersig::{NodeRef, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Search {
///     query: String,
/// }
///
/// hypersig::node_ref!(pub const FIELD: NodeRef<Search>);
///
/// # fn main() {
/// let field = input().node_ref(FIELD).bind(Search::query());
/// assert!(field.render().contains(&format!(r#"data-hs-ref="{}::FIELD""#, module_path!())));
/// # }
/// ```
///
/// # Compile errors
///
/// A declaration inside a function body, where `self::NAME` does not reach
/// the constant, and a type other than `NodeRef<..>`.
#[macro_export]
macro_rules! node_ref {
    ($(#[$attr:meta])* $vis:vis const $name:ident: NodeRef<$signals:ty>) => {
        $(#[$attr])*
        $vis const $name: $crate::NodeRef<$signals> =
            $crate::_private::node_ref(::core::concat!(::core::module_path!(), "::", ::core::stringify!($name)));
        $(#[$attr])*
        const _: () = {
            let _ = self::$name;
        };
    };
}

/// Declares a [`DialogRef`] constant named after its own path.
///
/// It writes the same two items as [`node_ref!`], with a `DialogRef`.
///
/// ```
/// use hypersig::{DialogRef, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Palette {}
///
/// hypersig::dialog_ref!(const PALETTE: DialogRef<Palette>);
///
/// # fn main() {
/// let html = PALETTE.dialog().render();
/// assert!(html.contains(&format!(r#"<dialog data-hs-ref="{}::PALETTE">"#, module_path!())));
/// # }
/// ```
///
/// # Compile errors
///
/// A declaration inside a function body, and a type other than
/// `DialogRef<..>`.
#[macro_export]
macro_rules! dialog_ref {
    ($(#[$attr:meta])* $vis:vis const $name:ident: DialogRef<$signals:ty>) => {
        $(#[$attr])*
        $vis const $name: $crate::DialogRef<$signals> =
            $crate::_private::dialog_ref(::core::concat!(::core::module_path!(), "::", ::core::stringify!($name)));
        $(#[$attr])*
        const _: () = {
            let _ = self::$name;
        };
    };
}

/// Builds an [`Attr`](html::Attr) for a name with no constant in [`html::attr`].
///
/// Such a name is a `data-*` attribute, such as `data-lat`, or an attribute
/// of a custom element. The name is checked at compile time against the rules of
/// [`Attr`](html::Attr), so a name that could run a script stops the build:
///
/// ```
/// use hypersig::html::div;
///
/// let map = div::<()>().attr(hypersig::attr!("data-zoom"), "12");
/// assert!(map.render().contains(r#"data-zoom="12""#));
/// ```
///
/// ```compile_fail
/// let _ = hypersig::html::img::<()>().attr(hypersig::attr!("onerror"), "alert(1)");
/// ```
#[macro_export]
macro_rules! attr {
    ($name:literal) => {
        const { $crate::_private::attr($name) }
    };
}

/// Builds a [`Tag`](html::Tag) for [`html::el`], checked at compile time.
///
/// ```
/// use hypersig::html::el;
///
/// let icon = el::<()>(hypersig::tag!("svg"));
/// assert_eq!(icon.render(), "<svg></svg>");
/// ```
///
/// ```compile_fail
/// let _ = hypersig::html::el::<()>(hypersig::tag!("img onerror=alert(1)"));
/// ```
#[macro_export]
macro_rules! tag {
    ($name:literal) => {
        const { $crate::_private::tag($name) }
    };
}
