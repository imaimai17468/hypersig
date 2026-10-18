//! What a page tells crawlers and link previews: the origin absolute URLs
//! start with, and the Open Graph card.

use std::fmt;

/// The scheme and host a site is served from, such as `https://example.com`.
///
/// A canonical URL and the URLs of a link preview must be absolute, and a
/// route's [`Url`](crate::Url) is a path, so [`Page::canonical`](crate::Page::canonical)
/// and [`OpenGraph::image`] join one to an `Origin`.
///
/// ```
/// use hypersig::Origin;
///
/// const SITE: Origin = Origin::new("https://example.com");
/// assert_eq!(SITE.as_str(), "https://example.com");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Origin(&'static str);

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
        let bytes = origin.as_bytes();
        let host = if starts_with(bytes, b"https://") {
            8
        } else if starts_with(bytes, b"http://") {
            7
        } else {
            panic!("an origin starts with `http://` or `https://`")
        };
        assert!(bytes.len() > host, "an origin has a host");
        let mut i = host;
        while i < bytes.len() {
            assert!(
                !matches!(bytes[i], b'/' | b'?' | b'#' | b'"' | b' '),
                "an origin ends at the host and port"
            );
            i += 1;
        }
        Self(origin)
    }

    /// The origin as text.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        self.0
    }

    /// `path` on this origin. `path` starts with `/`.
    pub(crate) fn join(self, path: &str) -> String {
        assert!(path.starts_with('/'), "a path on an origin starts with `/`");
        format!("{}{path}", self.0)
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

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
    site_name: Option<String>,
    image: Option<(String, String)>,
}

impl OpenGraph {
    /// A card with the page's title and description and no image.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
        meta(out, "property", "og:type", "website");
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

    use super::{OpenGraph, Origin};
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
    #[should_panic(expected = "starts with `http://` or `https://`")]
    fn an_origin_without_a_scheme_panics() {
        let _ = Origin::new("example.com");
    }

    #[test]
    #[should_panic(expected = "a path on an origin starts with `/`")]
    fn an_image_path_without_a_slash_panics() {
        let _ = OpenGraph::new().image(SITE, "og.png", "");
    }
}
