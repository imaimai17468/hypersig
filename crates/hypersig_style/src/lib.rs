//! Typed atomic CSS for hypersig.
//!
//! A [`Style`] is built from one method per standard CSS property, so a
//! misspelled property or keyword does not compile. Each declaration becomes
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

mod condition;
// Written by tests/generate.rs, whose output is kept as it writes it.
#[rustfmt::skip]
mod generated;
mod sheet;
mod style;
pub mod value;

pub use condition::{Condition, Media};
pub use generated::{Property, PseudoClass, PseudoElement};
pub use sheet::{Bucket, LAYER, Sheet, TOGGLED_LAYER};
pub use style::Style;
