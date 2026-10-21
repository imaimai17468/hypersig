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
#[derive(Debug, Clone, PartialEq)]
enum Amount {
    Unit(f64, &'static str),
    Var(String),
}

impl Amount {
    /// Writes the amount, leaving the unit off zero where `zero_bare` is set,
    /// as a length may be written but a duration may not.
    ///
    /// # Panics
    ///
    /// Panics if the number is NaN or infinite, which CSS cannot write.
    fn write(&self, f: &mut fmt::Formatter<'_>, zero_bare: bool) -> fmt::Result {
        match self {
            Self::Unit(n, unit) => {
                assert!(n.is_finite(), "a CSS number is finite");
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
        #[must_use]
        pub fn $name(value: impl Into<f64>) -> Length {
            Length(Amount::Unit(value.into(), $unit))
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
#[must_use]
pub fn pct(value: impl Into<f64>) -> Percentage {
    Percentage(Amount::Unit(value.into(), "%"))
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
#[must_use]
pub fn ms(value: impl Into<f64>) -> Time {
    Time(Amount::Unit(value.into(), "ms"))
}

/// A duration in seconds.
#[must_use]
pub fn secs(value: impl Into<f64>) -> Time {
    Time(Amount::Unit(value.into(), "s"))
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
#[must_use]
pub fn deg(value: impl Into<f64>) -> Angle {
    Angle(Amount::Unit(value.into(), "deg"))
}

/// An angle in whole turns.
#[must_use]
pub fn turn(value: impl Into<f64>) -> Angle {
    Angle(Amount::Unit(value.into(), "turn"))
}

/// A color: a named color such as [`Color::CRIMSON`], a hex or RGB value, a
/// mix of two colors, or a [`Token`](crate::Token) that holds one.
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url(String);

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("url(")?;
        quote(&self.0, f)?;
        f.write_str(")")
    }
}

/// The URL `href`.
#[must_use]
pub fn url(href: impl Into<String>) -> Url {
    Url(href.into())
}

/// A value written into the stylesheet as given, for the forms a property's
/// generated type leaves out, such as `margin: 0 auto`.
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

#[cfg(test)]
mod tests {
    use super::{quoted, raw, stays_in_declaration, url};

    #[test]
    fn a_string_escapes_every_character_that_would_end_it() {
        assert_eq!(quoted("a\"b\\c<d").to_string(), r#""a\"b\\c\3c d""#);
        // CSS reads `\r`, `\x0c`, and `\r\n` as a newline, which ends the
        // string and lets the rest be read as rules.
        assert_eq!(
            url("x\r);}a{b:c}\x0c\n").to_string(),
            r#"url("x\d );}a{b:c}\c \a ")"#,
        );
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
}
