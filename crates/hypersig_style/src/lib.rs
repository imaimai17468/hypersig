//! Typed atomic CSS for hypersig.
//!
//! A [`Style`] is built from one method per standard CSS property, and per
//! anchor positioning property, which MDN's data still marks experimental
//! although every major engine ships it. A misspelled property or keyword
//! does not compile. Each declaration becomes
//! one class whose name is derived from the declaration, and a [`Sheet`]
//! collects the styles a page uses and writes each rule once.
//!
//! The property list, each property's keywords, the pseudo-classes, the
//! pseudo-elements, and the named colors are generated from MDN's CSS data
//! (`mdn-data` 2.37.2). Where a property accepts a value made of one keyword
//! or one value of a type in [`value`], that value is typed. The forms the
//! generator leaves out, such as `margin: 0 auto` or a `calc()`, go through
//! [`value::Raw`] and are not checked. Ranges are not checked either, so a
//! negative width is accepted and the browser drops it.
//!
//! Use it through the `hypersig` crate.
// A library's caller owns standard output and standard error, so the library writes to neither.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// A library hands a failure back to its caller as an error, and marks `pub` only what it exports.
#![warn(clippy::unwrap_used, unreachable_pub)]
#![doc(
    html_logo_url = "https://imaimai17468.github.io/hypersig/icon.svg",
    html_favicon_url = "https://imaimai17468.github.io/hypersig/icon.svg"
)]

mod condition;
// Written by tests/generate.rs, whose output is kept as it writes it.
#[rustfmt::skip]
mod generated;
mod global;
mod keyframes;
pub mod pattern;
mod sheet;
mod style;
mod token;
pub mod value;

pub use condition::{Condition, Container, Media};
pub use generated::{Property, PseudoClass, PseudoElement};
pub use global::{Element, FontFace, Global, Selector};
pub use keyframes::Keyframes;
pub use sheet::{BASE_LAYER, Bucket, LAYER, Sheet, TOGGLED_LAYER, TOKEN_LAYER};
pub use style::Style;
pub use token::{Theme, Token, TokenValue};
