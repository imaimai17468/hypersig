//! Keyboard keys drawn as small caps in running text.

use hypersig::html::{El, kbd as kbd_el};
use hypersig::style::Style;
use hypersig::style::value::{
    AlignItems, Display, FontWeight, JustifyContent, LineHeight, UserSelect, WhiteSpace, px, raw,
    rem,
};

use crate::token;

/// A `<kbd>` that shows a key or a shortcut the visitor presses.
///
/// A screen reader reads its text as it stands, so write the key's name in
/// words where a symbol would be read as something else, such as "Ctrl"
/// rather than "^". For a shortcut, place one `kbd` per key side by side.
///
/// ```
/// use hypersig::Signals;
/// use hypersig::html::p;
/// use hypersig_ui::kbd;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Help {
///     open: bool,
/// }
///
/// let hint = p::<Help>().text("Press ").child(kbd().text("Esc")).text(" to close.");
/// assert!(hint.render().contains("<kbd"));
/// ```
#[must_use]
pub fn kbd<S>() -> El<S> {
    kbd_el().css(
        Style::new()
            .display(Display::InlineFlex)
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .gap(rem(0.25))
            .height(rem(1.25))
            .min_width(rem(1.25))
            .padding_inline(rem(0.25))
            .border_radius(px(4))
            .background_color(token::muted().var())
            .color(token::muted_foreground().var())
            .font_family(raw("inherit"))
            .font_size(rem(0.75))
            .line_height(LineHeight::Length(rem(1)))
            .font_weight(FontWeight::Number(500.0))
            .white_space(WhiteSpace::Nowrap)
            .user_select(UserSelect::None),
    )
}
