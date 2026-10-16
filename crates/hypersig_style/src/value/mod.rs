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

/// Writes `n` as a CSS number.
///
/// # Panics
///
/// Panics if `n` is NaN or infinite, which CSS cannot write.
fn number(n: f64, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    assert!(n.is_finite(), "a CSS number is finite");
    write!(f, "{n}")
}

/// A length such as `16px` or `1.5rem`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    value: f64,
    unit: &'static str,
}

impl Length {
    /// A length of zero, written `0`.
    pub const ZERO: Self = Self {
        value: 0.0,
        unit: "",
    };
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        number(self.value, f)?;
        if self.value != 0.0 {
            f.write_str(self.unit)?;
        }
        Ok(())
    }
}

macro_rules! lengths {
    ($($name:ident => $unit:literal: $doc:literal),* $(,)?) => {$(
        #[doc = concat!("A length in `", $unit, "`, ", $doc, ".")]
        #[must_use]
        pub fn $name(value: impl Into<f64>) -> Length {
            Length { value: value.into(), unit: $unit }
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

/// A percentage such as `50%`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(f64);

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        number(self.0, f)?;
        f.write_str("%")
    }
}

/// The percentage `value%`.
#[must_use]
pub fn pct(value: impl Into<f64>) -> Percentage {
    Percentage(value.into())
}

/// A duration such as `200ms`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    value: f64,
    unit: &'static str,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        number(self.value, f)?;
        f.write_str(self.unit)
    }
}

/// A duration in milliseconds.
#[must_use]
pub fn ms(value: impl Into<f64>) -> Time {
    Time {
        value: value.into(),
        unit: "ms",
    }
}

/// A duration in seconds.
#[must_use]
pub fn secs(value: impl Into<f64>) -> Time {
    Time {
        value: value.into(),
        unit: "s",
    }
}

/// An angle such as `90deg`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    value: f64,
    unit: &'static str,
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        number(self.value, f)?;
        f.write_str(self.unit)
    }
}

/// An angle in degrees.
#[must_use]
pub fn deg(value: impl Into<f64>) -> Angle {
    Angle {
        value: value.into(),
        unit: "deg",
    }
}

/// An angle in whole turns.
#[must_use]
pub fn turn(value: impl Into<f64>) -> Angle {
    Angle {
        value: value.into(),
        unit: "turn",
    }
}

/// A color: a named color such as [`Color::CRIMSON`], a hex or RGB value, or a
/// mix of two colors.
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
/// `<` becomes the escape `\3c `, so the text cannot close the `<style>`
/// element the stylesheet is written into.
fn quote(text: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("\"")?;
    for c in text.chars() {
        match c {
            '"' => f.write_str("\\\"")?,
            '\\' => f.write_str("\\\\")?,
            '\n' => f.write_str("\\a ")?,
            '<' => f.write_str("\\3c ")?,
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
/// Panics if `css` contains `;`, `{`, `}`, or `<`. The first three would end
/// the declaration or the rule, and `<` could close the `<style>` element.
#[must_use]
pub fn raw(css: impl Into<String>) -> Raw {
    let css = css.into();
    assert!(
        !css.contains([';', '{', '}', '<']),
        "a raw CSS value holds no `;`, `{{`, `}}`, or `<`"
    );
    Raw(css)
}

/// A keyword every property accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Global {
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

impl fmt::Display for Global {
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
