//! Separators: a line between groups of content.

use hypersig::html::{El, Orientation, attr, div, hr};
use hypersig::style::Style;
use hypersig::style::value::{AlignSelf, BorderStyle, px};

use crate::token;

/// A one-pixel line in [`token::border`] that runs along `orientation`.
///
/// A horizontal one is an `<hr>`, and a vertical one is a `<div>` with
/// `role="separator"` and `aria-orientation="vertical"`, so that a screen
/// reader announces both as separators.
///
/// ```
/// use hypersig::html::Orientation;
/// use hypersig::Signals;
/// use hypersig_ui::separator;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// let line = separator::<App>(Orientation::Vertical);
/// assert!(line.render().contains(r#"aria-orientation="vertical""#));
/// ```
#[must_use]
pub fn separator<S>(orientation: Orientation) -> El<S> {
    let line = Style::new()
        .flex_shrink(0.0)
        .border_style(BorderStyle::None)
        .background_color(token::border().var());
    match orientation {
        Orientation::Vertical => div()
            .attr(attr::ROLE, "separator")
            .attr(attr::ARIA_ORIENTATION, "vertical")
            .css(line.width(px(1)).align_self(AlignSelf::Stretch)),
        _ => hr().css(line.height(px(1)).width(hypersig::style::value::pct(100))),
    }
}
