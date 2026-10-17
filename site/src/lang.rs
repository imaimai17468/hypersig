//! The languages of the site, and texts that carry a translation into each.
//!
//! Every page is written once in Rust and rendered once per [`Lang`]. A text
//! is an [`L`], a struct with one field per language, so a text without its
//! Japanese translation fails to compile.

use hypersig::Segment;
use hypersig_i18n::Locale;
use serde::{Deserialize, Serialize};

/// A language of the site. English is the default and sits at the root.
#[derive(Segment, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    En,
    Ja,
}

/// One value per language of the site.
#[derive(Debug, Clone, Copy)]
pub struct L<T> {
    pub en: T,
    pub ja: T,
}

impl Locale for Lang {
    const ALL: &'static [Self] = &[Self::En, Self::Ja];
    type Text<T> = L<T>;

    fn pick<T>(self, text: L<T>) -> T {
        match self {
            Self::En => text.en,
            Self::Ja => text.ja,
        }
    }
}

impl Lang {
    /// The directory of the language's pages inside the site, ending in `/`,
    /// or `""` for English at the root.
    #[must_use]
    pub fn dir(self) -> &'static str {
        match self {
            Self::En => "",
            Self::Ja => "ja/",
        }
    }

    /// The language's name in itself, for the switcher in the header.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ja => "日本語",
        }
    }
}

/// A text of a page: its translations, or its English alone until the page is translated.
///
/// `T` is `str` for one text, `[&str]` for a list, and `[&[&str]]` for the
/// rows of a table.
#[derive(Debug)]
pub enum Text<'a, T: ?Sized> {
    Translated(L<&'a T>),
    English(&'a T),
}

impl<T: ?Sized> Clone for Text<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Text<'_, T> {}

impl<'a, T: ?Sized> Text<'a, T> {
    /// The text in `lang`.
    #[must_use]
    pub fn get(self, lang: Lang) -> &'a T {
        match self {
            Self::Translated(text) => lang.pick(text),
            Self::English(text) => text,
        }
    }

    /// The English text, which a heading's id is made from in every language.
    #[must_use]
    pub fn english(self) -> &'a T {
        self.get(Lang::En)
    }
}

impl<'a, T: ?Sized> From<L<&'a T>> for Text<'a, T> {
    fn from(text: L<&'a T>) -> Self {
        Self::Translated(text)
    }
}

impl<'a, T: ?Sized> From<&'a T> for Text<'a, T> {
    fn from(text: &'a T) -> Self {
        Self::English(text)
    }
}

impl<'a, T, const N: usize> From<&'a [T; N]> for Text<'a, [T]> {
    fn from(items: &'a [T; N]) -> Self {
        Self::English(items)
    }
}

impl<'a, T, const N: usize> From<L<&'a [T; N]>> for Text<'a, [T]> {
    fn from(items: L<&'a [T; N]>) -> Self {
        Self::Translated(L {
            en: items.en,
            ja: items.ja,
        })
    }
}
