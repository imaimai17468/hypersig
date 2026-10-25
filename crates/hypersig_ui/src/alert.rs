//! Alerts: a message that stands out from the content around it.

use hypersig::html::{El, attr, div};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::{BorderStyle, FontWeight, px, rem};

use crate::look;
use crate::token;

/// The look and the urgency of an alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AlertVariant {
    /// Information, read out when the screen reader is idle (`role="status"`).
    Default,
    /// An error, read out at once (`role="alert"`).
    Destructive,
}

/// A bordered box for a message, holding an [`alert_title`] and an
/// [`alert_description`].
///
/// Both roles are live regions. A screen reader reads a `role="alert"` box
/// when a patch puts it on the page, as for a server error. A
/// `role="status"` box is read when its text changes, and some screen
/// readers skip one that arrives with its text already in it, so keep a
/// status box on the page and change its text with `bind_text`.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{AlertVariant, alert, alert_description, alert_title};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// let failed = alert::<App>(AlertVariant::Destructive)
///     .child(alert_title().text("Payment failed"))
///     .child(alert_description().text("The card was declined."));
/// assert!(failed.render().contains(r#"role="alert""#));
/// ```
#[must_use]
pub fn alert<S>(variant: AlertVariant) -> El<S> {
    let (role, color) = match variant {
        AlertVariant::Default => ("status", token::card_foreground().var()),
        AlertVariant::Destructive => ("alert", token::destructive().var()),
    };
    div().attr(attr::ROLE, role).css(
        stack(rem(0.125))
            .padding_block(rem(0.75))
            .padding_inline(rem(1))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
            .border_color(token::border().var())
            .border_radius(token::radius().var())
            .background_color(token::card().var())
            .color(color)
            .merge(look::small_text()),
    )
}

/// The first line of an [`alert`].
#[must_use]
pub fn alert_title<S>() -> El<S> {
    div().css(Style::new().font_weight(FontWeight::Number(500.0)))
}

/// The text under the title of an [`alert`].
#[must_use]
pub fn alert_description<S>() -> El<S> {
    div().css(Style::new().color(token::muted_foreground().var()))
}
