//! Typed translations for hypersig, where a missing translation is a compile error.
//!
//! An application lists its locales in an enum and implements [`Locale`] for
//! it. The text of each locale sits in one value of [`Locale::Text`], a
//! struct with a field per locale, so a text without its translation into
//! one of the locales fails to compile, and a locale added to the enum fails
//! to compile until every text has it. The locale itself is a
//! [`Segment`], so it is the first segment of a page's
//! [`Route`](hypersig::Route), and a link to the same page in another
//! language is a link with another locale.
//!
//! ```
//! use hypersig::html::{a, p};
//! use hypersig::{Page, Route, Segment, Signals};
//! use hypersig_i18n::Locale;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
//! #[serde(rename_all = "lowercase")]
//! enum Lang {
//!     En,
//!     Ja,
//! }
//!
//! /// One value per language.
//! struct ByLang<T> {
//!     en: T,
//!     ja: T,
//! }
//!
//! impl Locale for Lang {
//!     const ALL: &'static [Self] = &[Self::En, Self::Ja];
//!     type Text<T> = ByLang<T>;
//!
//!     fn pick<T>(self, text: ByLang<T>) -> T {
//!         match self {
//!             Self::En => text.en,
//!             Self::Ja => text.ja,
//!         }
//!     }
//! }
//!
//! #[derive(Signals, Serialize, Deserialize)]
//! struct Home {}
//!
//! hypersig::routes! {
//!     fn pages(Router<()>);
//!     const HOME: Route<Lang> = "/{lang}" => home;
//! }
//!
//! async fn home((): (), lang: Lang) -> Page<Home> {
//!     let other = if lang == Lang::En { Lang::Ja } else { Lang::En };
//!     Page::new(lang.pick(ByLang { en: "Welcome", ja: "ようこそ" }), Home {})
//!         .lang(lang.tag())
//!         .child(p().text(lang.pick(ByLang { en: "Hello.", ja: "こんにちは。" })))
//!         .child(a().href(HOME.link_with(&other)).text(other.pick(ByLang { en: "English", ja: "日本語" })))
//! }
//!
//! assert_eq!(HOME.url(Lang::Ja).as_str(), "/ja");
//! # let _ = pages;
//! ```
//!
//! A message that takes values is a function per locale, as in
//! `ByLang<fn(u32) -> String>`, so its arguments are typed in every
//! translation alike.
//!
//! With the `axum` feature (on by default), [`LocaleRouterExt`] sends a
//! visitor who opens `/` to the page in the language their browser asks for.

#[cfg(feature = "axum")]
mod axum;

#[cfg(feature = "axum")]
pub use axum::LocaleRouterExt;

use hypersig::Segment;
use serde::Serialize;

/// The locales of an application: an enum with `#[derive(Segment)]` whose segments are BCP 47 tags.
///
/// The compiler checks that each text has every locale, through
/// [`Locale::Text`], and that [`Locale::pick`] handles every locale, through
/// its `match`. It does not check that [`Locale::ALL`] lists every variant;
/// a variant left out of it is never chosen by [`negotiate`].
pub trait Locale: Segment + Serialize + Copy + Eq + 'static {
    /// Every locale the application serves, the default first.
    const ALL: &'static [Self];

    /// One value per locale, usually a struct with a field per locale.
    type Text<T>;

    /// The value of `text` for this locale.
    fn pick<T>(self, text: Self::Text<T>) -> T;

    /// The BCP 47 tag of this locale, the string serde writes for it.
    ///
    /// # Panics
    ///
    /// Panics if serde does not write the locale as a string, which an enum
    /// with `#[derive(Segment)]` always does.
    #[must_use]
    fn tag(self) -> String {
        match serde_json::to_value(self) {
            Ok(serde_json::Value::String(tag)) => tag,
            _ => panic!("a `Segment` enum serializes to a string"),
        }
    }
}

/// The locale of `L` that best matches an `Accept-Language` header value, or the default.
///
/// The ranges in `accept_language` are tried from the highest `q` to the
/// lowest, and in the order written where two are equal. A range matches a
/// locale whose tag is the same, ignoring case, or the same after the last
/// `-subtag` of the range is removed one at a time, so `ja-JP` matches `ja`.
/// A locale whose tag starts with the range and a `-` matches too, so `en`
/// matches `en-US`. `*`, a header with no match, and an empty header give
/// the first locale of [`Locale::ALL`].
///
/// ```
/// # use hypersig::Segment;
/// # use hypersig_i18n::{Locale, negotiate};
/// # use serde::{Deserialize, Serialize};
/// # #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
/// # #[serde(rename_all = "lowercase")]
/// # enum Lang { En, Ja }
/// # struct ByLang<T> { en: T, ja: T }
/// # impl Locale for Lang {
/// #     const ALL: &'static [Self] = &[Self::En, Self::Ja];
/// #     type Text<T> = ByLang<T>;
/// #     fn pick<T>(self, t: ByLang<T>) -> T { match self { Self::En => t.en, Self::Ja => t.ja } }
/// # }
/// assert_eq!(negotiate::<Lang>("ja-JP,ja;q=0.9,en;q=0.8"), Lang::Ja);
/// assert_eq!(negotiate::<Lang>("fr, en;q=0.5"), Lang::En);
/// assert_eq!(negotiate::<Lang>("fr"), Lang::En);
/// ```
///
/// # Panics
///
/// Panics if [`Locale::ALL`] is empty.
#[must_use]
pub fn negotiate<L: Locale>(accept_language: &str) -> L {
    let default = *L::ALL
        .first()
        .expect("`Locale::ALL` lists at least one locale");
    let mut ranges: Vec<(&str, f32)> = accept_language
        .split(',')
        .filter_map(|item| {
            let mut parts = item.split(';');
            let range = parts.next()?.trim();
            let q = parts
                .find_map(|param| param.trim().strip_prefix("q="))
                .map_or(Some(1.0), |q| q.trim().parse::<f32>().ok())?;
            (!range.is_empty() && q > 0.0).then_some((range, q))
        })
        .collect();
    // A stable sort keeps the written order between ranges of the same weight.
    ranges.sort_by(|a, b| b.1.total_cmp(&a.1));
    let tags: Vec<(L, String)> = L::ALL
        .iter()
        .map(|&locale| (locale, locale.tag()))
        .collect();
    for (range, _) in ranges {
        if range == "*" {
            return default;
        }
        let mut prefix = range;
        loop {
            if let Some((locale, _)) = tags
                .iter()
                .find(|(_, tag)| tag.eq_ignore_ascii_case(prefix))
            {
                return *locale;
            }
            match prefix.rfind('-') {
                Some(end) => prefix = &prefix[..end],
                None => break,
            }
        }
        if let Some((locale, _)) = tags.iter().find(|(_, tag)| {
            tag.len() > range.len()
                && tag.as_bytes()[range.len()] == b'-'
                && tag[..range.len()].eq_ignore_ascii_case(range)
        }) {
            return *locale;
        }
    }
    default
}
