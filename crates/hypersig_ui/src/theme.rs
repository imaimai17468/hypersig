//! The page-wide rules and the dark theme.

use hypersig::style::value::{FontFamily, LineHeight, raw};
use hypersig::style::{Element, Global, Media, Selector, Style, Theme};

use crate::token;

/// The rules every page with these components needs: a reset, the font,
/// and the page colors.
///
/// `color-scheme` follows the theme, so a scrollbar or a native picker is
/// drawn dark beside the dark tokens.
///
/// ```
/// use hypersig::{Page, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// let html = Page::new("App", App {})
///     .global(hypersig_ui::global())
///     .theme(hypersig_ui::theme())
///     .render();
/// assert!(html.contains("--background:#09090b"));
/// ```
#[must_use]
pub fn global() -> Global {
    Global::preflight()
        .rule(Selector::Root, Style::new().color_scheme(raw("light dark")))
        .rule(
            Element::Body,
            Style::new()
                .font_family(raw(
                    "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\"",
                ))
                .line_height(LineHeight::Number(1.5))
                .background_color(token::background().var())
                .color(token::foreground().var()),
        )
        .rule(Element::Code, Style::new().font_family(FontFamily::Monospace))
}

/// The dark values of the tokens for a visitor whose system prefers dark.
///
/// Without it, every visitor sees the light values, which are the tokens'
/// defaults.
#[must_use]
pub fn theme() -> Theme {
    let dark = token::dark()
        .into_iter()
        .fold(Theme::new(), |theme, (token, value)| {
            theme.set(&token, value)
        });
    Theme::new().on(Media::Dark, dark)
}

/// The dark values of the tokens for one element and those inside it.
///
/// Use it with [`El::css_when`](hypersig::html::El::css_when) to let a
/// signal pick the theme rather than the system setting.
///
/// ```
/// use hypersig::html::div;
/// use hypersig::Signals;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {
///     dark: bool,
/// }
///
/// let app = div::<App>().css_when(hypersig_ui::dark(), App::dark());
/// # let _ = app;
/// ```
#[must_use]
pub fn dark() -> Style {
    token::dark().into_iter().fold(
        Style::new().color_scheme(raw("dark")),
        |style, (token, value)| style.token(&token, value),
    )
}

/// The light values of the tokens for one element and those inside it.
///
/// Use it where a signal picks the theme and the system prefers dark, since
/// [`theme`] sets the dark values on the whole page.
#[must_use]
pub fn light() -> Style {
    token::light().into_iter().fold(
        Style::new().color_scheme(raw("light")),
        |style, (token, value)| style.token(&token, value),
    )
}
