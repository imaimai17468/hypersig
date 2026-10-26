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
/// routes and its pages link to [`SafeUrl`](crate::SafeUrl) values.
///
/// A parameter is a whole segment written `{name}`, as in axum. `P` is `()`
/// for a path without one, a [`Segment`] type for one, and a tuple for two to
/// six, in the order they appear. The name inside the braces is for the
/// reader and axum, and the position decides which value fills it. A last
/// segment written `{*name}` takes the rest of the path, `/`s included, and
/// its value is a [`Rest`].
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
    base: Base,
    path: &'static str,
    marker: PhantomData<fn(P, Q)>,
}

/// The path prefix an application is served under, such as `/app` behind a proxy.
///
/// Name it once, as a `const`, after `under` on the first line of
/// [`routes!`](crate::routes). Links, [`Url`]s, the calls to the block's
/// actions, streams, and uploads, and the path a page loads the runtime
/// from then start with it, and the router still registers each path
/// without it, so the application runs either inside
/// `Router::nest(BASE.as_str(), ...)` or behind a proxy that strips the
/// prefix before the request arrives.
///
/// It is a constant because the routes and actions it prefixes are, so a
/// prefix read from the environment at run time cannot reach them. Choose it
/// when the program is built, with `option_env!` where a build sets it.
///
/// ```
/// use axum::Router;
/// use hypersig::html::a;
/// use hypersig::{Action, Base, Page, Patches, RouterExt, Route, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Shop {
///     count: i32,
/// }
///
/// const APP: Base = Base::new("/app");
///
/// hypersig::routes! {
///     fn pages(Router<()>) under APP;
///     const HOME: Route = "/" => home;
///     const ITEM: Route<u32> = "/items/{id}" => item;
///     action ADD: Action<Shop> = post "/add" => |(), ()| async { Patches::from(Shop::count().patch(1)) };
/// }
///
/// async fn home((): (), (): ()) -> Page<Shop> {
///     Page::new("Shop", Shop { count: 0 }).child(a().href(ITEM.link_with(&3_u32)).text("Tea"))
/// }
///
/// async fn item((): (), id: u32) -> Page<Shop> {
///     Page::new("Item", Shop { count: 0 }).child(a().href(HOME.link()).text(format!("Back from {id}")))
/// }
///
/// assert_eq!(ITEM.url(3).as_str(), "/app/items/3");
/// assert_eq!(HOME.url(()).as_str(), "/app");
/// assert_eq!(ITEM.path(), "/items/{id}");
/// assert_eq!(ADD.path(), "/add");
///
/// let app = pages(Router::new()).hypersig_runtime();
/// let _served: Router = Router::new().nest(APP.as_str(), app);
/// ```
///
/// # What stays unchecked
///
/// The compiler does not check that an action, a stream, or an upload a
/// page calls comes from a `routes!` block under the same base as the
/// page's. One from a block without `under APP` calls its path without the
/// prefix, which a proxy answers with its own 404, and which the router
/// answers with 404 inside `nest`.
/// A page served by a router nested at a path other than its base answers
/// with a 500 that names both. The paths given to
/// [`Page::stylesheet`](crate::Page::stylesheet),
/// [`Page::icon`](crate::Page::icon), and
/// [`Page::runtime_src`](crate::Page::runtime_src) are written as given.
/// A [`Url`] already starts with the base, so a canonical URL takes an
/// [`Origin`](crate::Origin) without [`Origin::under`](crate::Origin::under).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base(&'static str);

impl Base {
    /// No prefix: the application is served at the root of its host.
    pub const ROOT: Self = Self("");

    /// The prefix `path`, which starts with `/` and does not end with one.
    ///
    /// # Panics
    ///
    /// Panics if `path` does not start with `/`, ends with `/`, is `/`
    /// alone, or holds `{`, `}`, `?`, `#`, `%`, or `//`. In a `const`, each
    /// is a compile error.
    #[must_use]
    pub const fn new(path: &'static str) -> Self {
        let bytes = path.as_bytes();
        assert!(
            bytes.len() > 1 && bytes[0] == b'/' && bytes[bytes.len() - 1] != b'/',
            "a base path starts with `/`, does not end with `/`, and is not `/` alone"
        );
        let mut i = 0;
        while i < bytes.len() {
            assert!(
                !matches!(bytes[i], b'{' | b'}' | b'?' | b'#' | b'%'),
                "a base path is literal segments, without `{{`, `}}`, `?`, `#`, or `%`"
            );
            assert!(
                !(bytes[i] == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/'),
                "a base path has no empty segment"
            );
            i += 1;
        }
        Self(path)
    }

    /// The prefix as text, empty for [`Base::ROOT`].
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// `path` under this prefix, as axum's `nest` serves it: the prefix
    /// alone for `/`.
    pub(crate) fn join(self, path: &str) -> String {
        match (self.0, path) {
            ("", path) => path.to_owned(),
            (base, "/") => base.to_owned(),
            (base, path) => format!("{base}{path}"),
        }
    }
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
        assert!(
            P::REST_COUNT == P::REST_LAST as usize,
            "only the last value of a route's parameters can be `Rest`"
        );
        assert!(
            ends_in_wildcard(path) == P::REST_LAST,
            "a `{{*name}}` segment and a last parameter of type `Rest` go together"
        );
        Self {
            base: Base::ROOT,
            path,
            marker: PhantomData,
        }
    }

    /// The same page, served under `base`.
    #[must_use]
    pub(crate) const fn under(self, base: Base) -> Self {
        Self { base, ..self }
    }

    /// The path as axum matches it, with each parameter still written `{name}`.
    ///
    /// It leaves out the route's [`Base`], because the router registers the
    /// path without it.
    #[must_use]
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// The prefix the route's links and URLs start with.
    #[cfg(feature = "axum")]
    pub(crate) fn base(&self) -> Base {
        self.base
    }

    /// The literal parts of the path around its parameters, the first one under the base.
    fn parts(&self) -> Vec<String> {
        let mut parts: Vec<String> = literal_parts(self.path)
            .into_iter()
            .map(str::to_owned)
            .collect();
        parts[0] = if parts.len() == 1 {
            self.base.join(&parts[0])
        } else {
            format!("{}{}", self.base.as_str(), parts[0])
        };
        parts
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
        Route::new(path).under(self.base)
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
                let (params, search) = split_pair(args);
                (params, Some(search))
            }
        };
        let values = match P::COUNT {
            0 => Vec::new(),
            1 => vec![params],
            count => split_tuple(params, count),
        };
        Link::new(&self.parts(), values, search, P::REST_LAST)
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
        Url(fixed_url(
            &self.parts(),
            &params.segments(),
            P::REST_LAST,
            &search.query(),
        ))
    }

    /// The parameters this route's path takes from the start of `path`, percent-decoded.
    #[cfg(feature = "axum")]
    pub(crate) fn prefix_of(&self, path: &str) -> Option<P> {
        let wanted: Vec<&str> = match self.path {
            "/" => Vec::new(),
            route => route[1..].split('/').collect(),
        };
        let mut given = path.strip_prefix('/')?.split('/');
        let mut values = Vec::new();
        for part in wanted {
            let segment = given.next()?;
            if part.starts_with("{*") {
                let rest = std::iter::once(segment).chain(given.by_ref());
                values.push(percent_decode(&rest.collect::<Vec<_>>().join("/"))?);
            } else if part.starts_with('{') {
                values.push(percent_decode(segment)?);
            } else if part != segment {
                return None;
            }
        }
        let values: Vec<&str> = values.iter().map(String::as_str).collect();
        P::from_segments(&values)
    }
}

/// Decodes the `%XX` escapes of `text`, or `None` where the bytes are not UTF-8.
///
/// A `%` without two hex digits after it stays as it is.
#[cfg(feature = "axum")]
fn percent_decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if let Some(byte) = percent_escape(bytes, i) {
            out.push(byte);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// The byte that `bytes[at..]` stands for where it starts with `%` and two hex digits.
#[cfg(feature = "axum")]
pub(crate) fn percent_escape(bytes: &[u8], at: usize) -> Option<u8> {
    let &[b'%', high, low] = bytes.get(at..at + 3)? else {
        return None;
    };
    let digit = |byte: u8| char::from(byte).to_digit(16);
    let value = digit(high)? * 16 + digit(low)?;
    Some(u8::try_from(value).expect("two hex digits make at most 0xFF"))
}

impl Route<(), ()> {
    /// A link to this page, which has no parameters.
    #[must_use]
    pub fn link<S>(&self) -> Link<S> {
        Link::new(&self.parts(), Vec::new(), None, false)
    }
}

/// Declares the pages and actions of an application and the function that serves them.
///
/// The first line names the function and the router state its handlers
/// take. Each line after it declares one constant and names the handler that
/// serves it after `=>`, so nothing the browser can reach is declared
/// without a handler. The generated function registers every line with its
/// handler. A line is one of:
///
/// | Line | Constant | Served on |
/// |---|---|---|
/// | `const NAME: Route<P, Q> = "/path" => handler;` | a [`Route`] | `GET` |
/// | `action NAME: Action<S, A> = post "/path" => handler;` | an [`Action`](crate::Action) | `POST`, or `GET` with `get` in place of `post` |
/// | `stream NAME: StreamAction<S, A> = "/path" => handler;` | a [`StreamAction`](crate::StreamAction) | `GET` |
/// | `upload NAME: Upload<S, A> = "/path" => handler;` | an [`Upload`](crate::Upload) | `POST` |
///
/// Add `in PARENT` after the path of a route to declare a page under
/// another page of an earlier line: its path continues the parent's and its
/// parameters start with the parent's. Write `stream` before the path of an
/// upload to hand its handler the files in chunks, and `max_bytes N` after
/// the path to accept a body of at most `N` bytes in place of
/// [`Upload::DEFAULT_MAX_BYTES`](crate::Upload::DEFAULT_MAX_BYTES).
///
/// Write `under BASE` after `Router<STATE>` to serve every line of the
/// block under the path prefix that `BASE` names, which every link, URL,
/// and call to the block's constants then starts with. `BASE` is the name
/// of a [`Base`] constant in scope, without a path before it.
///
/// ```
/// use axum::Router;
/// use hypersig::html::{a, button, h1};
/// use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Profile {
///     name: String,
/// }
///
/// hypersig::routes! {
///     /// Serves every page and action of the application.
///     fn pages(Router<()>);
///     const HOME: Route = "/" => home;
///     const ORG: Route<String> = "/orgs/{org}" => org;
///     const MEMBER: Route<(String, i32)> = "/orgs/{org}/members/{id}" in ORG => member;
///     action RENAME: Action<Profile, String> = post "/rename" => rename;
/// }
///
/// async fn home((): (), (): ()) -> Page<Profile> {
///     let signals = Profile { name: "Ada".to_owned() };
///     Page::new("Home", signals)
///         .child(a().href(MEMBER.link_with(("acme", 1))).text("Ada"))
///         .child(button().on_click(RENAME.call_with("Grace")).text("Rename"))
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
/// async fn rename((): (), name: String) -> Patches<Profile> {
///     Profile::name().patch(name).into()
/// }
///
/// let _app: Router = pages(Router::new()).hypersig_runtime();
/// ```
///
/// # Handlers
///
/// Each handler takes the router state and its arguments, and may take one
/// `FromRequestParts` extractor between them, as [`Handler`](crate::Handler)
/// says. A page's handler receives the route's arguments, already parsed,
/// and answers with a [`Page`](crate::Page), an `Option<Page>`, or a
/// [`PageResponse`](crate::PageResponse). A path whose segments do not parse
/// into the parameter types, or a handler that answers `None`, gets a 404,
/// and a query string that does not deserialize into the search parameters
/// gets a 400.
///
/// An action's handler receives the arguments the call passed, already
/// deserialized, and answers with anything that converts into an
/// [`ActionResponse`](crate::ActionResponse), as
/// [`IntoActionResponse`](crate::IntoActionResponse) lists. A stream's
/// handler is a plain `fn` that returns a stream of [`Patches`](crate::Patches),
/// as [`PatchStream`](crate::PatchStream) says, and the response stays open
/// until the stream ends or the browser leaves the page. An upload's handler
/// receives an [`Uploaded`](crate::Uploaded), or an
/// [`UploadStream`](crate::UploadStream) with `stream`, and answers as an
/// action's does.
///
/// A [`RequestError`](crate::RequestError) refuses a call before its
/// handler runs: a request from another site, a `POST` whose body is not
/// JSON, a `GET` without the `hypersig-call` header the runtime adds,
/// or arguments that do not deserialize. An upload is also refused without
/// the header the runtime adds, with a body that is not
/// `multipart/form-data`, one longer than its limit, or one whose first part
/// is not the arguments, and parts other than the arguments and the files
/// are skipped. A handler's extractor runs after the header checks and
/// before the body is read, so one that refuses a visitor who is not signed
/// in answers before any file arrives.
///
/// The arguments come from the visitor's browser, which can send any value
/// of `A`, so an action's handler checks that the visitor may act on them.
/// Read the visitor from the request with an extractor, as the `auth`
/// example does.
///
/// # A page for a request that reaches no page
///
/// Write `else HANDLER` after the first line to show a page for each such
/// request, and for a path no route matches. The handler receives an
/// [`Unserved`](crate::Unserved), may take an extractor as a page's handler
/// does, and its page is sent with the status the `Unserved` names. The
/// generated function registers it as the router's fallback, so a later
/// `Router::fallback` replaces it for paths no route matches, and merging
/// the router with another that has a fallback panics, as axum does for any
/// two fallbacks. A call from the runtime to a path no route matches gets a
/// bare 404, because the runtime cannot apply a page.
///
/// ```
/// use axum::Router;
/// use hypersig::html::p;
/// use hypersig::{Page, Route, Signals, Unserved};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Shop {}
///
/// hypersig::routes! {
///     fn pages(Router<()>) else missing;
///     const ORDER: Route<u32> = "/orders/{id}" => order;
/// }
///
/// async fn order((): (), id: u32) -> Option<Page<Shop>> {
///     (id < 100).then(|| Page::new(format!("Order {id}"), Shop {}))
/// }
///
/// async fn missing((): (), unserved: Unserved) -> Page<Shop> {
///     Page::new("Not found", Shop {}).child(p().text(format!("{unserved}.")))
/// }
///
/// let _app: Router = pages(Router::new());
/// ```
///
/// # Parts
///
/// The actions, streams, and uploads of a [`Part`](crate::Part) are lines of
/// the application's block like any other, and the part's `render` calls
/// the constants they declare. Each is then registered once, and its one
/// handler serves every page that embeds the part, under the field the call
/// came from.
///
/// # What stays unchecked
///
/// The macro guarantees that each line has a handler, and nothing checks
/// that the generated function is called. A function that is never called
/// leaves every page of its block answering 404, and every action, stream,
/// and upload of it answering 404 to the runtime, which writes the status
/// to the console and to the signal a call names with
/// [`Call::error`](crate::Call::error), so a call without one changes
/// nothing on the page. rustc's `dead_code` lint does not report the
/// function, because it comes from the macro, but it does report a handler
/// `fn` that only the function names as never used. A closure written as
/// the handler gets no such warning. Each `routes!` block is its own
/// function, so declare every page and action of the application in one
/// block: the application then calls one function, and a missing call shows
/// on its first page.
///
/// A page can call an action, a stream, or an upload of another block,
/// whose [`Base`] may differ from the base of the page's own block. Such a
/// call goes to the other block's prefix, which answers 404 unless that
/// block is served there too. One block per application keeps every line
/// under one base.
///
/// A router passed to `Router::nest` serves its pages under a longer path
/// than their links point to, unless it is nested at the [`Base`] the first
/// line names after `under`. Such a page answers every request with a 500
/// that names the two paths.
///
/// A parent after `in` is a page itself. A path prefix that has no page of
/// its own cannot be a parent.
///
/// # Compile errors
///
/// A line without `=> handler`, an `action` line without `post` or `get`,
/// a line whose word before the name is not `const`, `action`, `stream`, or
/// `upload`, an action, a stream, or an upload made outside the macro, a
/// path that does not start with `/`, a `{` or `}` that does not enclose a
/// whole named segment, a path whose segments differ in number from the
/// route's parameters, a `{*name}` segment that is not last or whose value
/// is not a [`Rest`], a child whose path or parameters do not continue its
/// parent's, and a handler whose arguments or answer do not fit its line.
///
/// This macro needs the `axum` feature.
#[cfg(feature = "axum")]
#[macro_export]
macro_rules! routes {
    (@decl [$base:expr] $(#[$attr:meta])* $vis:vis const $name:ident: $ty:ty = $path:literal) => {
        $(#[$attr])*
        $vis const $name: $ty = $crate::_private::route($base, $path);
    };
    (
        @decl [$base:expr] $(#[$attr:meta])* $vis:vis const $name:ident: $ty:ty = $path:literal
        in $parent:ident
    ) => {
        $(#[$attr])*
        $vis const $name: $ty = $crate::_private::child(&$parent, $path);
    };
    (@decl [$base:expr] $(#[$attr:meta])* $vis:vis action $name:ident: $ty:ty = post $path:literal) => {
        $(#[$attr])*
        $vis const $name: $ty = $crate::_private::action($base, $crate::Method::Post, $path);
    };
    (@decl [$base:expr] $(#[$attr:meta])* $vis:vis action $name:ident: $ty:ty = get $path:literal) => {
        $(#[$attr])*
        $vis const $name: $ty = $crate::_private::action($base, $crate::Method::Get, $path);
    };
    (@decl [$base:expr] $(#[$attr:meta])* $vis:vis stream $name:ident: $ty:ty = $path:literal) => {
        $(#[$attr])*
        $vis const $name: $ty = $crate::_private::stream($base, $path);
    };
    (
        @decl [$base:expr] $(#[$attr:meta])* $vis:vis upload $name:ident: $ty:ty =
        $(stream)? $path:literal
    ) => {
        $(#[$attr])*
        $vis const $name: $ty =
            $crate::_private::upload($base, $path, $crate::_private::UPLOAD_MAX_BYTES);
    };
    (
        @decl [$base:expr] $(#[$attr:meta])* $vis:vis upload $name:ident: $ty:ty =
        $(stream)? $path:literal max_bytes $bytes:expr
    ) => {
        $(#[$attr])*
        $vis const $name: $ty = $crate::_private::upload($base, $path, $bytes);
    };
    (@decl [$base:expr] $($line:tt)*) => {
        ::core::compile_error!(::core::concat!(
            "this line of `routes!` is not one of `const NAME: Route<..> = \"/path\" => handler;`, ",
            "`const NAME: Route<..> = \"/path\" in PARENT => handler;`, ",
            "`action NAME: Action<..> = post \"/path\" => handler;` (or `get`), ",
            "`stream NAME: StreamAction<..> = \"/path\" => handler;`, ",
            "or `upload NAME: Upload<..> = \"/path\" => handler;` ",
            "(`stream` before the path to read the files in chunks, `max_bytes N` after it to set the limit), not `",
            ::core::stringify!($($line)*),
            "`"
        ));
    };
    (@register $router:ident $unserved:ident const [] $name:ident $handler:expr) => {
        $crate::RouterExt::page(
            $router, $name, $handler, $unserved.clone(),
        )
    };
    (@register $router:ident $unserved:ident action [$($mode:ident)?] $name:ident $handler:expr) => {
        $crate::RouterExt::action($router, $name, $handler)
    };
    (@register $router:ident $unserved:ident stream [] $name:ident $handler:expr) => {
        $crate::RouterExt::stream($router, $name, $handler)
    };
    (@register $router:ident $unserved:ident upload [] $name:ident $handler:expr) => {
        $crate::RouterExt::upload($router, $name, $handler)
    };
    (@register $router:ident $unserved:ident upload [stream] $name:ident $handler:expr) => {
        $crate::RouterExt::upload_stream(
            $router, $name, $handler,
        )
    };
    // A line `@decl` refused has already raised its error, so this one adds none.
    (@register $router:ident $unserved:ident $($line:tt)*) => {
        $router
    };
    (
        @main [$base:expr]
        $(#[$fn_attr:meta])*
        $fn_vis:vis fn $fn_name:ident(Router<$state:ty>) $(else $unserved:expr)?;
        $(
            $(#[$attr:meta])*
            $vis:vis $kind:ident $name:ident: $ty:ty = $($mode:ident)? $path:literal
                $(in $parent:ident)? $(max_bytes $bytes:expr)? => $handler:expr;
        )+
    ) => {
        $(
            $crate::routes!(
                @decl [$base] $(#[$attr])* $vis $kind $name: $ty = $($mode)? $path
                $(in $parent)? $(max_bytes $bytes)?
            );
        )+

        $(#[$fn_attr])*
        $fn_vis fn $fn_name(
            router: $crate::_private::Router<$state>,
        ) -> $crate::_private::Router<$state> {
            let unserved: ::core::option::Option<$crate::_private::UnservedPage<$state>> =
                ::core::option::Option::None
                $(.or(::core::option::Option::Some($crate::_private::UnservedPage::new($base, $unserved))))?;
            $(
                let router: $crate::_private::Router<$state> =
                    $crate::routes!(@register router unserved $kind [$($mode)?] $name $handler);
            )+
            match unserved {
                ::core::option::Option::Some(unserved) => unserved.fallback(router),
                ::core::option::Option::None => router,
            }
        }
    };
    (@main $($input:tt)*) => {
        ::core::compile_error!(
            "`routes!` takes `fn NAME(Router<STATE>);`, with `under BASE` and `else HANDLER` before the `;` where wanted, then one line per page, action, stream, and upload, each naming its handler after `=>`: `const NAME: Route<..> = \"/path\" => handler;`, `action NAME: Action<..> = post \"/path\" => handler;`, `stream NAME: StreamAction<..> = \"/path\" => handler;`, or `upload NAME: Upload<..> = \"/path\" => handler;`"
        );
    };
    (
        $(#[$fn_attr:meta])*
        $fn_vis:vis fn $fn_name:ident(Router<$state:ty>) under $base:ident $($rest:tt)*
    ) => {
        $crate::routes!(@main [$base] $(#[$fn_attr])* $fn_vis fn $fn_name(Router<$state>) $($rest)*);
    };
    (
        $(#[$fn_attr:meta])*
        $fn_vis:vis fn $fn_name:ident(Router<$state:ty>) $($rest:tt)*
    ) => {
        $crate::routes!(
            @main [$crate::Base::ROOT] $(#[$fn_attr])* $fn_vis fn $fn_name(Router<$state>) $($rest)*
        );
    };
    ($($input:tt)*) => {
        $crate::routes!(@main $($input)*);
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

/// The two halves of the pair `ast`, read off a literal `tuple` where it is
/// one, and taken from the value in the browser otherwise.
fn split_pair(ast: Value) -> (Value, Value) {
    let ast = match ast {
        Value::Array(items) => match <[Value; 3]>::try_from(items) {
            Ok([tag, first, second]) if tag == "tuple" => return (first, second),
            Ok(items) => Value::Array(items.into()),
            Err(items) => Value::Array(items),
        },
        ast => ast,
    };
    let nth = |index: usize| json!(["nth", ast, ["lit", index]]);
    (nth(0), nth(1))
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
        f.debug_struct("Route")
            .field("base", &self.base.as_str())
            .field("path", &self.path)
            .finish()
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
                let wildcard = i + 1 < bytes.len() && bytes[i + 1] == b'*';
                let start = if wildcard { i + 2 } else { i + 1 };
                i = start;
                while i < bytes.len() && bytes[i] != b'}' {
                    assert!(
                        bytes[i] != b'{' && bytes[i] != b'/' && bytes[i] != b'*',
                        "a route parameter is `{{name}}` or `{{*name}}`, with no `{{`, `/`, or `*` inside the name"
                    );
                    i += 1;
                }
                assert!(i < bytes.len(), "a route parameter's `{{` has no `}}`");
                assert!(i > start, "a route parameter has a name between its braces");
                assert!(
                    i + 1 == bytes.len() || bytes[i + 1] == b'/',
                    "a route parameter is a whole segment, so `}}` ends it"
                );
                assert!(
                    !wildcard || i + 1 == bytes.len(),
                    "a `{{*name}}` parameter is the last segment of the path"
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

/// Whether the last segment of `path` is a `{*name}` parameter.
const fn ends_in_wildcard(path: &str) -> bool {
    let bytes = path.as_bytes();
    let mut i = bytes.len();
    while i > 0 && bytes[i - 1] != b'/' {
        i -= 1;
    }
    i + 1 < bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'*'
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
    /// A link to the path whose literal parts are `parts`, with `values` between them.
    fn new(parts: &[String], values: Vec<Value>, search: Option<Value>, rest_last: bool) -> Self {
        let count = values.len();
        let is_rest = |index: usize| rest_last && index + 1 == count;
        let segments: Option<Vec<String>> = values
            .iter()
            .map(|value| literal(value).map(crate::ssr::js_string))
            .collect();
        let search_literal = search.as_ref().map(literal);
        if let (Some(segments), None | Some(Some(_))) = (segments, search_literal) {
            let query = search_literal
                .flatten()
                .map(query_string)
                .unwrap_or_default();
            return Self(LinkKind::Fixed(fixed_url(
                parts, &segments, rest_last, &query,
            )));
        }
        let mut ast = json!(["lit", parts[0]]);
        for (index, (value, part)) in values.into_iter().zip(&parts[1..]).enumerate() {
            let op = if is_rest(index) { "uri_path" } else { "uri" };
            ast = json!(["concat", ast, [op, value]]);
            if !part.is_empty() {
                ast = json!(["concat", ast, ["lit", part]]);
            }
        }
        if let Some(search) = search {
            ast = json!(["concat", ast, ["query", search]]);
        }
        // Only a path that starts with a parameter can start with `//`, when
        // that parameter is empty, and the browser checks it as `on_this_host`
        // does.
        if parts[0] == "/" {
            ast = json!([
                "if",
                ["starts", ast, ["lit", "//"]],
                ["concat", ["lit", "/."], ast],
                ast
            ]);
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
        Stmt::new(json!(["navigate", self.into_json()]))
    }

    /// A statement that shows the linked URL in the address bar as a new history entry.
    ///
    /// The page stays loaded, with its signals, its focus, and its scroll
    /// position, so the statement goes with the ones that bring the page in
    /// line with the URL, such as a call to the action that loads the rows a
    /// search names. The visitor's Back and Forward buttons move between the
    /// entries, and the browser then loads the page at the entry's URL from
    /// the server. A URL equal to the current one adds no entry.
    ///
    /// The type system does not check that the linked route serves the page
    /// the statement runs on. A link to another page's route shows that URL
    /// over this page's content, and reloading it loads the other page.
    ///
    /// ```
    /// use hypersig::html::input;
    /// use hypersig::{Action, Expr, Route, Search, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Search, Serialize, Deserialize, Default)]
    /// struct Filter {
    ///     q: String,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Fruit {
    ///     query: String,
    /// }
    ///
    /// # async fn fruit((): (), _: Filter) -> Option<hypersig::Page<Fruit>> { None }
    /// # async fn search((): (), _: String) -> hypersig::Patches<Fruit> { hypersig::Patches::new() }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const FRUIT: Route<(), Filter> = "/fruit" => fruit;
    ///     action SEARCH: Action<Fruit, String> = post "/search" => search;
    /// }
    ///
    /// let query = Fruit::query();
    /// let filter = Expr::lit(&Filter::default()).with(Filter::q(), query);
    /// let _field = input()
    ///     .bind(query)
    ///     .on_input(SEARCH.call_with(query).then(FRUIT.link_with(filter).push_url()));
    /// # let _ = pages;
    /// ```
    #[must_use]
    pub fn push_url(self) -> Stmt<S> {
        Stmt::new(json!(["history", "push", self.into_json()]))
    }

    /// A statement that shows the linked URL in the address bar in place of the current one.
    ///
    /// It is [`Link::push_url`] without the new history entry, for a change
    /// the Back button should skip, such as each letter typed into a search
    /// field.
    #[must_use]
    pub fn replace_url(self) -> Stmt<S> {
        Stmt::new(json!(["history", "replace", self.into_json()]))
    }

    /// The URL as an expression: a literal for a fixed link.
    fn into_json(self) -> Value {
        match self.0 {
            LinkKind::Fixed(url) => json!(["lit", url]),
            LinkKind::Bound(expr) => expr.into_json(),
        }
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

/// The value of the literal `ast`, or `None` where `ast` is computed in the browser.
fn literal(ast: &Value) -> Option<&Value> {
    match ast.as_array().map(Vec::as_slice) {
        Some([op, literal]) if op == "lit" => Some(literal),
        _ => None,
    }
}

/// The URL whose path is the literal `parts` with each of `segments` percent-encoded between them, followed by `query`.
fn fixed_url(parts: &[String], segments: &[String], rest_last: bool, query: &str) -> String {
    let mut url = parts[0].clone();
    for (index, (segment, part)) in segments.iter().zip(&parts[1..]).enumerate() {
        url.push_str(&encode_segment(
            segment,
            rest_last && index + 1 == segments.len(),
        ));
        url.push_str(part);
    }
    url.push_str(query);
    on_this_host(url)
}

/// `url`, kept on the site's own host.
///
/// An empty first segment makes a URL such as `//evil.example`, which names
/// another host, so `/.` goes in front of it. The browser drops the `.`
/// segment and stays on the page's host.
fn on_this_host(url: String) -> String {
    if url.starts_with("//") {
        format!("/.{url}")
    } else {
        url
    }
}

/// Percent-encodes one segment's value, keeping the `/`s of a [`Rest`] value.
fn encode_segment(text: &str, rest: bool) -> String {
    if rest {
        encode_uri_path(text)
    } else {
        encode_uri_component(text)
    }
}

/// Percent-encodes each `/`-separated part of `text` as `encodeURIComponent` does, keeping the `/`s.
///
/// A `/` at the start of `text` or right after another one is written
/// `%2F`, because `//` at the start of a URL names another host, so a value
/// such as `/evil.example` under the route `/{*path}` would otherwise send the
/// browser to `//evil.example`.
pub(crate) fn encode_uri_path(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut previous: Option<&str> = None;
    for segment in text.split('/') {
        match previous {
            Some("") => out.push_str("%2F"),
            Some(_) => out.push('/'),
            None => {}
        }
        out.push_str(&encode_uri_component(segment));
        previous = Some(segment);
    }
    out
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
/// A `null` field is left out, a list writes its name once per item in item
/// order, and each name and value is percent-encoded as `encodeURIComponent`
/// does, after the value is written as `String` would.
pub(crate) fn query_string(value: &Value) -> String {
    let Value::Object(fields) = value else {
        return String::new();
    };
    let mut pairs: Vec<(&String, &Value)> = fields.iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(b.0));
    let mut out = String::new();
    for (name, value) in pairs {
        let values = match value {
            Value::Array(items) => items.as_slice(),
            Value::Null => &[],
            one => std::slice::from_ref(one),
        };
        for value in values {
            out.push(if out.is_empty() { '?' } else { '&' });
            out.push_str(&encode_uri_component(name));
            out.push('=');
            out.push_str(&encode_uri_component(&crate::ssr::js_string(value)));
        }
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
/// fields. The segment is the string serde writes for the variant, so a link
/// that reads the enum from a signal and the server write the same URL. The
/// derive follows `#[serde(rename_all = "...")]` on the enum and
/// `#[serde(rename = "...")]` on a variant, and rejects any other serde
/// attribute, which could make serde write a variant differently. The
/// derive also implements [`Variants`], which lists the variants for
/// [`label`](crate::Expr::label).
///
/// ```
/// use hypersig::{Page, Route, Segment, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Segment, Serialize, Deserialize)]
/// #[serde(rename_all = "kebab-case")]
/// enum Tab {
///     Overview,
///     TeamSettings,
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
/// assert_eq!(TAB.url(("red".to_owned(), Tab::TeamSettings)).as_str(), "/teams/red/team-settings");
/// # let _ = pages;
/// ```
pub trait Segment: sealed::Sealed + Sized {
    /// Reads the value from a percent-decoded segment, or `None` if it is not one.
    #[doc(hidden)]
    fn parse(segment: &str) -> Option<Self>;

    /// Writes the value as the browser's `String` would, before percent-encoding.
    #[doc(hidden)]
    fn write(&self) -> String;

    /// Whether the value fills a `{*name}` segment, which may hold `/`.
    #[doc(hidden)]
    const REST: bool = false;
}

/// An enum with `#[derive(Segment)]`, whose variants the derive lists.
///
/// [`label`](crate::Expr::label) calls a function once per variant, so a
/// `match` in that function names a text for each one, and the compiler
/// rejects a `match` that leaves a variant out. Only the derive implements
/// it, from the enum's own variants.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not list its variants",
    note = "`label` takes an enum with `#[derive(Segment)]`, which lists every variant"
)]
pub trait Variants: Segment + 'static {
    /// Every variant, in declaration order.
    #[doc(hidden)]
    const VARIANTS: &'static [Self];
}

/// The rest of a path, from a `{*name}` segment at its end, which may hold `/`.
///
/// It is the last value of a route's parameters, and only there, as axum's
/// wildcard is the last segment. Each `/`-separated part is percent-encoded
/// in a link, and the `/`s between them stay as they are. axum does not
/// match a wildcard against an empty rest, so `"/files/"` answers 404.
///
/// ```
/// use hypersig::{Page, Rest, Route, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct File {}
///
/// # async fn file((): (), _: Rest) -> Option<Page<File>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const FILE: Route<Rest> = "/files/{*path}" => file;
/// }
///
/// assert_eq!(FILE.url(Rest::new("docs/a b.md")).as_str(), "/files/docs/a%20b.md");
/// # let _ = pages;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Rest(String);

impl Rest {
    /// The rest of a path, with its `/`s and before percent-encoding.
    #[must_use]
    pub fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }

    /// The rest of the path as text, percent-decoded.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Rest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Segment for Rest {
    const REST: bool = true;

    fn parse(segment: &str) -> Option<Self> {
        Some(Self(segment.to_owned()))
    }

    fn write(&self) -> String {
        self.0.clone()
    }
}

macro_rules! integer_segment {
    ($($ty:ty),*) => {$(
        impl Segment for $ty {
            // A page whose signals hold an integer the browser would round
            // does not render, so the URL of one names no page.
            fn parse(segment: &str) -> Option<Self> {
                let v: $ty = segment.parse().ok()?;
                crate::exact::holds_i128(i128::from(v)).then_some(v)
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

    /// Whether the last value is a [`Rest`].
    #[doc(hidden)]
    const REST_LAST: bool;

    /// How many of the values are a [`Rest`].
    #[doc(hidden)]
    const REST_COUNT: usize;

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
    const REST_LAST: bool = false;
    const REST_COUNT: usize = 0;

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
    const REST_LAST: bool = T::REST;
    const REST_COUNT: usize = T::REST as usize;

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
    ($count:literal: $($ty:ident $var:ident $index:tt),+; last $last:ident) => {
        impl<$($ty: Segment),+> sealed::Sealed for ($($ty,)+) {}

        impl<$($ty: Segment),+> Params for ($($ty,)+) {
            const COUNT: usize = $count;
            const REST_LAST: bool = $last::REST;
            const REST_COUNT: usize = 0 $(+ $ty::REST as usize)+;

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

tuple_params!(2: A a 0, B b 1; last B);
tuple_params!(3: A a 0, B b 1, C c 2; last C);
tuple_params!(4: A a 0, B b 1, C c 2, D d 3; last D);
tuple_params!(5: A a 0, B b 1, C c 2, D d 3, E e 4; last E);
tuple_params!(6: A a 0, B b 1, C c 2, D d 3, E e 4, F f 5; last F);

/// The search parameters of a [`Route`]: `()`, or a struct with `#[derive(Signals, Search)]`.
///
/// A struct's fields are written into the query string in the order of
/// their names, as `?name=value&...`. A field that is `None` is left out,
/// and a `Vec` field writes its name once per item, as
/// `?color=red&color=blue`, and nothing when it is empty. A value is written
/// as JavaScript's `String` writes it. `#[derive(Search)]` requires each
/// field's type to implement [`SearchValue`], so a struct in a field, or a
/// list of lists, is a compile error. The server reads the struct back,
/// with a key that appears more than once read into a `Vec` field, and
/// answers 400 where it cannot, such as for a value a field's `Deserialize`
/// rejects or a key repeated for a field that is not a `Vec`.
///
/// ```
/// use hypersig::{Search, Segment, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Segment, Serialize, Deserialize)]
/// #[serde(rename_all = "lowercase")]
/// enum Color {
///     Red,
///     Blue,
/// }
///
/// #[derive(Signals, Search, Serialize, Deserialize)]
/// struct Filter {
///     q: String,
///     page: Option<i32>,
///     color: Vec<Color>,
/// }
/// ```
///
/// The server and the browser compare field names by their UTF-8 bytes and
/// UTF-16 units, which agree for ASCII names. A `#[serde(rename)]` to a name
/// outside ASCII can put two fields in a different order on the two sides.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be the search parameters of a route",
    note = "a route's search parameters are `()` or a struct with `#[derive(Signals, Search)]`"
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

impl<Q: SearchFields> Search for Q {
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

/// A struct whose fields are each a [`SearchValue`], implemented by `#[derive(Search)]`.
#[doc(hidden)]
pub trait SearchFields: Signals {}

/// A [`Search`] field type: a [`SearchScalar`], or an `Option` or `Vec` of one.
///
/// The query string leaves out an `Option` that is `None`, and writes the
/// key of a `Vec` once per item. A list of lists, or an `Option` of a list,
/// has no form in the query string, so neither implements it.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be a field of search parameters",
    note = "a search field is a string, a number, a `bool`, an enum with `#[derive(Segment)]`, or an `Option` or a `Vec` of one, because the query string holds text values under each key"
)]
pub trait SearchValue {}

impl<T: SearchScalar> SearchValue for T {}
impl<T: SearchScalar> SearchValue for Option<T> {}
impl<T: SearchScalar> SearchValue for Vec<T> {}

/// A type that one value of the query string holds.
///
/// It is implemented for every [`Segment`] (the integers, `String`, and an
/// enum with `#[derive(Segment)]`), and for `f64`, `f32`, and `bool`.
///
/// Implement it for your own type only where serde writes the type as one of
/// these, such as a newtype with `#[serde(try_from = "i32", into = "i32")]`
/// that checks its value when the server reads it. The compiler does not
/// check that claim, and a type that serde writes as a list or an object
/// gives links whose query string the server answers with 400.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be one value of a query string",
    note = "a query string value is a string, a number, a `bool`, or an enum with `#[derive(Segment)]`"
)]
pub trait SearchScalar {}

impl<T: Segment> SearchScalar for T {}
impl SearchScalar for f64 {}
impl SearchScalar for f32 {}
impl SearchScalar for bool {}

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
    impl<Q: super::SearchFields> SealedSearch for Q {}

    pub trait Sealed {}
    impl Sealed for () {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for String {}
    impl Sealed for super::Rest {}
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
    fn counts_a_wildcard_at_the_end() {
        assert_eq!(count_params("/files/{*rest}"), 1);
        assert!(ends_in_wildcard("/files/{*rest}"));
        assert!(!ends_in_wildcard("/files/{rest}"));
    }

    #[test]
    #[should_panic(expected = "the last segment of the path")]
    fn rejects_a_wildcard_before_the_end() {
        let _ = count_params("/files/{*rest}/raw");
    }

    #[test]
    fn keeps_the_slashes_of_a_rest() {
        assert_eq!(encode_uri_path("a b/c?/d"), "a%20b/c%3F/d");
    }

    #[test]
    fn keeps_a_url_with_an_empty_first_segment_on_this_host() {
        assert_eq!(
            on_this_host("//evil.example/x".to_owned()),
            "/.//evil.example/x"
        );
        assert_eq!(on_this_host("/a//b".to_owned()), "/a//b");
    }

    #[test]
    fn encodes_a_slash_that_would_start_an_empty_segment() {
        // Under the route `/{*path}`, a leading `/` would make the URL
        // `//evil.example`, which names another host.
        assert_eq!(encode_uri_path("/evil.example"), "%2Fevil.example");
        assert_eq!(encode_uri_path("a//b/"), "a/%2Fb/");
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
    fn an_empty_first_segment_stays_on_this_host() {
        const REPO: Route<(String, String)> = Route::new("/{org}/{repo}");
        let url = REPO.url((String::new(), "evil.example".to_owned()));
        assert_eq!(url.as_str(), "/.//evil.example");
        let link: Link<()> = REPO.link_with(("", "evil.example"));
        assert_eq!(link.into_parts().ok().as_deref(), Some("/.//evil.example"));
    }

    #[test]
    fn a_bound_link_with_an_empty_first_segment_stays_on_this_host() {
        use crate::ssr::{Ctx, evaluate, js_string};
        const REPO: Route<(String, String)> = Route::new("/{org}/{repo}");
        let org: crate::Sig<(), String> = crate::_private::sig("org");
        let link: Link<()> = REPO.link_with((org, "evil.example"));
        let ast = link
            .into_parts()
            .expect_err("a link that reads a signal")
            .into_json();
        let root = json!({ "org": "" });
        let ctx = Ctx {
            root: (&root).into(),
            scope: Vec::new(),
            copy: false,
            vars: Vec::new(),
            rules: None,
        };
        let url = evaluate(&ast, &ctx).expect("the server evaluates a link");
        assert_eq!(js_string(&url), "/.//evil.example");
    }

    #[test]
    fn writes_the_query_string_in_name_order_without_nulls() {
        let search = json!({ "q": "a b&c", "page": 2, "open": true, "tag": null, "x": 1.5 });
        assert_eq!(query_string(&search), "?open=true&page=2&q=a%20b%26c&x=1.5");
        assert_eq!(query_string(&json!({ "tag": null })), "");
        assert_eq!(
            query_string(&json!({ "color": ["red", "a b"], "size": [], "q": "x" })),
            "?color=red&color=a%20b&q=x"
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

    #[test]
    fn an_integer_segment_the_browser_would_round_names_no_page() {
        assert_eq!(<u64>::from_segments(&["9007199254740992"]), Some(1 << 53));
        assert_eq!(
            <i64>::from_segments(&["-9007199254740992"]),
            Some(-(1 << 53))
        );
        assert_eq!(<u64>::from_segments(&["9007199254740993"]), None);
        assert_eq!(<i64>::from_segments(&["-9007199254740993"]), None);
    }
}
