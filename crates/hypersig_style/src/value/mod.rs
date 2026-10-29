//! The value types a property method accepts, and functions that build them.
//!
//! A property whose value is one of the hand-written types alone takes it
//! directly, as `color` takes a [`Color`]. A property with keywords of its own
//! takes an enum generated for it, such as [`Width`], which converts from each
//! hand-written type it accepts.

use std::borrow::Cow;
use std::fmt;

// Written by tests/generate.rs, whose output is kept as it writes it.
#[rustfmt::skip]
mod generated;

// One enum per property with keywords of its own, a few hundred in all, each
// reached as `value::Width` like the hand-written types here.
pub use generated::*;

/// A number with a unit, or a `var()` that stands for one.
/// `value`, for a declaration that writes it as a CSS number.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite, which CSS cannot write.
pub(crate) fn number(value: f64) -> f64 {
    assert!(
        value.is_finite(),
        "a CSS number is finite, and this one is {value}"
    );
    value
}

#[derive(Debug, Clone, PartialEq)]
enum Amount {
    Unit(f64, &'static str),
    /// A `var()` or a math function, written as it stands.
    Css(String),
    /// The expression inside a `calc()`, written without it.
    Calc(String),
}

impl Amount {
    /// `value` in `unit`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is NaN or infinite, which CSS cannot write.
    fn unit(value: impl Into<f64>, unit: &'static str) -> Self {
        Self::Unit(number(value.into()), unit)
    }

    /// Writes the amount, leaving the unit off zero where `zero_bare` is set,
    /// as a length may be written but a duration may not.
    fn write(&self, f: &mut fmt::Formatter<'_>, zero_bare: bool) -> fmt::Result {
        match self {
            Self::Unit(n, unit) => {
                write!(f, "{n}")?;
                if !(zero_bare && *n == 0.0) {
                    f.write_str(unit)?;
                }
                Ok(())
            }
            Self::Css(css) => f.write_str(css),
            Self::Calc(expression) => write!(f, "calc({expression})"),
        }
    }

    /// The amount as an argument of a math function.
    ///
    /// The unit stays on zero, because a bare `0` there is a number and not
    /// a length. An expression is put in parentheses where `grouped` is set,
    /// so that `a - (b + c)` keeps its meaning.
    fn term(&self, grouped: bool) -> String {
        match self {
            Self::Unit(n, unit) => format!("{n}{unit}"),
            Self::Css(css) => css.clone(),
            Self::Calc(expression) if grouped => format!("({expression})"),
            Self::Calc(expression) => expression.clone(),
        }
    }
}

/// A length such as `16px` or `1.5rem`, or a [`Token`](crate::Token) that holds one.
#[derive(Debug, Clone, PartialEq)]
pub struct Length(Amount);

impl Length {
    /// A length of zero, written `0`.
    pub const ZERO: Self = Self(Amount::Unit(0.0, "px"));

    pub(crate) fn var(css: String) -> Self {
        Self(Amount::Css(css))
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write(f, true)
    }
}

macro_rules! lengths {
    ($($name:ident => $unit:literal: $doc:literal),* $(,)?) => {$(
        #[doc = concat!("A length in `", $unit, "`, ", $doc, ".")]
        ///
        /// # Panics
        ///
        /// Panics if `value` is NaN or infinite.
        #[must_use]
        pub fn $name(value: impl Into<f64>) -> Length {
            Length(Amount::unit(value, $unit))
        }
    )*};
}

lengths! {
    px => "px": "CSS pixels",
    rem => "rem": "the root element's font size",
    em => "em": "the element's font size",
    ch => "ch": "the width of `0` in the element's font",
    ex => "ex": "the x-height of the element's font",
    lh => "lh": "the element's line height",
    vw => "vw": "1% of the viewport's width",
    vh => "vh": "1% of the viewport's height",
    vmin => "vmin": "1% of the viewport's smaller side",
    vmax => "vmax": "1% of the viewport's larger side",
    svh => "svh": "1% of the small viewport's height, with browser toolbars shown",
    lvh => "lvh": "1% of the large viewport's height, with browser toolbars hidden",
    dvh => "dvh": "1% of the viewport's current height",
    cqw => "cqw": "1% of the query container's width",
    cqh => "cqh": "1% of the query container's height",
    cm => "cm": "centimeters",
    mm => "mm": "millimeters",
    pt => "pt": "points",
}

/// A percentage such as `50%`, or a [`Token`](crate::Token) that holds one.
#[derive(Debug, Clone, PartialEq)]
pub struct Percentage(Amount);

impl Percentage {
    pub(crate) fn var(css: String) -> Self {
        Self(Amount::Css(css))
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write(f, false)
    }
}

/// The percentage `value%`.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite.
#[must_use]
pub fn pct(value: impl Into<f64>) -> Percentage {
    Percentage(Amount::unit(value, "%"))
}

/// A duration such as `200ms`, or a [`Token`](crate::Token) that holds one.
#[derive(Debug, Clone, PartialEq)]
pub struct Time(Amount);

impl Time {
    pub(crate) fn var(css: String) -> Self {
        Self(Amount::Css(css))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write(f, false)
    }
}

/// A duration in milliseconds.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite.
#[must_use]
pub fn ms(value: impl Into<f64>) -> Time {
    Time(Amount::unit(value, "ms"))
}

/// A duration in seconds.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite.
#[must_use]
pub fn secs(value: impl Into<f64>) -> Time {
    Time(Amount::unit(value, "s"))
}

/// An angle such as `90deg`, or a [`Token`](crate::Token) that holds one.
#[derive(Debug, Clone, PartialEq)]
pub struct Angle(Amount);

impl Angle {
    pub(crate) fn var(css: String) -> Self {
        Self(Amount::Css(css))
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write(f, false)
    }
}

/// An angle in degrees.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite.
#[must_use]
pub fn deg(value: impl Into<f64>) -> Angle {
    Angle(Amount::unit(value, "deg"))
}

/// An angle in whole turns.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite.
#[must_use]
pub fn turn(value: impl Into<f64>) -> Angle {
    Angle(Amount::unit(value, "turn"))
}

/// A value type that `calc()`, `min()`, `max()`, and `clamp()` combine.
///
/// It is implemented for [`Length`], [`Percentage`], [`Time`], and [`Angle`].
/// Each math function takes values of one type and returns that type, so a
/// length and a percentage cannot be mixed, as `calc(100% - 2rem)` would;
/// such a value goes through [`Raw`].
///
/// ```
/// use hypersig_style::value::{clamp, max, pct, px, rem, vw};
///
/// assert_eq!(clamp(rem(1.5), vw(4), rem(3)).to_string(), "clamp(1.5rem, 4vw, 3rem)");
/// assert_eq!((px(100) - rem(2) * 2.0).to_string(), "calc(100px - (2rem * 2))");
/// assert_eq!(max(pct(50), pct(20) + pct(10)).to_string(), "max(50%, 20% + 10%)");
/// ```
pub trait Quantity: fmt::Display + quantity::Sealed {}

mod quantity {
    pub trait Sealed: Sized {
        /// The value as an argument of a math function.
        fn into_term(self) -> String;
        /// The value written as `css`.
        fn from_css(css: String) -> Self;
    }
}

macro_rules! quantities {
    ($($ty:ident),*) => {$(
        impl quantity::Sealed for $ty {
            fn into_term(self) -> String {
                self.0.term(false)
            }

            fn from_css(css: String) -> Self {
                Self(Amount::Css(css))
            }
        }

        impl Quantity for $ty {}

        /// The sum, written `calc(a + b)`.
        impl std::ops::Add for $ty {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(Amount::Calc(format!("{} + {}", self.0.term(false), other.0.term(true))))
            }
        }

        /// The difference, written `calc(a - b)`.
        impl std::ops::Sub for $ty {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(Amount::Calc(format!("{} - {}", self.0.term(false), other.0.term(true))))
            }
        }

        /// The value times a number, written `calc(a * n)`.
        ///
        /// # Panics
        ///
        /// Panics if `factor` is NaN or infinite.
        impl std::ops::Mul<f64> for $ty {
            type Output = Self;

            fn mul(self, factor: f64) -> Self {
                let factor = number(factor);
                Self(Amount::Calc(format!("{} * {factor}", self.0.term(true))))
            }
        }

        /// The value divided by a number, written `calc(a / n)`.
        ///
        /// # Panics
        ///
        /// Panics if `divisor` is zero, NaN, or infinite.
        impl std::ops::Div<f64> for $ty {
            type Output = Self;

            fn div(self, divisor: f64) -> Self {
                let divisor = number(divisor);
                assert!(divisor != 0.0, "a CSS calc() divides by a number other than zero");
                Self(Amount::Calc(format!("{} / {divisor}", self.0.term(true))))
            }
        }
    )*};
}

quantities!(Length, Percentage, Time, Angle);

/// The CSS math function `name` over `terms`. A sum among them is written
/// without its `calc()`, which a math function reads as one argument.
fn math<T: Quantity, const N: usize>(name: &str, terms: [T; N]) -> T {
    let terms: Vec<String> = terms.into_iter().map(quantity::Sealed::into_term).collect();
    T::from_css(format!("{name}({})", terms.join(", ")))
}

/// `preferred`, kept between `min` and `max`, written `clamp()`.
#[must_use]
pub fn clamp<T: Quantity>(min: T, preferred: T, max: T) -> T {
    math("clamp", [min, preferred, max])
}

/// The smaller of `a` and `b`, written `min()`.
#[must_use]
pub fn min<T: Quantity>(a: T, b: T) -> T {
    math("min", [a, b])
}

/// The larger of `a` and `b`, written `max()`.
#[must_use]
pub fn max<T: Quantity>(a: T, b: T) -> T {
    math("max", [a, b])
}

/// A length from `raw`, for a form the typed lengths leave out.
///
/// ```
/// use hypersig_style::Style;
/// use hypersig_style::value::raw;
///
/// let style = Style::new().scroll_margin_top(raw("calc(1rem + env(safe-area-inset-top))"));
/// # let _ = style;
/// ```
impl From<Raw> for Length {
    fn from(raw: Raw) -> Self {
        Self(Amount::Css(raw.0))
    }
}

/// A color.
///
/// It is a named color such as [`Color::CRIMSON`], a hex or RGB value, a mix of
/// two colors, or a [`Token`](crate::Token) that holds one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color(Cow<'static, str>);

impl Color {
    /// The element's `color`, written `currentColor`.
    pub const CURRENT_COLOR: Self = Self::named("currentColor");

    /// Fully transparent black, written `transparent`. MDN lists it apart
    /// from the named colors, so the generator does not write it.
    pub const TRANSPARENT: Self = Self::named("transparent");

    pub(crate) const fn named(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }

    pub(crate) fn var(css: String) -> Self {
        Self(Cow::Owned(css))
    }

    /// The color `#rrggbb` from `rgb`, as in `Color::hex(0xb83f45)`.
    ///
    /// # Panics
    ///
    /// Panics if `rgb` is above `0xffffff`, such as an `#rrggbbaa` literal,
    /// which [`Color::hex_alpha`] takes. An eight-digit literal whose red
    /// byte is `00`, such as `0x00ff_0080`, is the six-digit `0xff0080` to
    /// the compiler, so it passes and is read as `#ff0080`.
    #[must_use]
    pub fn hex(rgb: u32) -> Self {
        assert!(
            rgb <= 0x00ff_ffff,
            "Color::hex takes six hex digits, not {rgb:#x}; use Color::hex_alpha for #rrggbbaa"
        );
        Self(Cow::Owned(format!("#{rgb:06x}")))
    }

    /// The color `#rrggbbaa` from `rgba`, as in `Color::hex_alpha(0xb83f4580)`.
    #[must_use]
    pub fn hex_alpha(rgba: u32) -> Self {
        Self(Cow::Owned(format!("#{rgba:08x}")))
    }

    /// The color with red, green, and blue channels from 0 to 255.
    #[must_use]
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(Cow::Owned(format!("rgb({red} {green} {blue})")))
    }

    /// The color from a hue and percentages of saturation and lightness.
    ///
    /// ```
    /// use hypersig_style::value::{Color, deg, pct};
    ///
    /// assert_eq!(Color::hsl(deg(210), pct(50), pct(40)).to_string(), "hsl(210deg 50% 40%)");
    /// ```
    #[must_use]
    pub fn hsl(hue: Angle, saturation: Percentage, lightness: Percentage) -> Self {
        Self::function("hsl", [hue.0, saturation.0, lightness.0])
    }

    /// The color in OKLCH from lightness, chroma, and hue.
    ///
    /// `chroma` is about 0 to 0.4, where 0 is gray.
    ///
    /// # Panics
    ///
    /// Panics if `chroma` is NaN or infinite.
    ///
    /// ```
    /// use hypersig_style::value::{Color, deg, pct};
    ///
    /// assert_eq!(Color::oklch(pct(70), 0.1, deg(210)).to_string(), "oklch(70% 0.1 210deg)");
    /// ```
    #[must_use]
    pub fn oklch(lightness: Percentage, chroma: f64, hue: Angle) -> Self {
        let chroma = Amount::unit(chroma, "");
        Self::function("oklch", [lightness.0, chroma, hue.0])
    }

    /// The color in Oklab from lightness and the `a` and `b` axes.
    ///
    /// `a` runs from green to red and `b` from blue to yellow, each about
    /// -0.4 to 0.4.
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` is NaN or infinite.
    #[must_use]
    pub fn oklab(lightness: Percentage, a: f64, b: f64) -> Self {
        Self::function(
            "oklab",
            [lightness.0, Amount::unit(a, ""), Amount::unit(b, "")],
        )
    }

    fn function(name: &str, channels: [Amount; 3]) -> Self {
        // A channel is not an argument of a math function, so a sum keeps
        // its `calc()` there.
        let [x, y, z] = channels.map(|c| match c {
            Amount::Calc(expression) => format!("calc({expression})"),
            c => c.term(false),
        });
        Self(Cow::Owned(format!("{name}({x} {y} {z})")))
    }

    /// This color at `opacity` percent, mixed with `transparent` in sRGB.
    #[must_use]
    pub fn opacity(&self, opacity: u8) -> Self {
        let opacity = opacity.min(100);
        Self(Cow::Owned(format!(
            "color-mix(in srgb, {self} {opacity}%, transparent)"
        )))
    }

    /// `weight` percent of this color mixed with the rest of `other`, in sRGB.
    #[must_use]
    pub fn mix(&self, other: &Self, weight: u8) -> Self {
        let weight = weight.min(100);
        Self(Cow::Owned(format!(
            "color-mix(in srgb, {self} {weight}%, {other})"
        )))
    }
}

/// A color from `raw`, for a form the constructors leave out.
///
/// ```
/// use hypersig_style::Style;
/// use hypersig_style::value::raw;
///
/// let style = Style::new().color(raw("color(display-p3 1 0.5 0)"));
/// # let _ = style;
/// ```
impl From<Raw> for Color {
    fn from(raw: Raw) -> Self {
        Self(Cow::Owned(raw.0))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Writes `text` as a double-quoted CSS string.
///
/// A control character becomes a hex escape such as `\a `, because CSS reads
/// a carriage return or a form feed as a newline, and a newline ends the
/// string so that the text after it is parsed as rules. `<` becomes `\3c `,
/// so the text cannot close the `<style>` element the stylesheet is written
/// into.
fn quote(text: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("\"")?;
    for c in text.chars() {
        match c {
            '"' => f.write_str("\\\"")?,
            '\\' => f.write_str("\\\\")?,
            '<' => f.write_str("\\3c ")?,
            c if c.is_control() => write!(f, "\\{:x} ", u32::from(c))?,
            c => write!(f, "{c}")?,
        }
    }
    f.write_str("\"")
}

/// A CSS string, as `content` takes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quoted(String);

impl fmt::Display for Quoted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        quote(&self.0, f)
    }
}

/// The CSS string holding `text`, escaped as needed.
#[must_use]
pub fn quoted(text: impl Into<String>) -> Quoted {
    Quoted(text.into())
}

/// A URL, written `url("...")`.
///
/// A path that starts with a single `/` names a file the application
/// serves. hypersig's page and its patches write it under the `Base` the
/// page's routes are declared `under`, as they write an `Asset`, so under
/// `Base::new("/app")` the URL `/fonts/inter.woff2` loads
/// `/app/fonts/inter.woff2`. A URL relative to the page, such as
/// `fonts/inter.woff2`, follows the page's own path, and one on another host
/// is written as given.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Url(&'static str);

impl Url {
    /// The URL as written in the program.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("url(")?;
        quote(self.0, f)?;
        f.write_str(")")
    }
}

/// The URL `href`, of a file the page loads.
///
/// `href` is a path the application serves, a URL relative to the page, or
/// an `http` or `https` URL on another host. It is text written in the
/// program, because each value names its own class, so a URL built at run
/// time would add a class per value.
///
/// # Panics
///
/// Panics if `href` is empty, has a scheme other than `http` or `https`, or
/// holds a space or a control character. In a `const` the panic is a compile
/// error.
#[must_use]
pub const fn url(href: &'static str) -> Url {
    assert!(
        is_loaded_url(href.as_bytes()),
        "a URL a page loads is a path or an `http` or `https` URL, with no space or \
         control character"
    );
    Url(href)
}

/// Whether `url` is a non-empty URL with no space or control character,
/// whose scheme, if it has one, is `http` or `https`.
const fn is_loaded_url(url: &[u8]) -> bool {
    if url.is_empty() {
        return false;
    }
    // The browser drops spaces and control characters around and inside the
    // scheme before reading it, so `java\tscript:` runs as `javascript:`.
    let mut i = 0;
    while i < url.len() {
        if url[i] <= b' ' || url[i] == 0x7f {
            return false;
        }
        i += 1;
    }
    let mut end = 0;
    while end < url.len() && scheme_byte(url[end], end == 0) {
        end += 1;
    }
    // Without a scheme before the first `:`, the URL is relative to the page.
    if end == 0 || end == url.len() || url[end] != b':' {
        return true;
    }
    same_ascii(url, end, b"http") || same_ascii(url, end, b"https")
}

/// Whether `byte` may sit in a URL scheme, whose first character is a letter.
const fn scheme_byte(byte: u8, first: bool) -> bool {
    byte.is_ascii_alphabetic()
        || (!first && (byte.is_ascii_digit() || matches!(byte, b'+' | b'-' | b'.')))
}

/// Whether the first `len` bytes of `url` are `scheme`, ignoring case.
const fn same_ascii(url: &[u8], len: usize, scheme: &[u8]) -> bool {
    if len != scheme.len() {
        return false;
    }
    let mut i = 0;
    while i < len {
        if url[i].to_ascii_lowercase() != scheme[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// A value written into the stylesheet as given.
///
/// It is for the forms the typed values leave out, such as `margin: 0 auto`,
/// a `calc()` that mixes a length and a percentage, or a `calc()` of numbers.
/// Every generated value enum, [`Color`], and [`Length`] convert from it.
///
/// Nothing checks that the browser accepts it. A value it rejects drops the
/// declaration, and the element renders as if the property were not set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw(String);

impl fmt::Display for Raw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The value `css`, unchecked.
///
/// # Panics
///
/// Panics if `css` could end more than its own declaration: where it contains
/// `;`, `{`, `}`, `<`, a backslash, `/*`, or a control character, or leaves a
/// quote or a parenthesis open. The first three would end the declaration or
/// the rule, `<` could close the `<style>` element, and an escape, a comment,
/// or an open quote or parenthesis would carry the value on into the rules
/// after it.
#[must_use]
pub fn raw(css: impl Into<String>) -> Raw {
    let css = css.into();
    assert!(
        stays_in_declaration(&css),
        "a raw CSS value holds no `;`, `{{`, `}}`, `<`, backslash, `/*`, or control \
         character, and closes every quote and parenthesis it opens",
    );
    Raw(css)
}

/// Whether `css` ends where it ends, so the declaration after it is read as
/// its own.
fn stays_in_declaration(css: &str) -> bool {
    let mut depth = 0_usize;
    let mut quote = None;
    for c in css.chars() {
        match (quote, c) {
            (_, ';' | '{' | '}' | '<' | '\\') => return false,
            (_, c) if c.is_control() => return false,
            (Some(open), c) if c == open => quote = None,
            (None, '"' | '\'') => quote = Some(c),
            (None, '(') => depth += 1,
            (None, ')') => match depth.checked_sub(1) {
                Some(less) => depth = less,
                None => return false,
            },
            _ => {}
        }
    }
    quote.is_none() && depth == 0 && !css.contains("/*")
}

/// A keyword every property accepts, which CSS calls a CSS-wide keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CssWideKeyword {
    /// `inherit`: the parent's computed value.
    Inherit,
    /// `initial`: the property's initial value.
    Initial,
    /// `unset`: `inherit` for an inherited property, `initial` otherwise.
    Unset,
    /// `revert`: the value from the browser's or the user's stylesheet.
    Revert,
    /// `revert-layer`: the value from the previous cascade layer.
    RevertLayer,
}

impl fmt::Display for CssWideKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Inherit => "inherit",
            Self::Initial => "initial",
            Self::Unset => "unset",
            Self::Revert => "revert",
            Self::RevertLayer => "revert-layer",
        })
    }
}

/// The two colors `scrollbar-color` takes, the thumb's and then the track's.
///
/// ```
/// use hypersig_style::value::{Color, ScrollbarColor};
///
/// let value = ScrollbarColor::from((Color::GRAY, Color::TRANSPARENT));
/// assert_eq!(value.to_string(), "gray transparent");
/// ```
impl From<(Color, Color)> for ScrollbarColor {
    fn from((thumb, track): (Color, Color)) -> Self {
        Self::Raw(Raw(format!("{thumb} {track}")))
    }
}

impl FontFamily {
    /// This family, then `next` where this one lacks a character.
    ///
    /// The browser takes each character from the first family in the list
    /// that has it, so a list ends in a generic family such as `sans-serif`,
    /// which every browser has.
    ///
    /// ```
    /// use hypersig_style::FontFace;
    /// use hypersig_style::value::{FontFamily, url};
    ///
    /// let inter = FontFace::new("Inter", url("/inter.woff2"));
    /// let body = inter.family().fallback(FontFamily::SansSerif);
    /// assert_eq!(body.to_string(), "\"Inter\", sans-serif");
    /// ```
    #[must_use]
    pub fn fallback(self, next: impl Into<Self>) -> Self {
        Self::Raw(Raw(format!("{self}, {}", next.into())))
    }
}

#[cfg(test)]
mod tests {
    use super::{is_loaded_url, px, quoted, raw, stays_in_declaration, url};

    #[test]
    fn a_string_escapes_every_character_that_would_end_it() {
        assert_eq!(quoted("a\"b\\c<d").to_string(), r#""a\"b\\c\3c d""#);
        // CSS reads `\r`, `\x0c`, and `\r\n` as a newline, which ends the
        // string and lets the rest be read as rules.
        assert_eq!(
            quoted("x\r);}a{b:c}\x0c\n").to_string(),
            r#""x\d );}a{b:c}\c \a ""#,
        );
        assert_eq!(
            url("x\");}a{b:c}<").to_string(),
            r#"url("x\");}a{b:c}\3c ")"#
        );
    }

    #[test]
    fn a_path_a_relative_url_and_another_host_are_urls_a_page_loads() {
        for href in [
            "/inter.woff2",
            "./inter.woff2",
            "inter.woff2",
            "//cdn.example/inter.woff2",
            "https://cdn.example/inter.woff2",
            "HTTP://cdn.example/inter.woff2",
            "inter.woff2?v=1:2",
        ] {
            assert_eq!(url(href).as_str(), href);
        }
    }

    #[test]
    fn a_url_with_a_script_scheme_or_a_space_is_not_one_a_page_loads() {
        for href in [
            "",
            "javascript:alert(1)",
            "java\tscript:alert(1)",
            "mailto:a@example.com",
            "data:font/woff2;base64,AAAA",
            "/a b.png",
        ] {
            assert!(!is_loaded_url(href.as_bytes()), "{href:?}");
        }
    }

    #[test]
    #[should_panic(expected = "a URL a page loads is a path")]
    fn a_url_with_a_script_scheme_panics_where_it_is_built() {
        let _ = url("javascript:alert(1)");
    }

    #[test]
    fn a_raw_value_keeps_quotes_and_parentheses_it_closes() {
        assert_eq!(
            raw("'Helvetica Neue', calc(100% - (2rem))").to_string(),
            "'Helvetica Neue', calc(100% - (2rem))",
        );
        for open in [
            "'Inter", "url(x", "a)", "a /* b", "\\66 oo", "a\nb", "'a;b'",
        ] {
            assert!(!stays_in_declaration(open), "{open:?}");
        }
    }

    #[test]
    #[should_panic(expected = "a CSS number is finite")]
    fn a_length_of_nan_panics_where_it_is_built() {
        let _ = px(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "a CSS number is finite")]
    fn an_opacity_of_nan_panics_where_it_is_set() {
        let _ = crate::Style::new().opacity(super::Opacity::Number(f64::NAN));
    }

    #[test]
    fn math_keeps_units_on_zero_and_groups_an_inner_sum() {
        use super::{Length, clamp, min, ms, rem, vw};
        assert_eq!((Length::ZERO + px(4)).to_string(), "calc(0px + 4px)");
        assert_eq!(Length::ZERO.to_string(), "0");
        assert_eq!(
            (rem(1) - (px(2) + vw(1))).to_string(),
            "calc(1rem - (2px + 1vw))"
        );
        assert_eq!(
            clamp(rem(1), vw(2) + rem(0.5), rem(2)).to_string(),
            "clamp(1rem, 2vw + 0.5rem, 2rem)"
        );
        assert_eq!((ms(300) / 2.0).to_string(), "calc(300ms / 2)");
        assert_eq!(
            (min(px(10), px(20)) * 2.0).to_string(),
            "calc(min(10px, 20px) * 2)"
        );
        let css = crate::Global::new()
            .rule(
                crate::Selector::Root,
                crate::Style::new().font_size(clamp(rem(1.5), vw(4), rem(3))),
            )
            .to_css();
        assert!(css.contains("font-size:clamp(1.5rem, 4vw, 3rem)"), "{css}");
    }

    #[test]
    #[should_panic(expected = "divides by a number other than zero")]
    fn a_division_by_zero_panics_where_it_is_built() {
        let _ = px(4) / 0.0;
    }

    #[test]
    fn a_color_function_writes_its_channels() {
        use super::{Color, deg, pct};
        assert_eq!(
            Color::hsl(deg(0), pct(0), pct(100)).to_string(),
            "hsl(0deg 0% 100%)"
        );
        assert_eq!(
            Color::oklab(pct(60), -0.1, 0.05).to_string(),
            "oklab(60% -0.1 0.05)"
        );
        assert_eq!(
            Color::hsl(deg(10) + deg(5), pct(50), pct(40)).to_string(),
            "hsl(calc(10deg + 5deg) 50% 40%)"
        );
    }

    #[test]
    #[should_panic(expected = "a CSS number is finite")]
    fn a_chroma_of_nan_panics_where_it_is_built() {
        let _ = super::Color::oklch(super::pct(50), f64::NAN, super::deg(0));
    }

    #[test]
    fn a_property_without_a_typed_form_for_a_value_takes_it_raw() {
        let css = crate::Global::new()
            .rule(
                crate::Selector::Root,
                crate::Style::new()
                    .font_size(raw("clamp(1.5rem, 4vw, 3rem)"))
                    .opacity(raw("calc(var(--x) / 2)"))
                    .z_index(raw("calc(var(--layer) + 1)"))
                    .padding_top(raw("calc(100% - 2rem)"))
                    .color(raw("light-dark(black, white)")),
            )
            .to_css();
        assert!(css.contains("font-size:clamp(1.5rem, 4vw, 3rem)"), "{css}");
        assert!(css.contains("z-index:calc(var(--layer) + 1)"), "{css}");
        assert!(css.contains("padding-top:calc(100% - 2rem)"), "{css}");
        assert!(css.contains("color:light-dark(black, white)"), "{css}");
    }

    #[test]
    #[should_panic(expected = "a CSS number is finite")]
    fn an_infinite_flex_grow_panics_where_it_is_set() {
        let _ = crate::Style::new().flex_grow(f64::INFINITY);
    }

    #[test]
    fn a_font_family_lists_its_fallbacks_in_order() {
        use super::FontFamily;
        let inter = crate::FontFace::new("Inter", url("/inter.woff2"));
        let list = inter
            .family()
            .fallback(FontFamily::SystemUi)
            .fallback(FontFamily::SansSerif);
        assert_eq!(list.to_string(), r#""Inter", system-ui, sans-serif"#);
        assert_eq!(
            FontFamily::from(quoted("JetBrains Mono"))
                .fallback(FontFamily::Monospace)
                .to_string(),
            r#""JetBrains Mono", monospace"#
        );
    }

    #[test]
    #[should_panic(expected = "Color::hex takes six hex digits")]
    fn an_eight_digit_hex_panics_where_it_is_built() {
        let _ = super::Color::hex(0x2563_eb80);
    }

    #[test]
    fn hex_alpha_keeps_all_four_bytes() {
        assert_eq!(super::Color::hex(0x00_2563).to_string(), "#002563");
        assert_eq!(
            super::Color::hex_alpha(0x0025_6380).to_string(),
            "#00256380"
        );
    }

    #[test]
    fn a_color_scheme_is_light_dark_or_both() {
        use super::ColorScheme;
        assert_eq!(ColorScheme::Light.to_string(), "light");
        assert_eq!(ColorScheme::Dark.to_string(), "dark");
        assert_eq!(ColorScheme::LightDark.to_string(), "light dark");
    }
}
