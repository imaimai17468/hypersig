//! What a page tells crawlers and link previews: the origin absolute URLs
//! start with, and the Open Graph card.

use std::fmt;

/// The scheme, host, and optional path prefix a site is served from.
///
/// `Origin::new("https://example.com")` holds the scheme and host, and
/// [`Origin::under`] adds the path the site is served under.
///
/// A canonical URL and the URLs of a link preview must be absolute, and a
/// route's [`Url`](crate::Url) is a path, so [`Page::canonical`](crate::Page::canonical)
/// and [`OpenGraph::image`] join one to an `Origin`. An application behind a
/// proxy that serves it under `/blog` and strips that prefix sees its routes
/// at `/`, so its `Url`s lack the prefix, and [`Origin::under`] adds it back.
///
/// ```
/// use hypersig::Origin;
///
/// const SITE: Origin = Origin::new("https://example.com");
/// assert_eq!(SITE.as_str(), "https://example.com");
///
/// const BLOG: Origin = SITE.under("/blog");
/// assert_eq!(BLOG.to_string(), "https://example.com/blog");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Origin {
    origin: &'static str,
    prefix: &'static str,
}

impl Origin {
    /// The origin `origin`: `http://` or `https://`, a host, and an optional port.
    ///
    /// # Panics
    ///
    /// Panics if `origin` has another scheme, no host, or anything after
    /// the host and port, a trailing `/` included. In a `const` the panic is
    /// a compile error.
    #[must_use]
    pub const fn new(origin: &'static str) -> Self {
        if let Err(reason) = check_origin(origin.as_bytes()) {
            panic!("{}", reason);
        }
        Self { origin, prefix: "" }
    }

    /// The same origin with the site served under `prefix`, such as `/blog`.
    ///
    /// # Panics
    ///
    /// Panics if `prefix` does not start with `/`, ends with `/`, holds
    /// `//`, or holds a query, a fragment, a quote, or a space. In a `const`
    /// the panic is a compile error.
    #[must_use]
    pub const fn under(self, prefix: &'static str) -> Self {
        if let Err(reason) = check_prefix(prefix.as_bytes()) {
            panic!("{}", reason);
        }
        Self {
            origin: self.origin,
            prefix,
        }
    }

    /// Reads an origin and an optional prefix from configuration, such as
    /// `https://example.com/blog`.
    ///
    /// It checks what [`Origin::new`] and [`Origin::under`] check, and
    /// returns an error in place of their panic. One trailing `/` after the
    /// host is allowed and means no prefix. The text must live for the rest of
    /// the program, so a value read at startup is leaked once with
    /// [`String::leak`].
    ///
    /// ```
    /// use hypersig::Origin;
    ///
    /// let site = Origin::parse("https://example.com/blog")?;
    /// assert_eq!((site.as_str(), site.prefix()), ("https://example.com", "/blog"));
    /// assert!(Origin::parse("example.com").is_err());
    /// # Ok::<(), hypersig::OriginError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`OriginError`] when `text` is not an origin followed by an
    /// optional prefix.
    pub fn parse(text: &'static str) -> Result<Self, OriginError> {
        let error = |reason| OriginError { text, reason };
        let host = scheme_len(text.as_bytes()).ok_or_else(|| error(NO_SCHEME))?;
        let (origin, prefix) = match text[host..].find('/') {
            Some(at) => text.split_at(host + at),
            None => (text, ""),
        };
        check_origin(origin.as_bytes()).map_err(error)?;
        let prefix = if prefix == "/" { "" } else { prefix };
        if !prefix.is_empty() {
            check_prefix(prefix.as_bytes()).map_err(error)?;
        }
        Ok(Self { origin, prefix })
    }

    /// The scheme, host, and port, without the prefix.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        self.origin
    }

    /// The path the site is served under, or `""` at the root of the host.
    #[must_use]
    pub fn prefix(self) -> &'static str {
        self.prefix
    }

    /// `path` on this origin, under its prefix. `path` starts with `/`.
    pub(crate) fn join(self, path: &str) -> String {
        assert!(path.starts_with('/'), "a path on an origin starts with `/`");
        format!("{}{}{path}", self.origin, self.prefix)
    }
}

/// Writes the origin and its prefix, the text every absolute URL on the site
/// starts with.
impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.origin)?;
        f.write_str(self.prefix)
    }
}

const NO_SCHEME: &str = "an origin starts with `http://` or `https://`";

/// The length of the scheme and `://` that `bytes` starts with.
const fn scheme_len(bytes: &[u8]) -> Option<usize> {
    if starts_with(bytes, b"https://") {
        Some(8)
    } else if starts_with(bytes, b"http://") {
        Some(7)
    } else {
        None
    }
}

/// Checks that `bytes` is a scheme, a host, and an optional port.
const fn check_origin(bytes: &[u8]) -> Result<(), &'static str> {
    let Some(host) = scheme_len(bytes) else {
        return Err(NO_SCHEME);
    };
    if bytes.len() <= host {
        return Err("an origin has a host");
    }
    let mut i = host;
    while i < bytes.len() {
        if matches!(bytes[i], b'/' | b'?' | b'#' | b'"' | b' ') {
            return Err("an origin ends at the host and port");
        }
        // `Page::framed_by` writes the origin into a header unescaped, where
        // a `;` or a `,` would start another directive or policy.
        if !(bytes[i].is_ascii_alphanumeric()
            || matches!(bytes[i], b'.' | b'-' | b':' | b'[' | b']'))
        {
            return Err(
                "an origin's host and port hold letters, digits, `.`, `-`, `:`, `[`, and `]`",
            );
        }
        i += 1;
    }
    Ok(())
}

/// Checks that `bytes` is a path prefix such as `/blog`.
const fn check_prefix(bytes: &[u8]) -> Result<(), &'static str> {
    if bytes.len() <= 1 || bytes[0] != b'/' || bytes[bytes.len() - 1] == b'/' {
        return Err("a prefix starts with `/` and does not end with one");
    }
    let mut i = 1;
    while i < bytes.len() {
        let empty_segment = bytes[i] == b'/' && bytes[i - 1] == b'/';
        if empty_segment || matches!(bytes[i], b'?' | b'#' | b'"' | b' ') {
            return Err("a prefix is a path without a query, a fragment, or an empty segment");
        }
        i += 1;
    }
    Ok(())
}

/// Text that [`Origin::parse`] refused, with the rule it broke.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginError {
    text: &'static str,
    reason: &'static str,
}

impl fmt::Display for OriginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not an origin: {}", self.text, self.reason)
    }
}

impl std::error::Error for OriginError {}

const fn starts_with(bytes: &[u8], prefix: &[u8]) -> bool {
    if bytes.len() < prefix.len() {
        return false;
    }
    let mut i = 0;
    while i < prefix.len() {
        if bytes[i] != prefix[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// The card a chat app or a social site shows for a link to the page.
///
/// It is written as Open Graph tags, which X also reads, plus the
/// `twitter:card` tag that picks the card's size. The card takes its title
/// from the page's title, its text from [`Page::description`](crate::Page::description),
/// and its URL from [`Page::canonical`](crate::Page::canonical).
///
/// ```
/// use hypersig::{OpenGraph, Origin, Page, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Post {}
///
/// const SITE: Origin = Origin::new("https://example.com");
///
/// let html = Page::new("Hello", Post {})
///     .open_graph(OpenGraph::new().site_name("Blog").image(SITE, "/og/hello.png", "A wave"))
///     .render();
/// assert!(html.contains(r#"<meta property="og:image" content="https://example.com/og/hello.png">"#));
/// assert!(html.contains(r#"<meta name="twitter:card" content="summary_large_image">"#));
/// ```
#[derive(Debug, Clone, Default)]
pub struct OpenGraph {
    kind: OpenGraphType,
    site_name: Option<String>,
    image: Option<(String, String)>,
    locale: Option<OpenGraphLocale>,
    alternate_locales: Vec<OpenGraphLocale>,
}

/// What a page shows, the `og:type` of its [`OpenGraph`] card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum OpenGraphType {
    /// A site or a page of it that none of the other types fits.
    #[default]
    Website,
    /// A post, a news story, or another dated text.
    Article,
    /// A book.
    Book,
    /// A person's page.
    Profile,
    /// A product for sale, the type Facebook's catalogs read.
    Product,
}

impl OpenGraphType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Website => "website",
            Self::Article => "article",
            Self::Book => "book",
            Self::Profile => "profile",
            Self::Product => "product",
        }
    }
}

/// A language and territory in the `ja_JP` form that `og:locale` takes.
///
/// Open Graph writes a locale as a language, `_`, and a territory, so a BCP 47
/// tag such as `ja` or `en-US` does not fit it.
///
/// ```
/// use hypersig::OpenGraphLocale;
///
/// const JA: OpenGraphLocale = OpenGraphLocale::new("ja_JP");
/// assert_eq!(JA.as_str(), "ja_JP");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenGraphLocale(&'static str);

impl OpenGraphLocale {
    /// The locale `locale`: two or three lowercase letters, `_`, and two uppercase letters.
    ///
    /// # Panics
    ///
    /// Panics if `locale` has another shape, such as `ja` or `en-US`. In a
    /// `const` the panic is a compile error.
    #[must_use]
    pub const fn new(locale: &'static str) -> Self {
        let bytes = locale.as_bytes();
        let mut ok = matches!(bytes.len(), 5 | 6) && bytes[bytes.len() - 3] == b'_';
        let language = bytes.len().saturating_sub(3);
        let mut i = 0;
        while ok && i < bytes.len() {
            ok = if i < language {
                bytes[i].is_ascii_lowercase()
            } else if i > language {
                bytes[i].is_ascii_uppercase()
            } else {
                true
            };
            i += 1;
        }
        assert!(
            ok,
            "an Open Graph locale is a language, `_`, and a territory, such as `ja_JP`"
        );
        Self(locale)
    }

    /// The locale as `og:locale` writes it.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        self.0
    }
}

impl OpenGraph {
    /// A card with the page's title and description and no image.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets what the page shows. A card without it is a [`OpenGraphType::Website`].
    #[must_use]
    pub fn kind(mut self, kind: OpenGraphType) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the locale the page is written in, the `og:locale` tag.
    #[must_use]
    pub fn locale(mut self, locale: OpenGraphLocale) -> Self {
        self.locale = Some(locale);
        self
    }

    /// Adds a locale the page is also served in, one `og:locale:alternate` tag per call.
    ///
    /// ```
    /// use hypersig::{OpenGraph, OpenGraphLocale, OpenGraphType, Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Post {}
    ///
    /// let html = Page::new("Hello", Post {})
    ///     .open_graph(
    ///         OpenGraph::new()
    ///             .kind(OpenGraphType::Article)
    ///             .locale(OpenGraphLocale::new("en_US"))
    ///             .alternate_locale(OpenGraphLocale::new("ja_JP")),
    ///     )
    ///     .render();
    /// assert!(html.contains(r#"<meta property="og:type" content="article">"#));
    /// assert!(html.contains(r#"<meta property="og:locale" content="en_US">"#));
    /// assert!(html.contains(r#"<meta property="og:locale:alternate" content="ja_JP">"#));
    /// ```
    #[must_use]
    pub fn alternate_locale(mut self, locale: OpenGraphLocale) -> Self {
        self.alternate_locales.push(locale);
        self
    }

    /// Sets the name of the site the page belongs to, shown above the title.
    #[must_use]
    pub fn site_name(mut self, name: impl Into<String>) -> Self {
        self.site_name = Some(name.into());
        self
    }

    /// Shows the image at `path` on `origin` as a large picture, described by `alt`.
    ///
    /// # Panics
    ///
    /// Panics if `path` does not start with `/`.
    #[must_use]
    pub fn image(
        mut self,
        origin: Origin,
        path: impl Into<String>,
        alt: impl Into<String>,
    ) -> Self {
        self.image = Some((origin.join(&path.into()), alt.into()));
        self
    }

    /// Writes the tags into `<head>`.
    pub(crate) fn write(
        &self,
        out: &mut String,
        title: &str,
        description: Option<&str>,
        url: Option<&str>,
    ) {
        meta(out, "property", "og:type", self.kind.as_str());
        meta(out, "property", "og:title", title);
        if let Some(description) = description {
            meta(out, "property", "og:description", description);
        }
        if let Some(url) = url {
            meta(out, "property", "og:url", url);
        }
        if let Some(name) = &self.site_name {
            meta(out, "property", "og:site_name", name);
        }
        if let Some(locale) = self.locale {
            meta(out, "property", "og:locale", locale.as_str());
        }
        for locale in &self.alternate_locales {
            meta(out, "property", "og:locale:alternate", locale.as_str());
        }
        let card = if let Some((src, alt)) = &self.image {
            meta(out, "property", "og:image", src);
            meta(out, "property", "og:image:alt", alt);
            "summary_large_image"
        } else {
            "summary"
        };
        meta(out, "name", "twitter:card", card);
    }
}

/// Writes `<meta {key}="{name}" content="{content}">`.
pub(crate) fn meta(out: &mut String, key: &str, name: &str, content: &str) {
    out.push_str("<meta");
    crate::html::write_attr(out, key, name);
    crate::html::write_attr(out, "content", content);
    out.push('>');
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::{OpenGraph, OpenGraphLocale, Origin};
    use crate::{Page, Signals};

    #[derive(Signals, Serialize, Deserialize)]
    struct Note {
        text: String,
    }

    fn note() -> Note {
        Note {
            text: String::new(),
        }
    }

    const SITE: Origin = Origin::new("http://localhost:3000");

    #[test]
    fn a_page_without_head_metadata_declares_an_empty_icon_and_nothing_else() {
        let html = Page::new("Note", note()).render();
        assert!(html.contains(r#"<link rel="icon" href="data:,">"#));
        assert!(!html.contains("description"));
        assert!(!html.contains("og:"));
    }

    #[test]
    fn the_card_reads_the_title_and_description_of_the_page() {
        let html = Page::new("Tom & Jerry", note())
            .description("A \"cat\"")
            .icon("/icon.svg")
            .open_graph(OpenGraph::new())
            .render();
        assert!(html.contains(r#"<link rel="icon" href="/icon.svg">"#));
        assert!(html.contains(r#"<meta property="og:title" content="Tom &amp; Jerry">"#));
        assert!(html.contains(r#"<meta property="og:description" content="A &quot;cat&quot;">"#));
        assert!(html.contains(r#"<meta name="twitter:card" content="summary">"#));
        assert!(!html.contains("og:url"));
        assert!(!html.contains("og:image"));
    }

    #[test]
    fn an_origin_takes_a_port() {
        assert_eq!(SITE.join("/a"), "http://localhost:3000/a");
    }

    #[test]
    fn a_prefix_goes_between_the_origin_and_the_path() {
        const BLOG: Origin = SITE.under("/blog/en");
        assert_eq!(
            BLOG.join("/posts/3"),
            "http://localhost:3000/blog/en/posts/3"
        );
        assert_eq!(BLOG.prefix(), "/blog/en");
        assert_eq!(BLOG.as_str(), "http://localhost:3000");
    }

    #[test]
    fn the_card_image_takes_the_prefix() {
        const BLOG: Origin = SITE.under("/blog");
        let html = Page::new("Note", note())
            .open_graph(OpenGraph::new().image(BLOG, "/og.png", "A note"))
            .render();
        assert!(html.contains(r#"content="http://localhost:3000/blog/og.png""#));
    }

    #[test]
    #[should_panic(expected = "a prefix starts with `/`")]
    fn a_prefix_with_a_trailing_slash_panics() {
        let _ = SITE.under("/blog/");
    }

    #[test]
    #[should_panic(expected = "without a query, a fragment, or an empty segment")]
    fn a_prefix_with_an_empty_segment_panics() {
        let _ = SITE.under("/a//b");
    }

    #[test]
    #[should_panic(expected = "starts with `http://` or `https://`")]
    fn an_origin_without_a_scheme_panics() {
        let _ = Origin::new("example.com");
    }

    #[test]
    fn an_origin_holds_only_what_a_host_and_port_can() {
        for text in [
            "https://a.example;script-src",
            "https://a.example,https://b.example",
            "https://a.example'",
            "https://a.example\n",
        ] {
            assert!(Origin::parse(text).is_err(), "{text}");
        }
        for text in [
            "http://localhost:3000",
            "https://[::1]:8080",
            "https://a-b.example",
        ] {
            assert!(Origin::parse(text).is_ok(), "{text}");
        }
    }

    #[test]
    fn a_card_is_a_website_until_given_another_type() {
        let html = Page::new("Note", note())
            .open_graph(OpenGraph::new())
            .render();
        assert!(html.contains(r#"<meta property="og:type" content="website">"#));
        assert!(!html.contains("og:locale"));
    }

    #[test]
    fn an_open_graph_locale_takes_a_three_letter_language() {
        assert_eq!(OpenGraphLocale::new("fil_PH").as_str(), "fil_PH");
    }

    #[test]
    #[should_panic(expected = "such as `ja_JP`")]
    fn a_bcp_47_tag_is_not_an_open_graph_locale() {
        let _ = OpenGraphLocale::new("en-US");
    }

    #[test]
    #[should_panic(expected = "such as `ja_JP`")]
    fn a_language_alone_is_not_an_open_graph_locale() {
        let _ = OpenGraphLocale::new("ja");
    }

    #[test]
    #[should_panic(expected = "a path on an origin starts with `/`")]
    fn an_image_path_without_a_slash_panics() {
        let _ = OpenGraph::new().image(SITE, "og.png", "");
    }
}
