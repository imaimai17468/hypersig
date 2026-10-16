//! The design tokens the components read their colors and radius from.
//!
//! The names and the light values follow shadcn/ui's zinc theme. The values
//! differ where shadcn/ui falls short of WCAG 2.2 AA: [`input`] is dark
//! enough for the 3:1 contrast that a control's boundary needs (1.4.11),
//! [`muted_foreground`] keeps 4.5:1 on [`muted`] as well as on
//! [`background`], and [`ring`] is drawn as a solid outline rather than a
//! translucent shadow (2.4.7).
//!
//! Set a token for a whole page with a [`Theme`](hypersig::style::Theme), or
//! for one element with [`Style::token`](hypersig::style::Style::token).
//!
//! ```
//! use hypersig::style::Theme;
//! use hypersig::style::value::Color;
//! use hypersig_ui::token;
//!
//! let brand = Theme::new().set(&token::primary(), Color::hex(0x0025_63eb));
//! assert!(brand.to_css().contains("--primary:#2563eb"));
//! ```

use hypersig::style::Token;
use hypersig::style::value::{Color, Length, rem};

macro_rules! colors {
    ($($(#[$doc:meta])* $name:ident => $css:literal, $light:literal, $dark:literal;)*) => {
        $(
            $(#[$doc])*
            #[must_use]
            pub fn $name() -> Token<Color> {
                Token::new($css, Color::hex($light))
            }
        )*

        /// Each color token with its value in the light theme.
        pub(crate) fn light() -> Vec<(Token<Color>, Color)> {
            vec![$(($name(), Color::hex($light))),*]
        }

        /// Each color token with its value in the dark theme.
        pub(crate) fn dark() -> Vec<(Token<Color>, Color)> {
            vec![$(($name(), Color::hex($dark))),*]
        }
    };
}

colors! {
    /// The page background.
    background => "background", 0x00ff_ffff, 0x0009_090b;
    /// Text on [`background`].
    foreground => "foreground", 0x0009_090b, 0x00fa_fafa;
    /// The surface of a card.
    card => "card", 0x00ff_ffff, 0x0018_181b;
    /// Text on [`card`].
    card_foreground => "card-foreground", 0x0009_090b, 0x00fa_fafa;
    /// The surface of a menu or another popover.
    popover => "popover", 0x00ff_ffff, 0x0018_181b;
    /// Text on [`popover`].
    popover_foreground => "popover-foreground", 0x0009_090b, 0x00fa_fafa;
    /// The fill of a default button, a checked box, and an on switch.
    primary => "primary", 0x0018_181b, 0x00e4_e4e7;
    /// Text and marks on [`primary`].
    primary_foreground => "primary-foreground", 0x00fa_fafa, 0x0018_181b;
    /// The fill of a secondary button.
    secondary => "secondary", 0x00f4_f4f5, 0x0027_272a;
    /// Text on [`secondary`].
    secondary_foreground => "secondary-foreground", 0x0018_181b, 0x00fa_fafa;
    /// A quiet fill, such as the track of a tab list.
    muted => "muted", 0x00f4_f4f5, 0x0027_272a;
    /// Secondary text, such as a description, at 4.8:1 or more on
    /// [`background`] and on [`muted`] in both themes.
    muted_foreground => "muted-foreground", 0x006b_6b74, 0x00a1_a1aa;
    /// The fill under the pointer in a menu or a ghost button.
    accent => "accent", 0x00f4_f4f5, 0x003f_3f46;
    /// Text on [`accent`].
    accent_foreground => "accent-foreground", 0x0018_181b, 0x00fa_fafa;
    /// The color of a destructive action and of an error message.
    destructive => "destructive", 0x00e7_000b, 0x00ff_6467;
    /// Text on [`destructive`].
    destructive_foreground => "destructive-foreground", 0x00ff_ffff, 0x0009_090b;
    /// The border of a card, a menu, and a separator, which mark no control.
    border => "border", 0x00e4_e4e7, 0x0027_272a;
    /// The border of a text field, a checkbox, a radio, and an off switch,
    /// at 3.3:1 or more on [`background`] and on [`card`] in both themes.
    input => "input", 0x008b_8b94, 0x0070_707a;
    /// The focus outline.
    ring => "ring", 0x0018_181b, 0x00d4_d4d8;
}

/// The corner radius of a card, a dialog, and a menu.
#[must_use]
pub fn radius() -> Token<Length> {
    Token::new("radius", rem(0.625))
}
