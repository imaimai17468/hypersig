//! A position in the viewport, which [`El::on_context_menu`](super::El::on_context_menu) hands its statement.

use serde::{Deserialize, Serialize};

/// A position in the viewport, in CSS pixels from its top left corner.
///
/// [`El::on_context_menu`](super::El::on_context_menu) hands one to its
/// statement as an [`Expr`](crate::Expr), which
/// [`NodeRef::show_popover_at`](crate::NodeRef::show_popover_at) opens a
/// popover at, and which a `Sig<S, Point>` can keep.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    /// The distance from the viewport's left edge.
    pub x: f64,
    /// The distance from the viewport's top edge.
    pub y: f64,
}
