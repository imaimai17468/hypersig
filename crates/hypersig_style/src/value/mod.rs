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
    Var(String),
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
            Self::Var(css) => f.write_str(css),
        }
    }
}

/// A length such as `16px` or `1.5rem`, or a [`Token`](crate::Token) that holds one.
#[derive(Debug, Clone, PartialEq)]
pub struct Length(Amount);

impl Length {
    /// A length of zero, written `0`.
    pub const ZERO: Self = Self(Amount::Unit(0.0, ""));

    pub(crate) fn var(css: String) -> Self {
        Self(Amount::Var(css))
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
        Self(Amount::Var(css))
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
        Self(Amount::Var(css))
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
        Self(Amount::Var(css))
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

    /// The color `#rrggbb` from the low 24 bits of `rgb`, as in `Color::hex(0xb83f45)`.
    #[must_use]
    pub fn hex(rgb: u32) -> Self {
        Self(Cow::Owned(format!("#{:06x}", rgb & 0x00ff_ffff)))
    }

    /// The color with red, green, and blue channels from 0 to 255.
    #[must_use]
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(Cow::Owned(format!("rgb({red} {green} {blue})")))
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
/// It is for the forms a property's generated type leaves out, such as `margin:
/// 0 auto`.
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
    #[should_panic(expected = "a CSS number is finite")]
    fn an_infinite_flex_grow_panics_where_it_is_set() {
        let _ = crate::Style::new().flex_grow(f64::INFINITY);
    }
}
