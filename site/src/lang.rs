//! The languages of the site, and texts that carry a translation into each.
//!
//! Every page is written once in Rust and rendered once per [`Lang`]. A text
//! is an [`L`], a struct with one field per language, so a text without its
//! Japanese translation fails to compile.

use hypersig::{OpenGraphLocale, Segment};
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

    /// The locale link previews read the language from, the `og:locale` tag.
    #[must_use]
    pub fn og_locale(self) -> OpenGraphLocale {
        match self {
            Self::En => OpenGraphLocale::new("en_US"),
            Self::Ja => OpenGraphLocale::new("ja_JP"),
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

/// A text of a page, with its translations.
///
/// `T` is `str` for one text, `[&str]` for a list, and `[&[&str]]` for the
/// rows of a table. There is no way to make one from English alone, so a
/// text without its translation fails to compile.
#[derive(Debug)]
pub enum Text<'a, T: ?Sized> {
    Translated(L<&'a T>),
    /// A value already in the page's language, such as the rows of a table
    /// whose cells were each picked with `Site::t`.
    Chosen(&'a T),
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
            Self::Chosen(text) => text,
        }
    }

    /// The English text, which a heading's id is made from in every language.
    ///
    /// # Panics
    ///
    /// Panics for a [`Text::Chosen`] value, which holds one language only.
    #[must_use]
    pub fn english(self) -> &'a T {
        match self {
            Self::Translated(text) => text.en,
            Self::Chosen(_) => panic!("a heading is written with its translations"),
        }
    }
}

impl<'a, T: ?Sized> From<L<&'a T>> for Text<'a, T> {
    fn from(text: L<&'a T>) -> Self {
        Self::Translated(text)
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
