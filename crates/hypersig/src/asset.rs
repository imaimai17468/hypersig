//! The files a page loads in `<head>`: its scripts, stylesheets, icon, and runtime.

use std::borrow::Cow;

use crate::href;
use crate::route::Base;

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
pub struct Asset(&'static str);

impl Asset {
    /// The file at `url`.
    ///
    /// `url` is a path the application serves, or a URL the page writes as
    /// given.
    ///
    /// # Panics
    ///
    /// Panics if `url` is empty, has a scheme other than `http` or `https`, or holds a
    /// space or a control character. In a `const` the panic is a compile
    /// error.
    #[must_use]
    pub const fn new(url: &'static str) -> Self {
        assert!(
            !url.is_empty()
                && href::check(url.as_bytes()).is_ok()
                && !href::has_scheme(url.as_bytes(), b"mailto"),
            "an asset is a path or an `http` or `https` URL, with no space or control character"
        );
        Self(url)
    }

    /// The URL as written in the program.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// The URL the page writes when a route under `base` serves it.
    pub(crate) fn under(self, base: Base) -> Cow<'static, str> {
        let served = self.0.starts_with('/') && !self.0.starts_with("//");
        if served && base != Base::ROOT {
            Cow::Owned(base.join(self.0))
        } else {
            Cow::Borrowed(self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Asset;
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
    #[should_panic(expected = "an asset is a path")]
    fn a_script_scheme_panics() {
        let _ = Asset::new("javascript:alert(1)");
    }

    #[test]
    #[should_panic(expected = "an asset is a path")]
    fn an_empty_url_panics() {
        let _ = Asset::new("");
    }

    #[test]
    #[should_panic(expected = "an asset is a path")]
    fn a_mail_address_panics() {
        let _ = Asset::new("mailto:a@example.com");
    }
}
