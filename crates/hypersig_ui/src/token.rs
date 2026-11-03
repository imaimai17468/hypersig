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
    ($($(#[$doc:meta])* $name:ident => $css:literal, $light:expr, $dark:expr;)*) => {
        $(
            $(#[$doc])*
            #[must_use]
            pub fn $name() -> Token<Color> {
                Token::new($css, $light)
            }
        )*

        /// Each color token with its value in the light theme.
        pub(crate) fn light() -> Vec<(Token<Color>, Color)> {
            vec![$(($name(), $light)),*]
        }

        /// Each color token with its value in the dark theme.
        pub(crate) fn dark() -> Vec<(Token<Color>, Color)> {
            vec![$(($name(), $dark)),*]
        }
    };
}

colors! {
    /// The page background.
    background => "background", hex(0x00ff_ffff), hex(0x0009_090b);
    /// Text on [`background`].
    foreground => "foreground", hex(0x0009_090b), hex(0x00fa_fafa);
    /// The surface of a card.
    card => "card", hex(0x00ff_ffff), hex(0x0018_181b);
    /// Text on [`card`].
    card_foreground => "card-foreground", hex(0x0009_090b), hex(0x00fa_fafa);
    /// The surface of a menu or another popover.
    popover => "popover", hex(0x00ff_ffff), hex(0x0018_181b);
    /// Text on [`popover`].
    popover_foreground => "popover-foreground", hex(0x0009_090b), hex(0x00fa_fafa);
    /// The fill of a default button, a checked box, and an on switch.
    primary => "primary", hex(0x0018_181b), hex(0x00e4_e4e7);
    /// Text and marks on [`primary`].
    primary_foreground => "primary-foreground", hex(0x00fa_fafa), hex(0x0018_181b);
    /// The fill of a secondary button.
    secondary => "secondary", hex(0x00f4_f4f5), hex(0x0027_272a);
    /// Text on [`secondary`].
    secondary_foreground => "secondary-foreground", hex(0x0018_181b), hex(0x00fa_fafa);
    /// A quiet fill, such as the track of a tab list.
    muted => "muted", hex(0x00f4_f4f5), hex(0x0027_272a);
    /// Secondary text, such as a description.
    ///
    /// It has 4.8:1 contrast or more on [`background`] and on [`muted`] in both
    /// themes.
    muted_foreground => "muted-foreground", hex(0x006b_6b74), hex(0x00a1_a1aa);
    /// The fill under the pointer in a menu or a ghost button.
    accent => "accent", hex(0x00f4_f4f5), hex(0x003f_3f46);
    /// Text on [`accent`].
    accent_foreground => "accent-foreground", hex(0x0018_181b), hex(0x00fa_fafa);
    /// The color of a destructive action and of an error message.
    destructive => "destructive", hex(0x00e7_000b), hex(0x00ff_6467);
    /// Text on [`destructive`].
    destructive_foreground => "destructive-foreground", hex(0x00ff_ffff), hex(0x0009_090b);
    /// The border of a card, a menu, and a separator, which mark no control.
    ///
    /// In the dark theme it is white at 12% opacity, which shows as
    /// `#27272a` on [`background`] and stays visible on [`muted`], the
    /// fill of a selected table row.
    border => "border", hex(0x00e4_e4e7), Color::hex_alpha(0xffff_ff1f);
    /// The border of a text field, a checkbox, a radio, and an off switch.
    ///
    /// It has 3.3:1 contrast or more on [`background`] and on [`card`] in both
    /// themes.
    input => "input", hex(0x008b_8b94), hex(0x0070_707a);
    /// The focus outline.
    ring => "ring", hex(0x0018_181b), hex(0x00d4_d4d8);
    /// The first series of a chart. The five chart colors keep 3:1 or more
    /// on [`background`], [`card`], and [`muted`] in both themes.
    chart_1 => "chart-1", hex(0x0015_5dfc), hex(0x0051_a2ff);
    /// The second series of a chart.
    chart_2 => "chart-2", hex(0x0000_9689), hex(0x0000_d5be);
    /// The third series of a chart.
    chart_3 => "chart-3", hex(0x00f5_4900), hex(0x00ff_8904);
    /// The fourth series of a chart.
    chart_4 => "chart-4", hex(0x0098_10fa), hex(0x00c2_7aff);
    /// The fifth series of a chart.
    chart_5 => "chart-5", hex(0x00bb_4d00), hex(0x00ff_d230);
}

/// `Color::hex`, short enough to keep each token on one line; a token whose
/// value is translucent calls `Color::hex_alpha` instead.
fn hex(rgb: u32) -> Color {
    Color::hex(rgb)
}

/// The corner radius of a card, a dialog, and a menu.
#[must_use]
pub fn radius() -> Token<Length> {
    Token::new("radius", rem(0.625))
}
