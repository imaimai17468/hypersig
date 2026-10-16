//! Selects: a labeled native `<select>` bound to a signal.

use hypersig::html::{El, div, label, select};
use hypersig::style::pattern::stack;
use hypersig::style::value::{Appearance, BorderStyle, Cursor, Display, Position, px, raw, rem};
use hypersig::style::{PseudoElement, Style};
use hypersig::{Expr, Sig};
use serde::Serialize;

use crate::field::{field_style, label_style};
use crate::look::{self, icon};
use crate::token;

/// A select labeled `title`, offering `options`, that sets `selected` to the
/// chosen option's value.
///
/// It is a native `<select>`, which a screen reader, a keyboard, and a
/// phone's picker wheel already handle. The closed control is drawn like a
/// text field with a chevron. Where the browser supports
/// `appearance: base-select` and `::picker(select)`, which
/// browser-compat-data 8.1.6 lists for Chrome 135 and Safari 27, the open
/// list is drawn as a popover card too. Elsewhere it is the browser's own
/// picker. The label's `for` is derived from the signal's path.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::select_field;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
/// enum Region {
///     Tokyo,
///     Osaka,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Deploy {
///     region: Region,
/// }
///
/// let region = select_field(
///     "Region",
///     Deploy::region(),
///     [(Region::Tokyo, "Tokyo"), (Region::Osaka, "Osaka")],
/// );
/// assert!(region.render().contains(r#"for="ui-region-select""#));
/// ```
#[must_use]
pub fn select_field<S, T: Serialize + PartialEq, L: Into<String>>(
    title: impl Into<String>,
    selected: Sig<S, T>,
    options: impl IntoIterator<Item = (T, L)>,
) -> El<S> {
    let id = crate::id::of(selected, "select");
    div()
        .css(stack(rem(0.5)))
        .child(
            label()
                .attr("for", id.clone())
                .css(label_style())
                .text(title),
        )
        .child(
            div()
                .css(Style::new().position(Position::Relative))
                .child(
                    select()
                        .id(id)
                        .css(
                            field_style()
                                .height(rem(2.25))
                                .padding_inline_end(rem(2))
                                .appearance(Appearance::None)
                                .background_color(token::background().var())
                                .cursor(Cursor::Pointer)
                                .pseudo(
                                    PseudoElement::PickerIcon,
                                    Style::new().display(Display::None),
                                )
                                .pseudo(PseudoElement::PickerSelect, picker_style()),
                        )
                        // `base-select` sits in the toggled layer above
                        // `none`, so a browser that rejects the value keeps
                        // `appearance: none` from the atoms layer.
                        .css_when(
                            Style::new().appearance(Appearance::BaseSelect),
                            Expr::lit(&true),
                        )
                        .bind_select(selected, options),
                )
                .child(
                    icon(look::icon::CHEVRON_DOWN, rem(1)).css(
                        Style::new()
                            .position(Position::Absolute)
                            .right(rem(0.625))
                            .top(rem(0.625))
                            .color(token::muted_foreground().var()),
                    ),
                ),
        )
}

/// The open list of a `base-select` select, drawn like a menu.
fn picker_style() -> Style {
    Style::new()
        .appearance(Appearance::BaseSelect)
        .margin_block(rem(0.25))
        .padding(rem(0.25))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(px(6))
        .background_color(token::popover().var())
        .color(token::popover_foreground().var())
        .box_shadow(raw(
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
        ))
}
