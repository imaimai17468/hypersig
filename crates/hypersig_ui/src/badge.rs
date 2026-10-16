//! Badges: short labels such as a status or a count.

use hypersig::html::{El, span};
use hypersig::style::Style;
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Display, FontWeight, LineHeight, WhiteSpace, px, rem,
};

use crate::token;

/// The look of a badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BadgeVariant {
    /// Filled with [`token::primary`].
    Default,
    /// Filled with [`token::secondary`].
    Secondary,
    /// Filled with [`token::destructive`].
    Destructive,
    /// A border with no fill.
    Outline,
}

/// A `<span>` that shows a short label, such as a status, as a badge.
///
/// A badge is text and takes no role, so a screen reader reads it where it
/// stands. Where the color alone carries the meaning, put the meaning in the
/// text too, such as "Failed" rather than a red dot.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{BadgeVariant, badge};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// let status = badge::<App>(BadgeVariant::Secondary).text("Draft");
/// assert!(status.render().starts_with("<span"));
/// ```
#[must_use]
pub fn badge<S>(variant: BadgeVariant) -> El<S> {
    let (background, text, border) = match variant {
        BadgeVariant::Default => (
            token::primary().var(),
            token::primary_foreground().var(),
            Color::TRANSPARENT,
        ),
        BadgeVariant::Secondary => (
            token::secondary().var(),
            token::secondary_foreground().var(),
            Color::TRANSPARENT,
        ),
        BadgeVariant::Destructive => (
            token::destructive().var(),
            token::destructive_foreground().var(),
            Color::TRANSPARENT,
        ),
        BadgeVariant::Outline => (
            Color::TRANSPARENT,
            token::foreground().var(),
            token::border().var(),
        ),
    };
    span().css(
        Style::new()
            .display(Display::InlineFlex)
            .align_items(AlignItems::Center)
            .gap(rem(0.25))
            .padding_block(rem(0.125))
            .padding_inline(rem(0.5))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
            .border_color(border)
            .border_radius(px(6))
            .background_color(background)
            .color(text)
            .font_size(rem(0.75))
            .line_height(LineHeight::Length(rem(1)))
            .font_weight(FontWeight::Number(500.0))
            .white_space(WhiteSpace::Nowrap),
    )
}
