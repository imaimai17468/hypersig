//! Pages served at typed paths, and the links that point to them.

use std::fmt;
use std::marker::PhantomData;

use serde_json::{Value, json};

use crate::expr::{Expr, IntoExpr};

/// A page served at a path whose parameters are `P`.
///
/// Declare one as a `const` and use the same value on both sides: the router
/// serves the page with `RouterExt::page`, and an element links to it with
/// [`El::href`](crate::html::El::href) and [`Route::link`] or
/// [`Route::link_with`]. Both sides then agree on the path and on the type
/// of each parameter in it.
///
/// A parameter is a whole segment written `{name}`, as in axum. `P` is `()`
/// for a path without one, a [`Segment`] type for one, and a tuple for two or
/// three, in the order they appear. The name inside the braces is for the
/// reader and axum, and the position decides which value fills it.
///
/// ```
/// use hypersig::html::a;
/// use hypersig::{Route, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Team {
///     lead: i32,
/// }
///
/// const HOME: Route = Route::new("/");
/// const MEMBER: Route<i32> = Route::new("/members/{id}");
/// const POST: Route<(String, i32)> = Route::new("/blog/{slug}/comments/{id}");
///
/// let home = a::<Team>().href(HOME.link()).render();
/// assert!(home.contains(r#"href="/""#));
///
/// let lead = a().href(MEMBER.link_with(Team::lead())).render();
/// assert!(lead.contains("data-hs-attr"));
///
/// let comment = a::<Team>().href(POST.link_with(("hello world", 3))).render();
/// assert!(comment.contains(r#"href="/blog/hello%20world/comments/3""#));
/// ```
pub struct Route<P = ()> {
    path: &'static str,
    marker: PhantomData<fn(P)>,
}

impl<P: Params> Route<P> {
    /// A page served at `path`, whose `{name}` segments take the values of `P`.
    ///
    /// # Panics
    ///
    /// Panics if `path` does not start with `/`, if a `{` or `}` does not
    /// enclose a whole segment with a name, or if the number of such segments
    /// differs from the number of values in `P`. In a `const`, each of these is
    /// a compile error.
    #[must_use]
    pub const fn new(path: &'static str) -> Self {
        assert!(
            !path.is_empty() && path.as_bytes()[0] == b'/',
            "a route path starts with `/`"
        );
        assert!(
            count_params(path) == P::COUNT,
            "the route's `{{name}}` segments differ in number from the values of its parameter type"
        );
        Self {
            path,
            marker: PhantomData,
        }
    }

    /// The path as axum matches it, with each parameter still written `{name}`.
    #[must_use]
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// A page under this one, served at the whole path `path`.
    ///
    /// `path` repeats this route's path and continues it after a `/`, and
    /// `Q` repeats this route's parameters and may add more after them, so a
    /// link to the child needs every value its path holds.
    ///
    /// ```
    /// use hypersig::Route;
    ///
    /// const ORG: Route<String> = Route::new("/orgs/{org}");
    /// const MEMBER: Route<(String, i32)> = ORG.child("/orgs/{org}/members/{id}");
    /// assert_eq!(MEMBER.path(), "/orgs/{org}/members/{id}");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `path` does not continue this route's path, and for every
    /// reason [`Route::new`] does. In a `const`, each of these is a compile error.
    #[must_use]
    pub const fn child<Q: Params + Extends<P>>(&self, path: &'static str) -> Route<Q> {
        assert!(
            continues(path, self.path),
            "a child route's path is its parent's path followed by `/` and more segments"
        );
        Route::new(path)
    }

    /// A link to this page with the parameters `params`.
    ///
    /// `params` can be literals, which give a fixed link, or expressions such
    /// as `item.at(User::id())`, which the browser evaluates and keeps up to
    /// date. Each value is percent-encoded as `encodeURIComponent` does.
    #[must_use]
    pub fn link_with<S>(&self, params: impl IntoExpr<S, P>) -> Link<S> {
        let params = params.into_expr().into_json();
        let values: Vec<Value> = match P::COUNT {
            1 => vec![params],
            count => match params.as_array().map(Vec::as_slice) {
                Some([op, parts @ ..]) if op == "tuple" && parts.len() == count => parts.to_vec(),
                _ => (0..count)
                    .map(|index| json!(["nth", params, ["lit", index]]))
                    .collect(),
            },
        };
        Link::new(self.path, values)
    }
}

impl Route<()> {
    /// A link to this page, which has no parameters.
    #[must_use]
    pub fn link<S>(&self) -> Link<S> {
        Link::new(self.path, Vec::new())
    }
}

impl<P> Clone for Route<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for Route<P> {}

impl<P> fmt::Debug for Route<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Route").field(&self.path).finish()
    }
}

/// The number of `{name}` segments in `path`.
///
/// Panics where a brace does not enclose a whole, named segment, so that
/// `Route::new` in a `const` fails to compile on it.
const fn count_params(path: &str) -> usize {
    let bytes = path.as_bytes();
    let mut count = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                assert!(
                    bytes[i - 1] == b'/',
                    "a route parameter is a whole segment, so `{{` follows `/`"
                );
                let start = i + 1;
                i = start;
                while i < bytes.len() && bytes[i] != b'}' {
                    assert!(
                        bytes[i] != b'{' && bytes[i] != b'/' && bytes[i] != b'*',
                        "a route parameter is `{{name}}`, with no `{{`, `/`, or `*` inside"
                    );
                    i += 1;
                }
                assert!(i < bytes.len(), "a route parameter's `{{` has no `}}`");
                assert!(i > start, "a route parameter has a name between its braces");
                assert!(
                    i + 1 == bytes.len() || bytes[i + 1] == b'/',
                    "a route parameter is a whole segment, so `}}` ends it"
                );
                count += 1;
            }
            b'}' => panic!("a route path has a `}}` with no `{{`"),
            _ => {}
        }
        i += 1;
    }
    count
}

/// Whether `path` is `parent` followed by one or more segments.
const fn continues(path: &str, parent: &str) -> bool {
    let (path, parent) = (path.as_bytes(), parent.as_bytes());
    if path.len() <= parent.len() {
        return false;
    }
    let mut i = 0;
    while i < parent.len() {
        if path[i] != parent[i] {
            return false;
        }
        i += 1;
    }
    parent[parent.len() - 1] == b'/' || path[parent.len()] == b'/'
}

/// The parts of `path` around its parameters: one more than there are parameters.
fn literal_parts(path: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut rest = path;
    while let Some(open) = rest.find('{') {
        parts.push(&rest[..open]);
        let close = rest[open..]
            .find('}')
            .expect("`Route::new` checked that every `{` has a `}`");
        rest = &rest[open + close + 1..];
    }
    parts.push(rest);
    parts
}

/// A link to a [`Route`], for [`El::href`](crate::html::El::href) on the page whose signals are `S`.
pub struct Link<S>(LinkKind<S>);

enum LinkKind<S> {
    Fixed(String),
    Bound(Expr<S, String>),
}

impl<S> Link<S> {
    fn new(path: &'static str, values: Vec<Value>) -> Self {
        let parts = literal_parts(path);
        let literals: Option<Vec<&Value>> = values
            .iter()
            .map(|value| match value.as_array().map(Vec::as_slice) {
                Some([op, literal]) if op == "lit" => Some(literal),
                _ => None,
            })
            .collect();
        if let Some(literals) = literals {
            let mut url = parts[0].to_owned();
            for (literal, part) in literals.into_iter().zip(&parts[1..]) {
                url.push_str(&encode_uri_component(&crate::ssr::js_string(literal)));
                url.push_str(part);
            }
            return Self(LinkKind::Fixed(url));
        }
        let mut ast = json!(["lit", parts[0]]);
        for (value, part) in values.into_iter().zip(&parts[1..]) {
            ast = json!(["concat", ast, ["uri", value]]);
            if !part.is_empty() {
                ast = json!(["concat", ast, ["lit", part]]);
            }
        }
        Self(LinkKind::Bound(Expr::from_json(ast)))
    }

    /// The fixed URL, or the expression the browser evaluates into one.
    pub(crate) fn into_parts(self) -> Result<String, Expr<S, String>> {
        match self.0 {
            LinkKind::Fixed(url) => Ok(url),
            LinkKind::Bound(expr) => Err(expr),
        }
    }
}

impl<S> fmt::Debug for Link<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            LinkKind::Fixed(url) => f.debug_tuple("Link").field(url).finish(),
            LinkKind::Bound(expr) => f.debug_tuple("Link").field(expr).finish(),
        }
    }
}

/// Percent-encodes `text` as JavaScript's `encodeURIComponent` does.
pub(crate) fn encode_uri_component(text: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(HEX[usize::from(byte >> 4)]));
            out.push(char::from(HEX[usize::from(byte & 0xf)]));
        }
    }
    out
}

/// A type one `{name}` segment of a [`Route`] holds.
///
/// It is implemented for `String` and the integer types, whose text the
/// browser's `String` and the server write the same way. An `i64` or `u64`
/// past 2^53 loses precision in the browser, as it does in any signal.
pub trait Segment: sealed::Sealed + Sized {
    /// Reads the value from a percent-decoded segment, or `None` if it is not one.
    #[doc(hidden)]
    fn parse(segment: &str) -> Option<Self>;
}

macro_rules! integer_segment {
    ($($ty:ty),*) => {$(
        impl Segment for $ty {
            fn parse(segment: &str) -> Option<Self> {
                segment.parse().ok()
            }
        }
    )*};
}

integer_segment!(i32, i64, u32, u64);

impl Segment for String {
    fn parse(segment: &str) -> Option<Self> {
        Some(segment.to_owned())
    }
}

/// The parameters of a [`Route`]: `()`, one [`Segment`], or a tuple of two or three.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be the parameters of a route",
    note = "a route takes `()`, one integer or `String`, or a tuple of two or three of them"
)]
pub trait Params: sealed::Sealed + Sized {
    /// The number of `{name}` segments the path has.
    #[doc(hidden)]
    const COUNT: usize;

    /// Reads the values from the percent-decoded segments, in path order.
    #[doc(hidden)]
    fn from_segments(segments: &[&str]) -> Option<Self>;
}

impl Params for () {
    const COUNT: usize = 0;

    fn from_segments(_: &[&str]) -> Option<Self> {
        Some(())
    }
}

impl<T: Segment> Params for T {
    const COUNT: usize = 1;

    fn from_segments(segments: &[&str]) -> Option<Self> {
        match segments {
            [one] => T::parse(one),
            _ => None,
        }
    }
}

impl<A: Segment, B: Segment> Params for (A, B) {
    const COUNT: usize = 2;

    fn from_segments(segments: &[&str]) -> Option<Self> {
        match segments {
            [a, b] => Some((A::parse(a)?, B::parse(b)?)),
            _ => None,
        }
    }
}

impl<A: Segment, B: Segment, C: Segment> Params for (A, B, C) {
    const COUNT: usize = 3;

    fn from_segments(segments: &[&str]) -> Option<Self> {
        match segments {
            [a, b, c] => Some((A::parse(a)?, B::parse(b)?, C::parse(c)?)),
            _ => None,
        }
    }
}

/// Parameters that start with the parameters `P` of a parent [`Route`].
///
/// [`Route::child`] requires it, so that a child's parameters repeat its
/// parent's in the same order and may add more after them.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not start with `{P}`, the parameters of the parent route",
    note = "a child route's parameters repeat its parent's, in order, and may add more after them"
)]
pub trait Extends<P>: Params {}

impl Extends<()> for () {}
impl<A: Segment> Extends<()> for A {}
impl<A: Segment> Extends<A> for A {}
impl<A: Segment, B: Segment> Extends<()> for (A, B) {}
impl<A: Segment, B: Segment> Extends<A> for (A, B) {}
impl<A: Segment, B: Segment> Extends<(A, B)> for (A, B) {}
impl<A: Segment, B: Segment, C: Segment> Extends<()> for (A, B, C) {}
impl<A: Segment, B: Segment, C: Segment> Extends<A> for (A, B, C) {}
impl<A: Segment, B: Segment, C: Segment> Extends<(A, B)> for (A, B, C) {}
impl<A: Segment, B: Segment, C: Segment> Extends<(A, B, C)> for (A, B, C) {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for () {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for String {}
    impl<A: super::Segment, B: super::Segment> Sealed for (A, B) {}
    impl<A: super::Segment, B: super::Segment, C: super::Segment> Sealed for (A, B, C) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_whole_segment_parameters() {
        assert_eq!(count_params("/"), 0);
        assert_eq!(count_params("/users/{id}"), 1);
        assert_eq!(count_params("/a/{x}/b/{y}/{z}"), 3);
    }

    #[test]
    #[should_panic(expected = "`{` follows `/`")]
    fn rejects_a_parameter_inside_a_segment() {
        let _ = count_params("/user-{id}");
    }

    #[test]
    #[should_panic(expected = "no `{`, `/`, or `*` inside")]
    fn rejects_a_wildcard() {
        let _ = count_params("/files/{*rest}");
    }

    #[test]
    fn a_child_continues_its_parent_after_a_slash() {
        assert!(continues("/orgs/{org}/members", "/orgs/{org}"));
        assert!(continues("/members", "/"));
        assert!(!continues("/orgs/{org}x", "/orgs/{org}"));
        assert!(!continues("/orgs/{org}", "/orgs/{org}"));
        assert!(!continues("/teams/{org}/members", "/orgs/{org}"));
    }

    #[test]
    fn splits_the_literal_parts() {
        assert_eq!(literal_parts("/a/{x}/b/{y}"), ["/a/", "/b/", ""]);
        assert_eq!(literal_parts("/"), ["/"]);
    }

    #[test]
    fn encodes_as_encode_uri_component() {
        assert_eq!(
            encode_uri_component("a b/é?&=#"),
            "a%20b%2F%C3%A9%3F%26%3D%23"
        );
        assert_eq!(encode_uri_component("-_.!~*'()"), "-_.!~*'()");
    }

    #[test]
    fn literal_parameters_give_a_fixed_link() {
        const POST: Route<(String, i32)> = Route::new("/blog/{slug}/comments/{id}");
        let link: Link<()> = POST.link_with(("a/b", -4));
        assert_eq!(
            link.into_parts().ok().as_deref(),
            Some("/blog/a%2Fb/comments/-4")
        );
    }

    #[test]
    fn parses_segments_in_path_order() {
        assert_eq!(
            <(String, i32)>::from_segments(&["x", "7"]),
            Some(("x".to_owned(), 7))
        );
        assert_eq!(<i32>::from_segments(&["seven"]), None);
        assert_eq!(<()>::from_segments(&[]), Some(()));
    }
}
