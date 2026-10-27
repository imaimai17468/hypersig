//! The files a page loads: its scripts, stylesheets, icon, and runtime in
//! `<head>`, and the files its CSS loads with `url()`.

use std::borrow::Cow;

use crate::route::Base;
use crate::style::value::{Url, quoted, url};

/// The URL of a file a page loads.
///
/// [`Page::script`](crate::Page::script),
/// [`Page::stylesheet`](crate::Page::stylesheet),
/// [`Page::icon`](crate::Page::icon), and
/// [`Page::runtime_src`](crate::Page::runtime_src) take one.
///
/// A path that starts with a single `/`, such as `/map.js`, names a file the
/// application serves, and the page writes it under the [`Base`] of the
/// route that serves the page, as it writes the runtime's path. So a page of
/// a `routes!` block declared `under APP` loads `/app/map.js`. Any other
/// URL is written as given: a URL relative to the page, such as
/// `./hypersig.js`, or one on another host, such as
/// `https://cdn.jsdelivr.net/npm/...`.
///
/// It is built only from text written in the program, because a script's
/// URL decides what code the page runs, so a URL a visitor typed cannot
/// become one.
///
/// ```
/// use hypersig::{Asset, Page, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Place {}
///
/// const MAP: Asset = Asset::new("/map.js");
///
/// let html = Page::new("Place", Place {}).script(MAP).render();
/// assert!(html.contains(r#"<script type="module" src="/map.js"></script>"#));
/// ```
///
/// Nothing checks that the router serves the file. A path that no service
/// answers loads nothing, and the browser's console shows the 404.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Asset(Url);

impl Asset {
    /// The file at `href`.
    ///
    /// `href` is a path the application serves, or a URL the page writes as
    /// given.
    ///
    /// # Panics
    ///
    /// Panics if `href` is empty, has a scheme other than `http` or `https`, or holds a
    /// space or a control character. In a `const` the panic is a compile
    /// error.
    #[must_use]
    pub const fn new(href: &'static str) -> Self {
        Self(url(href))
    }

    /// The URL as written in the program.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0.as_str()
    }

    /// The URL the page writes when a route under `base` serves it.
    pub(crate) fn under(self, base: Base) -> Cow<'static, str> {
        let href = self.as_str();
        let served = href.starts_with('/') && !href.starts_with("//");
        if served && base != Base::ROOT {
            Cow::Owned(base.join(href))
        } else {
            Cow::Borrowed(href)
        }
    }
}

/// `css` with each `url()` of a path the application serves under `base`.
///
/// The stylesheet writes a [`Url`] as `url("...")` with its text escaped, so
/// `url("/` followed by anything but `/` is the start of a root path. A `"`
/// inside a CSS string is written `\"`, so text inside a string cannot
/// match. The base is escaped the way the string around it is.
pub(crate) fn css_under(css: &str, base: Base) -> Cow<'_, str> {
    const OPEN: &str = "url(\"/";
    if base == Base::ROOT || !css.contains(OPEN) {
        return Cow::Borrowed(css);
    }
    let escaped = quoted(base.as_str()).to_string();
    let prefix = &escaped[1..escaped.len() - 1];
    let mut out = String::with_capacity(css.len() + prefix.len());
    let mut rest = css;
    while let Some(at) = rest.find(OPEN) {
        // `before` ends with `url("` and `path` starts with the `/`.
        let (before, path) = rest.split_at(at + OPEN.len() - 1);
        out.push_str(before);
        if !path[1..].starts_with('/') {
            out.push_str(prefix);
        }
        out.push('/');
        rest = &path[1..];
    }
    out.push_str(rest);
    Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::{Asset, css_under};
    use crate::route::Base;

    const APP: Base = Base::new("/app");

    #[test]
    fn a_path_the_application_serves_goes_under_the_base() {
        assert_eq!(Asset::new("/map.js").under(APP), "/app/map.js");
        assert_eq!(Asset::new("/map.js").under(Base::ROOT), "/map.js");
    }

    #[test]
    fn a_relative_url_and_another_host_are_written_as_given() {
        for url in [
            "./hypersig.js",
            "style.css",
            "//cdn.example/a.js",
            "https://cdn.example/a.js",
        ] {
            assert_eq!(Asset::new(url).under(APP), url);
        }
    }

    #[test]
    #[should_panic(expected = "a URL a page loads is a path")]
    fn a_script_scheme_panics() {
        let _ = Asset::new("javascript:alert(1)");
    }

    #[test]
    #[should_panic(expected = "a URL a page loads is a path")]
    fn an_empty_url_panics() {
        let _ = Asset::new("");
    }

    #[test]
    #[should_panic(expected = "a URL a page loads is a path")]
    fn a_mail_address_panics() {
        let _ = Asset::new("mailto:a@example.com");
    }

    #[test]
    fn a_root_path_in_css_goes_under_the_base() {
        let css = r#"@font-face{src:url("/inter.woff2")}.a{background-image:url("/a.png")}"#;
        assert_eq!(
            css_under(css, APP),
            r#"@font-face{src:url("/app/inter.woff2")}.a{background-image:url("/app/a.png")}"#
        );
        assert_eq!(css_under(css, Base::ROOT), css);
    }

    #[test]
    fn a_relative_url_another_host_and_a_string_in_css_are_written_as_given() {
        for css in [
            r#".a{background-image:url("a.png")}"#,
            r#".a{background-image:url("//cdn.example/a.png")}"#,
            r#".a{background-image:url("https://cdn.example/a.png")}"#,
            r#".a{content:"url(\"/a.png"}"#,
        ] {
            assert_eq!(css_under(css, APP), css);
        }
    }

    #[test]
    fn the_base_is_escaped_as_the_string_around_it() {
        assert_eq!(
            css_under(r#"url("/a.png")"#, Base::new("/a\"b<c")),
            r#"url("/a\"b\3c c/a.png")"#
        );
    }
}
