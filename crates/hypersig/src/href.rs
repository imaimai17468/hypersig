//! The URLs a page points `href` and `src` at: the router's own pages, and
//! [`SafeUrl`] for an address the router does not serve.

use std::borrow::Cow;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, json};

use crate::expr::{Expr, IntoExpr};
use crate::route::{Link, Url};
use crate::signal::Sig;

/// A URL whose scheme cannot run a script: `http:`, `https:`, `mailto:`, or
/// none at all, as in `/logo.png`, `#top`, `?page=2`, or `//cdn.example/a.js`.
///
/// [`El::href`](crate::html::El::href) and [`El::src`](crate::html::El::src)
/// take it for an address the router does not serve. A field of this type in
/// a page's signals holds only such a URL, because deserializing it checks
/// the same rule, so a link to a URL a visitor typed cannot be a
/// `javascript:` link.
///
/// ```
/// use hypersig::SafeUrl;
///
/// const REPOSITORY: SafeUrl = SafeUrl::new("https://github.com/imaimai17468/hypersig");
/// assert_eq!(REPOSITORY.as_str(), "https://github.com/imaimai17468/hypersig");
///
/// let typed = SafeUrl::parse("javascript:alert(1)");
/// assert!(typed.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct SafeUrl(Cow<'static, str>);

impl SafeUrl {
    /// The URL `url`, written in the program.
    ///
    /// # Panics
    ///
    /// Panics if `url` has a scheme other than `http`, `https`, or `mailto`,
    /// or holds a space or a control character. In a `const` the panic is a
    /// compile error.
    #[must_use]
    pub const fn new(url: &'static str) -> Self {
        if let Err(reason) = check(url.as_bytes()) {
            panic!("{}", reason);
        }
        Self(Cow::Borrowed(url))
    }

    /// Reads a URL from outside the program, such as one a visitor typed.
    ///
    /// # Errors
    ///
    /// Returns [`UnsafeUrl`] where [`SafeUrl::new`] would panic.
    pub fn parse(url: impl Into<String>) -> Result<Self, UnsafeUrl> {
        let url = url.into();
        match check(url.as_bytes()) {
            Ok(()) => Ok(Self(Cow::Owned(url))),
            Err(reason) => Err(UnsafeUrl { url, reason }),
        }
    }

    /// The URL as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SafeUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SafeUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::parse(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl<S> IntoExpr<S, SafeUrl> for SafeUrl {
    fn into_expr(self) -> Expr<S, SafeUrl> {
        Expr::from_json(json!(["lit", self.0]))
    }
}

/// Text that [`SafeUrl::parse`] refused, with the rule it broke.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafeUrl {
    url: String,
    reason: &'static str,
}

impl fmt::Display for UnsafeUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a safe URL: {}", self.url, self.reason)
    }
}

impl std::error::Error for UnsafeUrl {}

/// Whether `url` is one [`SafeUrl`] holds.
const fn check(url: &[u8]) -> Result<(), &'static str> {
    // The browser drops spaces and control characters around and inside the
    // scheme before reading it, so `java\tscript:` runs as `javascript:`.
    let mut i = 0;
    while i < url.len() {
        if url[i] <= b' ' || url[i] == 0x7f {
            return Err("it holds a space or a control character");
        }
        i += 1;
    }
    let mut end = 0;
    while end < url.len() && scheme_byte(url[end], end == 0) {
        end += 1;
    }
    // Without a scheme before the first `:`, the URL is relative to the page.
    if end == 0 || end == url.len() || url[end] != b':' {
        return Ok(());
    }
    if same_ascii(url, end, b"http")
        || same_ascii(url, end, b"https")
        || same_ascii(url, end, b"mailto")
    {
        Ok(())
    } else {
        Err("its scheme is not `http`, `https`, or `mailto`")
    }
}

/// Whether `byte` may sit in a URL scheme, whose first character is a letter.
const fn scheme_byte(byte: u8, first: bool) -> bool {
    byte.is_ascii_alphabetic()
        || (!first && (byte.is_ascii_digit() || matches!(byte, b'+' | b'-' | b'.')))
}

/// Whether the first `len` bytes of `url` are `scheme`, ignoring case.
const fn same_ascii(url: &[u8], len: usize, scheme: &[u8]) -> bool {
    if len != scheme.len() {
        return false;
    }
    let mut i = 0;
    while i < len {
        if url[i].to_ascii_lowercase() != scheme[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// A value [`El::href`](crate::html::El::href) and
/// [`El::src`](crate::html::El::src) take: a [`Link`] or a [`Url`] to a page
/// the router serves, a [`SafeUrl`], or an expression of one, which the
/// browser keeps up to date as the signals change.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a URL an `href` or a `src` takes on the page whose signals are `{S}`",
    note = "give a `Link` or a `Url` from a route, a `SafeUrl`, or a signal or expression of `SafeUrl`; a `String` becomes one through `SafeUrl::parse`"
)]
pub trait IntoHref<S>: sealed::Sealed {
    /// The fixed URL, or the expression the browser evaluates into one.
    #[doc(hidden)]
    fn into_parts(self) -> Result<String, Expr<S, String>>;
}

mod sealed {
    pub trait Sealed {}
    impl<S> Sealed for crate::Link<S> {}
    impl Sealed for crate::Url {}
    impl Sealed for super::SafeUrl {}
    impl<S> Sealed for crate::Expr<S, super::SafeUrl> {}
    impl<S, R> Sealed for crate::Sig<S, super::SafeUrl, R> {}
}

impl<S> IntoHref<S> for Link<S> {
    fn into_parts(self) -> Result<String, Expr<S, String>> {
        Link::into_parts(self)
    }
}

impl<S> IntoHref<S> for Url {
    fn into_parts(self) -> Result<String, Expr<S, String>> {
        Ok(self.as_str().to_owned())
    }
}

impl<S> IntoHref<S> for SafeUrl {
    fn into_parts(self) -> Result<String, Expr<S, String>> {
        Ok(self.0.into_owned())
    }
}

impl<S> IntoHref<S> for Expr<S, SafeUrl> {
    fn into_parts(self) -> Result<String, Expr<S, String>> {
        match self.into_json() {
            Value::Array(items)
                if items.len() == 2 && items[0] == "lit" && items[1].is_string() =>
            {
                Ok(items[1].as_str().unwrap_or_default().to_owned())
            }
            // A `SafeUrl` is a string in the page's signals, so the browser
            // reads the same value as text.
            ast => Err(Expr::from_json(ast)),
        }
    }
}

impl<S, R> IntoHref<S> for Sig<S, SafeUrl, R> {
    fn into_parts(self) -> Result<String, Expr<S, String>> {
        IntoExpr::<S, SafeUrl>::into_expr(self).into_parts()
    }
}

#[cfg(test)]
mod tests {
    use super::SafeUrl;

    #[test]
    fn keeps_a_url_whose_scheme_cannot_run_a_script() {
        for url in [
            "https://example.com/a?b#c",
            "HTTP://example.com",
            "mailto:a@example.com",
            "/logo.png",
            "#top",
            "?page=2",
            "",
            "//cdn.example/a.js",
            "docs/a:b",
            "./a:b",
        ] {
            assert!(SafeUrl::parse(url).is_ok(), "{url}");
        }
    }

    #[test]
    fn refuses_a_url_that_could_run_a_script() {
        for url in [
            "javascript:alert(1)",
            "JavaScript:alert(1)",
            "java\tscript:alert(1)",
            " javascript:alert(1)",
            "data:text/html,<script>alert(1)</script>",
            "vbscript:x",
            "blob:https://example.com/x",
            "a b",
        ] {
            assert!(SafeUrl::parse(url).is_err(), "{url:?}");
        }
    }

    #[test]
    fn deserializing_checks_the_same_rule() {
        serde_json::from_str::<SafeUrl>(r#""https://example.com""#).expect("an https: URL");
        let error = serde_json::from_str::<SafeUrl>(r#""javascript:alert(1)""#)
            .expect_err("a javascript: URL");
        assert!(error.to_string().contains("not a safe URL"), "{error}");
    }
}
