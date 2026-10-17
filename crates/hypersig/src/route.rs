//! Pages served at typed paths, and the links that point to them.

use std::fmt;
use std::marker::PhantomData;

use serde_json::{Value, json};

use crate::expr::{Expr, IntoExpr, Stmt};
use crate::signal::Signals;

/// A page served at a path whose parameters are `P`, with search parameters `Q`.
///
/// Declare one in [`routes!`](crate::routes), which names the handler that
/// serves it, and link to it with [`El::href`](crate::html::El::href) and
/// [`Route::link`] or [`Route::link_with`]. The router and the links then
/// agree on the path and on the type of each parameter in it, and a route
/// without a handler cannot be declared.
/// `routes!` needs the `axum` feature, so a build without it declares no
/// routes and its pages link with `attr("href", ...)`.
///
/// A parameter is a whole segment written `{name}`, as in axum. `P` is `()`
/// for a path without one, a [`Segment`] type for one, and a tuple for two to
/// six, in the order they appear. The name inside the braces is for the
/// reader and axum, and the position decides which value fills it.
///
/// `Q` is `()` for a page without a query string, or a struct with
/// `#[derive(Signals)]` whose fields the query string holds. [`Search`] says
/// how its fields are written.
///
/// ```
/// use hypersig::html::a;
/// use hypersig::{Page, Route, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Team {
///     lead: i32,
/// }
///
/// # async fn page<P>((): (), _: P) -> Option<Page<Team>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const HOME: Route = "/" => page;
///     const MEMBER: Route<i32> = "/members/{id}" => page;
///     const POST: Route<(String, i32)> = "/blog/{slug}/comments/{id}" => page;
/// }
///
/// let home = a::<Team>().href(HOME.link()).render();
/// assert!(home.contains(r#"href="/""#));
///
/// let lead = a().href(MEMBER.link_with(Team::lead())).render();
/// assert!(lead.contains("data-hs-attr"));
///
/// let comment = a::<Team>().href(POST.link_with(("hello world", 3))).render();
/// assert!(comment.contains(r#"href="/blog/hello%20world/comments/3""#));
/// # let _ = pages;
/// ```
pub struct Route<P = (), Q = ()> {
    path: &'static str,
    marker: PhantomData<fn(P, Q)>,
}

impl<P: Params, Q: Search> Route<P, Q> {
    /// A page served at `path`, whose `{name}` segments take the values of `P`.
    ///
    /// Panics if `path` does not start with `/`, if a `{` or `}` does not
    /// enclose a whole segment with a name, or if the number of such segments
    /// differs from the number of values in `P`. `routes!` calls it in a
    /// `const`, where each of these is a compile error.
    #[must_use]
    pub(crate) const fn new(path: &'static str) -> Self {
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
    /// `C` repeats this route's parameters and may add more after them, so a
    /// link to the child needs every value its path holds. The child's
    /// search parameters `R` are its own. Panics where either does not hold,
    /// and for every reason `new` does.
    #[must_use]
    pub(crate) const fn child<C: Params + Extends<P>, R: Search>(
        &self,
        path: &'static str,
    ) -> Route<C, R> {
        assert!(
            continues(path, self.path),
            "a child route's path is its parent's path followed by `/` and more segments"
        );
        Route::new(path)
    }

    /// A link to this page with the arguments `args`.
    ///
    /// `args` are the path's parameters `P`, or for a page with search
    /// parameters, the tuple `(P, Q)`, or `Q` alone where `P` is `()`. They
    /// can be literals, which give a fixed link, or expressions such as
    /// `item.at(User::id())`, which the browser evaluates and keeps up to
    /// date. Each value is percent-encoded as `encodeURIComponent` does.
    #[must_use]
    pub fn link_with<S>(&self, args: impl IntoExpr<S, Q::Args<P>>) -> Link<S> {
        let args = args.into_expr().into_json();
        let (params, search) = match (Q::TYPED, P::COUNT) {
            (false, _) => (args, None),
            (true, 0) => (Value::Null, Some(args)),
            (true, _) => {
                let mut parts = split_tuple(args, 2).into_iter();
                let params = parts.next().unwrap_or_default();
                (params, parts.next())
            }
        };
        let values = match P::COUNT {
            0 => Vec::new(),
            1 => vec![params],
            count => split_tuple(params, count),
        };
        Link::new(self.path, values, search)
    }

    /// The URL of this page with the arguments `args`, written on the server.
    ///
    /// Use it where the server sends the browser to the page: a page
    /// handler's redirect, or `Patches::navigate` in an action. `args` take
    /// the shape they take in [`Route::link_with`], and each value is
    /// percent-encoded as there.
    ///
    /// ```
    /// # use hypersig::{Page, Route, Signals};
    /// # #[derive(Signals, serde::Serialize, serde::Deserialize)]
    /// # struct Blog {}
    /// # async fn post((): (), _: (String, i32)) -> Option<Page<Blog>> { None }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const POST: Route<(String, i32)> = "/blog/{slug}/comments/{id}" => post;
    /// }
    ///
    /// assert_eq!(POST.url(("hello world".to_owned(), 3)).as_str(), "/blog/hello%20world/comments/3");
    /// # let _ = pages;
    /// ```
    #[must_use]
    pub fn url(&self, args: Q::Args<P>) -> Url {
        let (params, search) = Q::split(args);
        let parts = literal_parts(self.path);
        let mut url = parts[0].to_owned();
        for (segment, part) in params.segments().iter().zip(&parts[1..]) {
            url.push_str(&encode_uri_component(segment));
            url.push_str(part);
        }
        url.push_str(&search.query());
        Url(url)
    }
}

impl Route<(), ()> {
    /// A link to this page, which has no parameters.
    #[must_use]
    pub fn link<S>(&self) -> Link<S> {
        Link::new(self.path, Vec::new(), None)
    }
}

/// Declares the pages of an application and the function that serves them.
///
/// The first line names the function and the router state its handlers
/// take. Each line after it declares a [`Route`] constant with its path and
/// the handler that builds the page, so every route has a handler. The
/// generated function registers each route with its handler on `GET`. Add
/// `in PARENT` after the path to declare a page under another page of an
/// earlier line: its path continues the parent's and its parameters start
/// with the parent's.
///
/// ```
/// use axum::Router;
/// use hypersig::html::{a, h1};
/// use hypersig::{Page, Route, RouterExt, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Profile {
///     name: String,
/// }
///
/// hypersig::routes! {
///     /// Serves every page of the application.
///     fn pages(Router<()>);
///     const HOME: Route = "/" => home;
///     const ORG: Route<String> = "/orgs/{org}" => org;
///     const MEMBER: Route<(String, i32)> = "/orgs/{org}/members/{id}" in ORG => member;
/// }
///
/// async fn home((): (), (): ()) -> Page<Profile> {
///     let signals = Profile { name: "Ada".to_owned() };
///     Page::new("Home", signals).child(a().href(MEMBER.link_with(("acme", 1))).text("Ada"))
/// }
///
/// async fn org((): (), org: String) -> Page<Profile> {
///     Page::new(org.clone(), Profile { name: org }).child(h1().bind_text(Profile::name()))
/// }
///
/// async fn member((): (), (_org, id): (String, i32)) -> Option<Page<Profile>> {
///     let name = ["Ada", "Grace"].get(usize::try_from(id).ok()?)?;
///     let signals = Profile { name: (*name).to_owned() };
///     Some(Page::new(*name, signals).child(a().href(HOME.link()).text("Home")))
/// }
///
/// let _app: Router = pages(Router::new()).hypersig_runtime();
/// ```
///
/// A handler receives the router state and the route's arguments, already
/// parsed, and answers with a [`Page`](crate::Page), an `Option<Page>`, or a
/// [`PageResponse`](crate::PageResponse). A path whose segments do not parse
/// into the parameter types, or a handler that answers `None`, gets a 404,
/// and a query string that does not deserialize into the search parameters
/// gets a 400.
///
/// # What stays unchecked
///
/// The macro guarantees that each route has a handler, and nothing checks
/// that the generated function is called. A function that is never called
/// leaves every page of its block answering 404. rustc's `dead_code` lint
/// does not report the function, because it comes from the macro, but it
/// does report a handler `fn` that only the function names as never used.
/// A closure written as the handler gets no such warning. Each `routes!`
/// block is its own function, so an application with several blocks calls
/// each of them.
///
/// Call the function on the router the application serves, never on one
/// passed to `Router::nest`. The links point to each route's path, and a
/// nested router would serve the pages under a longer one, so such a page
/// answers every request with a 500 that names the two paths.
///
/// A parent after `in` is a page itself. A path prefix that has no page of
/// its own cannot be a parent.
///
/// # Compile errors
///
/// A line without `=> handler`, a path that does not start with `/`, a `{`
/// or `}` that does not enclose a whole named segment, a path whose
/// segments differ in number from the route's parameters, and a child whose
/// path or parameters do not continue its parent's.
///
/// This macro needs the `axum` feature.
#[cfg(feature = "axum")]
#[macro_export]
macro_rules! routes {
    (@route $path:literal) => {
        $crate::_private::route($path)
    };
    (@route $path:literal, $parent:ident) => {
        $crate::_private::child(&$parent, $path)
    };
    (
        $(#[$fn_attr:meta])*
        $fn_vis:vis fn $fn_name:ident(Router<$state:ty>);
        $(
            $(#[$attr:meta])*
            $vis:vis const $name:ident: $ty:ty = $path:literal $(in $parent:ident)? => $handler:expr;
        )+
    ) => {
        $(
            $(#[$attr])*
            $vis const $name: $ty = $crate::routes!(@route $path $(, $parent)?);
        )+

        $(#[$fn_attr])*
        $fn_vis fn $fn_name(
            router: $crate::_private::Router<$state>,
        ) -> $crate::_private::Router<$state> {
            $(
                let router = <$crate::_private::Router<$state> as $crate::RouterExt<$state>>::page(
                    router, $name, $handler,
                );
            )+
            router
        }
    };
    ($($input:tt)*) => {
        ::core::compile_error!(
            "`routes!` takes `fn NAME(Router<STATE>);`, then one `const NAME: Route<..> = \"/path\" => handler;` per page, each naming its handler after `=>`"
        );
    };
}

/// The parts of a tuple expression, or an `nth` expression for each where it is not one.
fn split_tuple(ast: Value, count: usize) -> Vec<Value> {
    match ast {
        Value::Array(mut items) if items.len() == count + 1 && items[0] == "tuple" => {
            items.split_off(1)
        }
        ast => (0..count)
            .map(|index| json!(["nth", ast, ["lit", index]]))
            .collect(),
    }
}

/// The URL of a page, written on the server by [`Route::url`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url(String);

impl Url {
    /// The URL as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<P, Q> Clone for Route<P, Q> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P, Q> Copy for Route<P, Q> {}

impl<P, Q> fmt::Debug for Route<P, Q> {
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
    fn new(path: &'static str, values: Vec<Value>, search: Option<Value>) -> Self {
        let parts = literal_parts(path);
        let literal = |value: &Value| match value.as_array().map(Vec::as_slice) {
            Some([op, literal]) if op == "lit" => Some(literal.clone()),
            _ => None,
        };
        let literals: Option<Vec<Value>> = values.iter().map(literal).collect();
        let search_literal = search.as_ref().map(literal);
        if let (Some(literals), None | Some(Some(_))) = (literals, &search_literal) {
            let mut url = parts[0].to_owned();
            for (literal, part) in literals.iter().zip(&parts[1..]) {
                url.push_str(&encode_uri_component(&crate::ssr::js_string(literal)));
                url.push_str(part);
            }
            if let Some(Some(search)) = &search_literal {
                url.push_str(&query_string(search));
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
        if let Some(search) = search {
            ast = json!(["concat", ast, ["query", search]]);
        }
        Self(LinkKind::Bound(Expr::from_json(ast)))
    }

    /// A statement that loads the linked page, as following the link would.
    ///
    /// ```
    /// use hypersig::html::button;
    /// use hypersig::{Route, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Search {
    ///     id: i32,
    /// }
    ///
    /// # async fn member((): (), _: i32) -> Option<hypersig::Page<Search>> { None }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const MEMBER: Route<i32> = "/members/{id}" => member;
    /// }
    ///
    /// let _open = button().on_click(MEMBER.link_with(Search::id()).navigate()).text("Open");
    /// # let _ = pages;
    /// ```
    #[must_use]
    pub fn navigate(self) -> Stmt<S> {
        let url = match self.0 {
            LinkKind::Fixed(url) => json!(["lit", url]),
            LinkKind::Bound(expr) => expr.into_json(),
        };
        Stmt::new(json!(["navigate", url]))
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

/// The query string of `value`, the JSON of a [`Search`] struct, as the runtime's `query` writes it.
///
/// The fields are written in the order of their names, compared byte by byte.
/// A `null` field is left out, and each name and value is percent-encoded as
/// `encodeURIComponent` does, after the value is written as `String` would.
pub(crate) fn query_string(value: &Value) -> String {
    let Value::Object(fields) = value else {
        return String::new();
    };
    let mut pairs: Vec<(&String, &Value)> = fields.iter().filter(|(_, v)| !v.is_null()).collect();
    pairs.sort_by(|a, b| a.0.cmp(b.0));
    let mut out = String::new();
    for (name, value) in pairs {
        out.push(if out.is_empty() { '?' } else { '&' });
        out.push_str(&encode_uri_component(name));
        out.push('=');
        out.push_str(&encode_uri_component(&crate::ssr::js_string(value)));
    }
    out
}

/// A type one `{name}` segment of a [`Route`] holds.
///
/// It is implemented for `String` and the integer types, whose text the
/// browser's `String` and the server write the same way. An `i64` or `u64`
/// past 2^53 loses precision in the browser, as it does in any signal.
///
/// `#[derive(Segment)]` implements it for an enum whose variants have no
/// fields. The segment is the variant's name, which is also the string serde
/// writes for the variant, so a link that reads the enum from a signal and the
/// server write the same URL. The derive rejects `#[serde(...)]` on the enum
/// and its variants, because a rename would make the two disagree.
///
/// ```
/// use hypersig::{Page, Route, Segment, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Segment, Serialize, Deserialize)]
/// enum Tab {
///     Overview,
///     Settings,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Team {}
///
/// # async fn tab((): (), _: (String, Tab)) -> Option<Page<Team>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const TAB: Route<(String, Tab)> = "/teams/{team}/{tab}" => tab;
/// }
///
/// assert_eq!(TAB.url(("red".to_owned(), Tab::Settings)).as_str(), "/teams/red/Settings");
/// # let _ = pages;
/// ```
pub trait Segment: sealed::Sealed + Sized {
    /// Reads the value from a percent-decoded segment, or `None` if it is not one.
    #[doc(hidden)]
    fn parse(segment: &str) -> Option<Self>;

    /// Writes the value as the browser's `String` would, before percent-encoding.
    #[doc(hidden)]
    fn write(&self) -> String;
}

macro_rules! integer_segment {
    ($($ty:ty),*) => {$(
        impl Segment for $ty {
            fn parse(segment: &str) -> Option<Self> {
                segment.parse().ok()
            }

            fn write(&self) -> String {
                self.to_string()
            }
        }
    )*};
}

integer_segment!(i32, i64, u32, u64);

impl Segment for String {
    fn parse(segment: &str) -> Option<Self> {
        Some(segment.to_owned())
    }

    fn write(&self) -> String {
        self.clone()
    }
}

/// The parameters of a [`Route`]: `()`, one [`Segment`], or a tuple of two to six.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be the parameters of a route",
    note = "a route takes `()`, one `Segment` (an integer, `String`, or an enum with `#[derive(Segment)]`), or a tuple of two to six of them"
)]
pub trait Params: sealed::Sealed + Sized {
    /// The number of `{name}` segments the path has.
    #[doc(hidden)]
    const COUNT: usize;

    /// Reads the values from the percent-decoded segments, in path order.
    #[doc(hidden)]
    fn from_segments(segments: &[&str]) -> Option<Self>;

    /// Writes the values in path order, before percent-encoding.
    #[doc(hidden)]
    fn segments(&self) -> Vec<String>;

    /// The arguments of a route with these parameters and search parameters `Q`.
    #[doc(hidden)]
    type With<Q>;

    #[doc(hidden)]
    fn join<Q>(self, search: Q) -> Self::With<Q>;

    #[doc(hidden)]
    fn split<Q>(args: Self::With<Q>) -> (Self, Q);
}

impl Params for () {
    const COUNT: usize = 0;

    fn from_segments(_: &[&str]) -> Option<Self> {
        Some(())
    }

    fn segments(&self) -> Vec<String> {
        Vec::new()
    }

    type With<Q> = Q;

    fn join<Q>(self, search: Q) -> Q {
        search
    }

    fn split<Q>(args: Q) -> (Self, Q) {
        ((), args)
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

    fn segments(&self) -> Vec<String> {
        vec![self.write()]
    }

    type With<Q> = (Self, Q);

    fn join<Q>(self, search: Q) -> (Self, Q) {
        (self, search)
    }

    fn split<Q>(args: (Self, Q)) -> (Self, Q) {
        args
    }
}

/// Implements `Params` and the seal for a tuple of `Segment`s.
macro_rules! tuple_params {
    ($count:literal: $($ty:ident $var:ident $index:tt),+) => {
        impl<$($ty: Segment),+> sealed::Sealed for ($($ty,)+) {}

        impl<$($ty: Segment),+> Params for ($($ty,)+) {
            const COUNT: usize = $count;

            fn from_segments(segments: &[&str]) -> Option<Self> {
                match segments {
                    [$($var),+] => Some(($($ty::parse($var)?,)+)),
                    _ => None,
                }
            }

            fn segments(&self) -> Vec<String> {
                vec![$(self.$index.write()),+]
            }

            type With<Q> = (Self, Q);

            fn join<Q>(self, search: Q) -> (Self, Q) {
                (self, search)
            }

            fn split<Q>(args: (Self, Q)) -> (Self, Q) {
                args
            }
        }
    };
}

tuple_params!(2: A a 0, B b 1);
tuple_params!(3: A a 0, B b 1, C c 2);
tuple_params!(4: A a 0, B b 1, C c 2, D d 3);
tuple_params!(5: A a 0, B b 1, C c 2, D d 3, E e 4);
tuple_params!(6: A a 0, B b 1, C c 2, D d 3, E e 4, F f 5);

/// The search parameters of a [`Route`]: `()`, or a struct with `#[derive(Signals)]`.
///
/// A struct's fields are written into the query string in the order of
/// their names, as `?name=value&...`, and a field that is `None` is left
/// out. A value is written as JavaScript's `String` writes it, so a field is
/// a string, a number, a `bool`, or an `Option` of one. axum's `Query`
/// reads the struct back on the server and answers 400 where it cannot.
///
/// The server and the browser compare field names by their UTF-8 bytes and
/// UTF-16 units, which agree for ASCII names. A `#[serde(rename)]` to a name
/// outside ASCII can put two fields in a different order on the two sides.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be the search parameters of a route",
    note = "a route's search parameters are `()` or a struct with `#[derive(Signals)]`"
)]
pub trait Search: sealed::SealedSearch + Sized {
    /// Whether the route has a query string.
    #[doc(hidden)]
    const TYPED: bool;

    /// What a route with parameters `P` and these search parameters takes and its handler receives.
    #[doc(hidden)]
    type Args<P: Params>;

    #[doc(hidden)]
    fn join<P: Params>(params: P, search: Self) -> Self::Args<P>;

    #[doc(hidden)]
    fn split<P: Params>(args: Self::Args<P>) -> (P, Self);

    /// The value of a route without a query string, which needs no parsing.
    #[doc(hidden)]
    fn unit() -> Option<Self>;

    /// The query string, with its `?`, or empty.
    #[doc(hidden)]
    fn query(&self) -> String;
}

impl Search for () {
    const TYPED: bool = false;
    type Args<P: Params> = P;

    fn join<P: Params>(params: P, (): ()) -> P {
        params
    }

    fn split<P: Params>(args: P) -> (P, ()) {
        (args, ())
    }

    fn unit() -> Option<Self> {
        Some(())
    }

    fn query(&self) -> String {
        String::new()
    }
}

impl<Q: Signals> Search for Q {
    const TYPED: bool = true;
    type Args<P: Params> = P::With<Q>;

    fn join<P: Params>(params: P, search: Q) -> P::With<Q> {
        params.join(search)
    }

    fn split<P: Params>(args: P::With<Q>) -> (P, Q) {
        P::split(args)
    }

    fn unit() -> Option<Self> {
        None
    }

    fn query(&self) -> String {
        query_string(&serde_json::to_value(self).expect("a Signals struct serializes to JSON"))
    }
}

/// Parameters that start with the parameters `P` of a parent [`Route`].
///
/// A page declared with `in PARENT` in [`routes!`](crate::routes) requires
/// it, so that a child's parameters repeat its parent's in the same order and
/// may add more after them.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not start with `{P}`, the parameters of the parent route",
    note = "a child route's parameters repeat its parent's, in order, and may add more after them"
)]
pub trait Extends<P>: Params {}

/// Implements `Extends` for the tuple `$tuple` of the `Segment`s `$ty`, once per prefix.
macro_rules! extends {
    ($tuple:ty; $generics:tt => $($prefix:ty),+) => {
        $(extends!(@one $tuple; $generics; $prefix);)+
    };
    (@one $tuple:ty; [$($ty:ident),+]; $prefix:ty) => {
        impl<$($ty: Segment),+> Extends<$prefix> for $tuple {}
    };
}

impl Extends<()> for () {}
impl<A: Segment> Extends<()> for A {}
impl<A: Segment> Extends<A> for A {}
extends!((A, B); [A, B] => (), A, (A, B));
extends!((A, B, C); [A, B, C] => (), A, (A, B), (A, B, C));
extends!((A, B, C, D); [A, B, C, D] => (), A, (A, B), (A, B, C), (A, B, C, D));
extends!((A, B, C, D, E); [A, B, C, D, E] => (), A, (A, B), (A, B, C), (A, B, C, D), (A, B, C, D, E));
extends!((A, B, C, D, E, F); [A, B, C, D, E, F] => (), A, (A, B), (A, B, C), (A, B, C, D), (A, B, C, D, E), (A, B, C, D, E, F));

pub(crate) mod sealed {
    pub trait SealedSearch {}
    impl SealedSearch for () {}
    impl<Q: crate::signal::Signals> SealedSearch for Q {}

    pub trait Sealed {}
    impl Sealed for () {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for String {}
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
    fn a_url_matches_the_fixed_link() {
        const POST: Route<(String, i32)> = Route::new("/blog/{slug}/comments/{id}");
        let link: Link<()> = POST.link_with(("a/b é", -4));
        assert_eq!(
            link.into_parts().ok(),
            Some(POST.url(("a/b é".to_owned(), -4)).to_string())
        );
    }

    #[test]
    fn writes_the_query_string_in_name_order_without_nulls() {
        let search = json!({ "q": "a b&c", "page": 2, "open": true, "tag": null, "x": 1.5 });
        assert_eq!(query_string(&search), "?open=true&page=2&q=a%20b%26c&x=1.5");
        assert_eq!(query_string(&json!({ "tag": null })), "");
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
