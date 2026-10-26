//! Typed translations for hypersig, where a missing translation is a compile error.
//!
//! An application lists its locales in an enum and implements [`Locale`] for
//! it. The text of each locale sits in one value of [`Locale::Text`], a
//! struct with a field per locale, so a text without its translation into
//! one of the locales fails to compile, and a locale added to the enum fails
//! to compile until every text has it. The locale itself is a
//! [`Segment`](hypersig::Segment), so it is the first segment of a page's
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
// A library's caller owns standard output and standard error, so the library writes to neither.
#![warn(clippy::print_stdout, clippy::print_stderr)]
#![doc(
    html_logo_url = "https://imaimai17468.github.io/hypersig/icon.svg",
    html_favicon_url = "https://imaimai17468.github.io/hypersig/icon.svg"
)]

#[cfg(feature = "axum")]
mod axum;

#[cfg(feature = "axum")]
pub use axum::LocaleRouterExt;

use hypersig::{LanguageTag, Origin, Page, Signals, Url, Variants};
use serde::Serialize;

/// The locales of an application: an enum with `#[derive(Segment)]` whose segments are BCP 47 tags.
///
/// The compiler checks that each text has every locale, through
/// [`Locale::Text`], and that [`Locale::pick`] handles every locale, through
/// its `match`. The locales the application serves are the enum's variants,
/// which [`locales`] lists in declaration order, so the first variant is
/// the default that [`negotiate`] falls back to.
pub trait Locale: Variants + Serialize + Copy + Eq {
    /// One value per locale, usually a struct with a field per locale.
    type Text<T>;

    /// The value of `text` for this locale.
    fn pick<T>(self, text: Self::Text<T>) -> T;

    /// The BCP 47 tag of this locale, the string serde writes for it.
    ///
    /// [`Page::lang`] and [`Page::alternate`] take it.
    ///
    /// # Panics
    ///
    /// Panics if serde does not write the locale as a string, which an enum
    /// with `#[derive(Segment)]` always does, or if the string is not shaped
    /// as a BCP 47 tag, as `en_US` from `#[serde(rename = "en_US")]` is not.
    #[must_use]
    fn tag(self) -> LanguageTag {
        match serde_json::to_value(self) {
            Ok(serde_json::Value::String(tag)) => LanguageTag::parse(tag)
                .unwrap_or_else(|error| panic!("a locale's segment is its language tag: {error}")),
            other => panic!("a `Segment` enum serializes to a string, not {other:?}"),
        }
    }
}

/// Every locale of `L`, its variants in declaration order, the default first.
///
/// ```
/// # use hypersig::Segment;
/// # use hypersig_i18n::{Locale, locales};
/// # use serde::{Deserialize, Serialize};
/// # #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
/// # #[serde(rename_all = "lowercase")]
/// # enum Lang { En, Ja }
/// # struct ByLang<T> { en: T, ja: T }
/// # impl Locale for Lang {
/// #     type Text<T> = ByLang<T>;
/// #     fn pick<T>(self, t: ByLang<T>) -> T { match self { Self::En => t.en, Self::Ja => t.ja } }
/// # }
/// assert_eq!(locales::<Lang>(), [Lang::En, Lang::Ja]);
/// ```
#[must_use]
pub fn locales<L: Locale>() -> &'static [L] {
    L::VARIANTS
}

/// The locale of `L` that best matches an `Accept-Language` header value, or the default.
///
/// The ranges in `accept_language` are tried from the highest `q` to the
/// lowest, and in the order written where two are equal. A range matches a
/// locale whose tag is the same, ignoring case, or the same after the last
/// `-subtag` of the range is removed one at a time, so `ja-JP` matches `ja`.
/// A locale whose tag starts with the range and a `-` matches too, so `en`
/// matches `en-US`. `*`, a header with no match, and an empty header give
/// the first locale of [`locales`].
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
/// Panics if `L` has no variant.
#[must_use]
pub fn negotiate<L: Locale>(accept_language: &str) -> L {
    let default = *locales::<L>()
        .first()
        .expect("a locale enum has at least one variant");
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
    let tags: Vec<(L, LanguageTag)> = locales::<L>()
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
                .find(|(_, tag)| tag.as_str().eq_ignore_ascii_case(prefix))
            {
                return *locale;
            }
            match prefix.rfind('-') {
                Some(end) => prefix = &prefix[..end],
                None => break,
            }
        }
        if let Some((locale, _)) = tags.iter().find(|(_, tag)| {
            let tag = tag.as_str();
            tag.len() > range.len()
                && tag.as_bytes()[range.len()] == b'-'
                && tag[..range.len()].eq_ignore_ascii_case(range)
        }) {
            return *locale;
        }
    }
    default
}

/// Adds the alternate of each locale to a [`Page`].
pub trait LocalePage: Sized {
    /// Names the page's address in each locale of `L`, where `url` gives the address in one.
    ///
    /// It adds one [`Page::alternate`] per locale of [`locales`], tagged
    /// with [`Locale::tag`], and a [`Page::alternate_default`] at the first
    /// locale, the default that [`negotiate`] falls back to.
    ///
    /// ```
    /// # use hypersig::{Origin, Page, Route, Segment, Signals};
    /// # use hypersig_i18n::{Locale, LocalePage};
    /// # use serde::{Deserialize, Serialize};
    /// # #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
    /// # #[serde(rename_all = "lowercase")]
    /// # enum Lang { En, Ja }
    /// # struct ByLang<T> { en: T, ja: T }
    /// # impl Locale for Lang {
    /// #     type Text<T> = ByLang<T>;
    /// #     fn pick<T>(self, t: ByLang<T>) -> T { match self { Self::En => t.en, Self::Ja => t.ja } }
    /// # }
    /// # #[derive(Signals, Serialize, Deserialize)]
    /// # struct Post {}
    /// # async fn post((): (), _: (Lang, u32)) -> Option<Page<Post>> { None }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const POST: Route<(Lang, u32)> = "/{lang}/posts/{id}" => post;
    /// }
    ///
    /// const SITE: Origin = Origin::new("https://example.com");
    ///
    /// let html = Page::new("Post", Post {})
    ///     .alternates(SITE, |lang: Lang| POST.url((lang, 3)))
    ///     .render();
    /// assert!(html.contains(r#"<link rel="alternate" hreflang="en" href="https://example.com/en/posts/3">"#));
    /// assert!(html.contains(r#"<link rel="alternate" hreflang="ja" href="https://example.com/ja/posts/3">"#));
    /// assert!(html.contains(r#"<link rel="alternate" hreflang="x-default" href="https://example.com/en/posts/3">"#));
    /// # let _ = pages;
    /// ```
    #[must_use]
    fn alternates<L: Locale>(self, origin: Origin, url: impl Fn(L) -> Url) -> Self;
}

impl<S: Signals> LocalePage for Page<S> {
    fn alternates<L: Locale>(self, origin: Origin, url: impl Fn(L) -> Url) -> Self {
        let mut page = self;
        for &locale in locales::<L>() {
            page = page.alternate(origin, &url(locale), locale.tag());
        }
        match locales::<L>().first() {
            Some(&default) => page.alternate_default(origin, &url(default)),
            None => page,
        }
    }
}
