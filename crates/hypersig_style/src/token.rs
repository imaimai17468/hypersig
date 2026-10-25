//! Design tokens: CSS custom properties with a typed value, and the theme
//! that sets them for the page.

use std::fmt::{self, Write as _};

use crate::condition::{Media, open_block};
use crate::sheet::{ORDER, TOKEN_LAYER};
use crate::value::{Angle, Color, Length, Percentage, Time};

/// A value type a [`Token`] can hold.
///
/// It is implemented for the types whose value can be a `var()`: [`Color`],
/// [`Length`], [`Percentage`], [`Time`], and [`Angle`].
pub trait TokenValue: fmt::Display + sealed::Sealed {
    #[doc(hidden)]
    fn from_var(css: String) -> Self;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! token_values {
    ($($ty:ty),*) => {$(
        impl sealed::Sealed for $ty {}

        impl TokenValue for $ty {
            fn from_var(css: String) -> Self {
                <$ty>::var(css)
            }
        }
    )*};
}

token_values!(Color, Length, Percentage, Time, Angle);

/// A named value of type `T`, written as the custom property `--<name>`.
///
/// [`Token::var`] reads it as `var(--<name>, <default>)`, so a page without a
/// [`Theme`] shows the default. A [`Theme`] sets it for the whole page, and
/// [`Style::token`](crate::Style::token) for one element and those inside it.
///
/// Two tokens with the same name are one custom property to the browser, and
/// nothing checks that each name is used once.
///
/// ```
/// use hypersig_style::value::{Color, px};
/// use hypersig_style::{Media, Style, Theme, Token};
///
/// let brand = Token::new("brand", Color::hex(0xb83f45));
/// let button = Style::new().background_color(brand.var()).padding(px(8));
/// let theme = Theme::new().on(Media::Dark, Theme::new().set(&brand, Color::hex(0xff6b6b)));
/// assert!(theme.to_css().contains("@media (prefers-color-scheme: dark){:root{--brand:#ff6b6b}}"));
/// # let _ = button;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Token<T> {
    name: &'static str,
    default: T,
}

impl<T: TokenValue> Token<T> {
    /// A token named `name` whose value is `default` until something sets it.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty or holds a character other than an ASCII
    /// letter, digit, `-`, or `_`, which a custom property name would need
    /// escaped.
    #[must_use]
    pub fn new(name: &'static str, default: T) -> Self {
        assert!(
            !name.is_empty()
                && name
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
            "a token name is ASCII letters, digits, `-`, and `_`"
        );
        Self { name, default }
    }

    /// The name, without the leading `--`.
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The token's current value, as `var(--<name>, <default>)`.
    #[must_use]
    pub fn var(&self) -> T {
        T::from_var(format!("var(--{}, {})", self.name, self.default))
    }
}

/// Values for tokens across the whole page, written on `:root`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Theme {
    declarations: Vec<(Vec<Media>, &'static str, String)>,
}

impl Theme {
    /// A theme that sets nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `token` to `value`. A later value for the same token under the
    /// same conditions replaces an earlier one.
    #[must_use]
    pub fn set<T: TokenValue>(mut self, token: &Token<T>, value: impl Into<T>) -> Self {
        self.insert(Vec::new(), token.name, value.into().to_string());
        self
    }

    /// Adds the values of `theme`, applied only under `media`.
    #[must_use]
    pub fn on(mut self, media: Media, theme: Self) -> Self {
        for (mut conditions, name, value) in theme.declarations {
            if let Err(at) = conditions.binary_search(&media) {
                conditions.insert(at, media);
            }
            self.insert(conditions, name, value);
        }
        self
    }

    /// Adds the values of `theme`, which win over this theme's own.
    #[must_use]
    pub fn merge(mut self, theme: Self) -> Self {
        for (media, name, value) in theme.declarations {
            self.insert(media, name, value);
        }
        self
    }

    fn insert(&mut self, media: Vec<Media>, name: &'static str, value: String) {
        if let Some(old) = self
            .declarations
            .iter_mut()
            .find(|(m, n, _)| *m == media && *n == name)
        {
            old.2 = value;
        } else {
            self.declarations.push((media, name, value));
        }
    }

    /// Whether the theme sets nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty()
    }

    /// The theme as a stylesheet in [`TOKEN_LAYER`]: the unconditioned values
    /// first, then one block per set of media conditions, in the order
    /// [`Media`] lists them.
    #[must_use]
    pub fn to_css(&self) -> String {
        let mut groups: Vec<&Vec<Media>> = self.declarations.iter().map(|(m, _, _)| m).collect();
        groups.sort();
        groups.dedup();
        let mut out = format!("{ORDER}@layer {TOKEN_LAYER}{{");
        for media in groups {
            if !media.is_empty() {
                open_block(&mut out, "@media ", media.iter().map(|m| m.query()));
            }
            out.push_str(":root{");
            let mut first = true;
            for (_, name, value) in self.declarations.iter().filter(|(m, _, _)| m == media) {
                if !first {
                    out.push(';');
                }
                first = false;
                let _ = write!(out, "--{name}:{value}");
            }
            out.push('}');
            if !media.is_empty() {
                out.push('}');
            }
        }
        out.push('}');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{Theme, Token};
    use crate::value::{Color, px, rem};
    use crate::{Media, Sheet, Style};

    #[test]
    fn reads_a_token_with_its_default_as_fallback() {
        let space = Token::new("space-2", rem(0.5));
        let style = Style::new().padding(space.var());
        let mut sheet = Sheet::new();
        sheet.add(&style);
        assert!(sheet.to_css().contains("{padding:var(--space-2, 0.5rem)}"));
    }

    #[test]
    fn sets_a_token_on_an_element() {
        let brand = Token::new("brand", Color::RED);
        let style = Style::new().token(&brand, Color::BLUE);
        assert_eq!(style.class_names().collect::<Vec<_>>(), ["--brand_blue"]);
        let mut sheet = Sheet::new();
        sheet.add(&style);
        assert!(sheet.to_css().ends_with("{.--brand_blue{--brand:blue}}"));
    }

    #[test]
    fn writes_the_theme_in_the_token_layer() {
        let brand = Token::new("brand", Color::RED);
        let gap = Token::new("gap", px(4));
        let theme = Theme::new()
            .set(&brand, Color::BLUE)
            .on(Media::Dark, Theme::new().set(&brand, Color::WHITE))
            .set(&gap, px(8))
            .set(&brand, Color::NAVY);
        assert_eq!(
            theme.to_css(),
            "@layer hypersig-tokens,hypersig-base,hypersig-atoms,hypersig-toggled;\
             @layer hypersig-tokens{:root{--brand:navy;--gap:8px}\
             @media (prefers-color-scheme: dark){:root{--brand:white}}}"
        );
    }

    #[test]
    #[should_panic(expected = "a token name is ASCII letters")]
    fn rejects_a_name_that_needs_escaping() {
        let _ = Token::new("brand color", Color::RED);
    }
}
