//! Cards: a bordered surface with a header, content, and a footer.

use hypersig::html::{El, div, h2, h3, h4, p};
use hypersig::style::Style;
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{BorderStyle, FontWeight, LineHeight, px, raw, rem};

use crate::look;
use crate::token;

/// The level of a heading, which follows the headings around it.
///
/// A screen reader lists a page by its headings, so a card title one level
/// below the section it sits in keeps that list in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Level {
    /// `<h2>`.
    H2,
    /// `<h3>`.
    H3,
    /// `<h4>`.
    H4,
}

impl Level {
    pub(crate) fn heading<S>(self) -> El<S> {
        match self {
            Self::H2 => h2(),
            Self::H3 => h3(),
            Self::H4 => h4(),
        }
    }
}

/// A `<div>` with a border, a shadow, and [`token::card`] behind it.
///
/// Put a [`card_header`], a [`card_content`], and a [`card_footer`] in it.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{Level, card, card_content, card_description, card_header, card_title};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// let plan = card::<App>()
///     .child(
///         card_header()
///             .child(card_title(Level::H3).text("Pro"))
///             .child(card_description().text("For teams of up to ten.")),
///     )
///     .child(card_content().text("$20 a month"));
/// assert!(plan.render().contains("<h3"));
/// ```
#[must_use]
pub fn card<S>() -> El<S> {
    div().css(
        stack(rem(1.5))
            .padding_block(rem(1.5))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
            .border_color(token::border().var())
            .border_radius(raw(format!("calc({} + 4px)", token::radius().var())))
            .background_color(token::card().var())
            .color(token::card_foreground().var())
            .box_shadow(raw(
                "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
            )),
    )
}

/// The top of a [`card`], holding its title and description.
#[must_use]
pub fn card_header<S>() -> El<S> {
    div().css(stack(rem(0.375)).padding_inline(rem(1.5)))
}

/// The title of a [`card`], as a heading of `level`.
#[must_use]
pub fn card_title<S>(level: Level) -> El<S> {
    level.heading().css(
        Style::new()
            .font_size(rem(1))
            .line_height(LineHeight::Number(1.0))
            .font_weight(FontWeight::Number(600.0)),
    )
}

/// A line under the title of a [`card`], in [`token::muted_foreground`].
#[must_use]
pub fn card_description<S>() -> El<S> {
    p().css(look::small_text().color(token::muted_foreground().var()))
}

/// The body of a [`card`].
#[must_use]
pub fn card_content<S>() -> El<S> {
    div().css(Style::new().padding_inline(rem(1.5)))
}

/// The bottom of a [`card`], laying its children out in a row.
#[must_use]
pub fn card_footer<S>() -> El<S> {
    div().css(hstack(rem(0.5)).padding_inline(rem(1.5)))
}
