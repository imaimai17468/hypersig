//! The full HTML document a page is served as.

use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;

use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::expr::{IntoExpr, IntoStmt, Stmt};
use crate::head::{OpenGraph, Origin, meta};
use crate::html::{
    HIDE, INLINE, Node, Shortcut, Stylesheet, check_names, escape_into, keys_json, write_attr,
    write_json_attr,
};
use crate::route::{Base, Url};
use crate::signal::{Sig, Signals};
use crate::ssr::{self, Ctx, Val};
use crate::style::{Global, Sheet, Theme};
use crate::tape::{Ref, Tape};

/// A full HTML document for a page whose signals are `S`.
///
/// The document loads the runtime from [`RUNTIME_PATH`](crate::RUNTIME_PATH),
/// under the [`Base`] of the route that serves it, or from
/// [`Page::runtime_src`], and puts the initial signal values on
/// `<body>`. It is rendered with those
/// values already applied: text, visibility, classes, attributes, form
/// values, list items, and conditional elements show before the runtime
/// loads, and the runtime adopts the elements instead of rebuilding them.
pub struct Page<S> {
    title: String,
    bound_title: Option<Value>,
    signals: S,
    stylesheets: Vec<&'static str>,
    scripts: Vec<&'static str>,
    styles: Vec<&'static str>,
    theme: Theme,
    global: Global,
    computed: Vec<(String, Value)>,
    body: Vec<Node<S>>,
    keys: Vec<(Shortcut, Stmt<S>)>,
    /// The path given to [`Page::runtime_src`], if any.
    runtime: Option<&'static str>,
    lang: Option<String>,
    description: Option<String>,
    canonical: Option<String>,
    icon: Option<&'static str>,
    open_graph: Option<OpenGraph>,
    noindex: bool,
    nofollow: bool,
    /// Each `hreflang` with its absolute URL, where `None` is `x-default`.
    alternates: Vec<(Option<String>, String)>,
    /// Each JSON-LD block, serialized with `<` escaped.
    json_ld: Vec<String>,
    nonce: Option<String>,
}

impl<S> fmt::Debug for Page<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Page")
            .field("title", &self.title)
            .finish_non_exhaustive()
    }
}

impl<S: Signals> Page<S> {
    /// Starts a page with its title and initial signal values.
    #[must_use]
    pub fn new(title: impl Into<String>, signals: S) -> Self {
        Self {
            title: title.into(),
            bound_title: None,
            signals,
            stylesheets: Vec::new(),
            scripts: Vec::new(),
            styles: Vec::new(),
            theme: Theme::new(),
            global: Global::new(),
            computed: Vec::new(),
            body: Vec::new(),
            keys: Vec::new(),
            runtime: None,
            lang: None,
            description: None,
            canonical: None,
            icon: None,
            open_graph: None,
            noindex: false,
            nofollow: false,
            alternates: Vec::new(),
            json_ld: Vec::new(),
            nonce: None,
        }
    }

    /// Sets the summary of the page that search results and link previews show.
    ///
    /// ```
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Note {}
    ///
    /// let html = Page::new("Note", Note {}).description("Notes & lists").render();
    /// assert!(html.contains(r#"<meta name="description" content="Notes &amp; lists">"#));
    /// ```
    #[must_use]
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Names `url` on `origin` as the page's one address, for a page that
    /// several URLs serve, such as with and without search parameters.
    ///
    /// ```
    /// use hypersig::{Origin, Page, Route, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Post {}
    ///
    /// # async fn post((): (), _: u32) -> Page<Post> { Page::new("Post", Post {}) }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const POST: Route<u32> = "/posts/{id}" => post;
    /// }
    ///
    /// const SITE: Origin = Origin::new("https://example.com");
    ///
    /// let html = Page::new("Post", Post {}).canonical(SITE, &POST.url(3)).render();
    /// assert!(html.contains(r#"<link rel="canonical" href="https://example.com/posts/3">"#));
    /// # let _ = pages;
    /// ```
    #[must_use]
    pub fn canonical(mut self, origin: Origin, url: &Url) -> Self {
        self.canonical = Some(origin.join(url.as_str()));
        self
    }

    /// Names `url` on `origin` as the same page in the language `hreflang`, a BCP 47 tag.
    ///
    /// Search engines show the visitor the version in their language. Each
    /// version lists every version, itself included, and
    /// `hypersig_i18n::LocalePage::alternates` adds one per locale of an
    /// application.
    ///
    /// ```
    /// use hypersig::{Origin, Page, Route, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Home {}
    ///
    /// # async fn en((): (), (): ()) -> Page<Home> { Page::new("Home", Home {}) }
    /// # async fn ja((): (), (): ()) -> Page<Home> { Page::new("Home", Home {}) }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const EN: Route = "/en" => en;
    ///     const JA: Route = "/ja" => ja;
    /// }
    ///
    /// const SITE: Origin = Origin::new("https://example.com");
    ///
    /// let html = Page::new("Home", Home {})
    ///     .alternate(SITE, &EN.url(()), "en")
    ///     .alternate(SITE, &JA.url(()), "ja")
    ///     .alternate_default(SITE, &EN.url(()))
    ///     .render();
    /// assert!(html.contains(r#"<link rel="alternate" hreflang="ja" href="https://example.com/ja">"#));
    /// assert!(html.contains(r#"<link rel="alternate" hreflang="x-default" href="https://example.com/en">"#));
    /// # let _ = pages;
    /// ```
    #[must_use]
    pub fn alternate(mut self, origin: Origin, url: &Url, hreflang: impl Into<String>) -> Self {
        self.alternates
            .push((Some(hreflang.into()), origin.join(url.as_str())));
        self
    }

    /// Names `url` on `origin` as the version for a visitor whose language no
    /// [`Page::alternate`] matches, the `x-default` alternate.
    #[must_use]
    pub fn alternate_default(mut self, origin: Origin, url: &Url) -> Self {
        self.alternates.push((None, origin.join(url.as_str())));
        self
    }

    /// Asks search engines to leave the page out of their results.
    ///
    /// ```
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Cart {}
    ///
    /// let html = Page::new("Cart", Cart {}).noindex().nofollow().render();
    /// assert!(html.contains(r#"<meta name="robots" content="noindex, nofollow">"#));
    /// ```
    #[must_use]
    pub fn noindex(mut self) -> Self {
        self.noindex = true;
        self
    }

    /// Asks search engines not to follow the links on the page.
    #[must_use]
    pub fn nofollow(mut self) -> Self {
        self.nofollow = true;
        self
    }

    /// Adds `data` to `<head>` as JSON-LD, the structured data search engines read.
    ///
    /// The JSON is written into a `<script type="application/ld+json">`
    /// block with each `<` written as `\u003c`, so no string inside it can
    /// close the block. A later call adds another block.
    ///
    /// ```
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    /// use serde_json::json;
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Post {}
    ///
    /// let html = Page::new("Post", Post {})
    ///     .json_ld(&json!({ "@type": "Article", "headline": "A & </script>" }))
    ///     .render();
    /// assert!(html.contains(
    ///     r#"<script type="application/ld+json">{"@type":"Article","headline":"A & \u003c/script>"}</script>"#
    /// ));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `data` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error or a map with keys that are not
    /// strings.
    #[must_use]
    pub fn json_ld(mut self, data: &impl serde::Serialize) -> Self {
        let json = serde_json::to_string(data).expect("JSON-LD data serializes to JSON");
        self.json_ld.push(json.replace('<', "\\u003c"));
        self
    }

    /// Puts `nonce` on the page's `<style>` and `<script>` elements, for a
    /// Content-Security-Policy that allows styles by nonce.
    ///
    /// With it, a page runs under `style-src 'nonce-…'` and `script-src
    /// 'self'` without `'unsafe-inline'`. The runtime puts the same nonce on
    /// each `<style>` it adds for the rules a patch brings. `nonce` is the
    /// value the server generated for this response and wrote into its
    /// `Content-Security-Policy` header, at least 128 random bits.
    ///
    /// A nonce does not reach a `style` attribute, so with a nonce the values
    /// of [`El::bind_style`](crate::html::El::bind_style) and
    /// [`El::bind_token`](crate::html::El::bind_token) are written as rules
    /// of a nonced `<style>`, one per element, and the runtime sets them
    /// inline from then on. A value holding `{`, `}`, `;`, `<`, `>`, a quote,
    /// a backslash, a comment, or unbalanced parentheses could end its rule,
    /// so it is left out, and the element shows without it until the runtime
    /// sets it. A `style` set with [`El::attr`](crate::html::El::attr) stays
    /// an attribute and needs `style-src-attr 'unsafe-inline'`.
    ///
    /// ```
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Note {}
    ///
    /// let html = Page::new("Note", Note {}).csp_nonce("2726c7f26c").render();
    /// let script = format!(r#"<script type="module" src="{}" nonce="2726c7f26c">"#, hypersig::RUNTIME_PATH);
    /// assert!(html.contains(&script));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `nonce` is empty or holds a character other than a letter,
    /// a digit, `+`, `/`, `-`, `_`, or a trailing `=`, which a
    /// Content-Security-Policy nonce cannot hold.
    #[must_use]
    pub fn csp_nonce(mut self, nonce: impl Into<String>) -> Self {
        let nonce = nonce.into();
        let body = nonce.trim_end_matches('=');
        assert!(
            !body.is_empty()
                && body
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'/' | b'-' | b'_')),
            "a CSP nonce is base64 or base64url text"
        );
        self.nonce = Some(nonce);
        self
    }

    /// Uses the file at `href` as the page's icon in tabs and bookmarks.
    ///
    /// A page without it declares an empty icon, so the browser does not
    /// request `/favicon.ico`.
    #[must_use]
    pub fn icon(mut self, href: &'static str) -> Self {
        self.icon = Some(href);
        self
    }

    /// Adds the card a link to the page shows in chat apps and social sites.
    #[must_use]
    pub fn open_graph(mut self, card: OpenGraph) -> Self {
        self.open_graph = Some(card);
        self
    }

    /// Sets the language of the page, the `lang` attribute of `<html>`, to the BCP 47 tag `tag`.
    ///
    /// The browser reads it for hyphenation, the voice of a screen reader,
    /// and the fonts it picks for the text. A page without it has no `lang`.
    ///
    /// ```
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Note {}
    ///
    /// let html = Page::new("ノート", Note {}).lang("ja").render();
    /// assert!(html.starts_with(r#"<!doctype html><html lang="ja">"#));
    /// ```
    #[must_use]
    pub fn lang(mut self, tag: impl Into<String>) -> Self {
        self.lang = Some(tag.into());
        self
    }

    /// Keeps `signal` equal to `value`, recomputed when a signal `value` reads changes.
    ///
    /// The result is stored in a field of the signal struct, so bindings read
    /// it like any other signal and rerun only when the result changes, rather
    /// than each evaluating `value`. The server computes it once for the first
    /// HTML, and an action call can pass it as an argument like any signal.
    /// Computed signals are evaluated in the order they are declared, so one
    /// may read an earlier one.
    ///
    /// The type system does not stop a statement from setting `signal` or the
    /// server from patching it. Either value lasts until a signal `value` reads
    /// changes.
    ///
    /// ```
    /// use hypersig::html::span;
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Cart {
    ///     price: f64,
    ///     quantity: i32,
    ///     total: f64,
    /// }
    ///
    /// let html = Page::new("Cart", Cart { price: 2.5, quantity: 3, total: 0.0 })
    ///     .computed(Cart::total(), Cart::price() * Cart::quantity().to_f64())
    ///     .child(span().bind_text(Cart::total().to_text()))
    ///     .render();
    /// assert!(html.contains("<span data-hs-text=") && html.contains(">7.5</span>"));
    /// ```
    #[must_use]
    pub fn computed<T>(mut self, signal: Sig<S, T>, value: impl IntoExpr<S, T>) -> Self {
        self.computed
            .push((signal.path(), value.into_expr().into_json()));
        self
    }

    /// Keeps the document's title, the text of the browser tab, equal to `title`.
    ///
    /// The server writes the first value into `<title>`, and the runtime
    /// writes each later one into `document.title`, so a tab can show a count
    /// that changes while the page is open. A search engine reads the HTML,
    /// so it shows the first value, and an [`OpenGraph`] card keeps the
    /// title of [`Page::new`].
    ///
    /// ```
    /// use hypersig::{Page, Signals, if_else};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Inbox {
    ///     unread: i32,
    /// }
    ///
    /// let unread = Inbox::unread();
    /// let html = Page::new("Inbox", Inbox { unread: 3 })
    ///     .bind_title(if_else(unread.gt(0), unread.to_text().concat(" unread - Inbox"), "Inbox"))
    ///     .render();
    /// assert!(html.contains("<title>3 unread - Inbox</title>"));
    /// ```
    #[must_use]
    pub fn bind_title(mut self, title: impl IntoExpr<S, String>) -> Self {
        self.bound_title = Some(title.into_expr().into_json());
        self
    }

    /// Loads the runtime from `src` instead of [`RUNTIME_PATH`](crate::RUNTIME_PATH),
    /// for a page served as a static file next to a copy of
    /// [`RUNTIME_JS`](crate::RUNTIME_JS).
    #[must_use]
    pub fn runtime_src(mut self, src: &'static str) -> Self {
        self.runtime = Some(src);
        self
    }

    /// Adds a `<link rel="stylesheet">` for the file at `href` to `<head>`.
    #[must_use]
    pub fn stylesheet(mut self, href: &'static str) -> Self {
        self.stylesheets.push(href);
        self
    }

    /// Adds a `<script type="module">` for the file at `src` to `<head>`.
    ///
    /// It is for a script the page's elements need, such as one that defines
    /// a custom element. The scripts run after the runtime, in the order they
    /// were added. A custom element defined then is upgraded in place, so it
    /// keeps the attributes and the listeners the runtime gave it. It carries
    /// the page's [`csp_nonce`](Page::csp_nonce), if there is one.
    ///
    /// ```
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Place {}
    ///
    /// let html = Page::new("Place", Place {}).script("/map.js").render();
    /// assert!(html.contains(r#"<script type="module" src="/map.js"></script>"#));
    /// ```
    #[must_use]
    pub fn script(mut self, src: &'static str) -> Self {
        self.scripts.push(src);
        self
    }

    /// Adds `sheet` to `<head>` as a `<style>` block.
    ///
    /// Its rules sit outside any cascade layer, so they override the classes
    /// of [`El::css`](crate::html::El::css) wherever both set a property.
    #[must_use]
    pub fn style(mut self, sheet: Stylesheet) -> Self {
        self.styles.push(sheet.css());
        self
    }

    /// Sets token values for the whole page. A later call adds to the theme,
    /// and a value for the same token under the same conditions replaces the
    /// earlier one.
    ///
    /// ```
    /// use hypersig::html::p;
    /// use hypersig::style::value::Color;
    /// use hypersig::style::{Media, Style, Theme, Token};
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Note {}
    ///
    /// let ink = Token::new("ink", Color::hex(0x0022_2222));
    /// let html = Page::new("Note", Note {})
    ///     .theme(Theme::new().on(Media::Dark, Theme::new().set(&ink, Color::hex(0x00ee_eeee))))
    ///     .child(p().css(Style::new().color(ink.var())).text("Hello"))
    ///     .render();
    /// assert!(html.contains("@media (prefers-color-scheme: dark){:root{--ink:#eeeeee}}"));
    /// ```
    #[must_use]
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = std::mem::take(&mut self.theme).merge(theme);
        self
    }

    /// Adds rules for elements by their tag, and fonts, to the page. A later
    /// call adds to the earlier ones.
    ///
    /// ```
    /// use hypersig::html::p;
    /// use hypersig::style::value::FontFamily;
    /// use hypersig::style::{Element, Global, Style};
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Note {}
    ///
    /// let html = Page::new("Note", Note {})
    ///     .global(Global::preflight().rule(Element::Body, Style::new().font_family(FontFamily::SystemUi)))
    ///     .child(p().text("Hello"))
    ///     .render();
    /// assert!(html.contains("<style data-hs-global>"));
    /// ```
    #[must_use]
    pub fn global(mut self, global: Global) -> Self {
        self.global = std::mem::take(&mut self.global).merge(global);
        self
    }

    /// Runs `stmt` when `shortcut` is pressed anywhere on the page.
    ///
    /// It runs while nothing has focus too, and matches as
    /// [`El::on_key`](crate::html::El::on_key) does: a text field keeps the
    /// keys it edits text with unless the shortcut holds Ctrl, Alt, Meta, or
    /// [`primary`](Shortcut::primary), and a handler of `El::on_key` nearer
    /// the focus that matches the same key runs in its place.
    ///
    /// ```
    /// use hypersig::html::{Key, input};
    /// use hypersig::{NodeRef, Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Docs {
    ///     query: String,
    /// }
    ///
    /// const SEARCH: NodeRef<Docs> = NodeRef::new("search");
    /// let html = Page::new("Docs", Docs { query: String::new() })
    ///     .on_key(Key::Char('/'), SEARCH.focus())
    ///     .child(input().node_ref(SEARCH).bind(Docs::query()))
    ///     .render();
    /// assert!(html.contains("<body data-hs-signals"));
    /// assert!(html.contains(r#" data-hs-on-key="[[&quot;/&quot;"#));
    /// ```
    #[must_use]
    pub fn on_key(mut self, shortcut: impl Into<Shortcut>, stmt: impl IntoStmt<S>) -> Self {
        self.keys.push((shortcut.into(), stmt.into_stmt()));
        self
    }

    /// Appends a node to `<body>`.
    #[must_use]
    pub fn child(mut self, child: impl Into<Node<S>>) -> Self {
        self.body.push(child.into());
        self
    }

    /// The signals as JSON, with each computed signal set from the others.
    fn initial_values(&self) -> Tape {
        let mut root =
            Tape::new(&self.signals).expect("a Signals struct serializes to a JSON object");
        for (path, value) in &self.computed {
            let ctx = Ctx {
                root: root.root(),
                scope: Vec::new(),
                copy: false,
                vars: Vec::new(),
                rules: None,
            };
            if let Some(value) = ssr::evaluate(value, &ctx).map(Val::into_value) {
                root = root.replaced(path, &value);
            }
        }
        root
    }

    /// Writes what search engines and link previews read into `<head>`.
    fn write_metadata(&self, out: &mut String) {
        if let Some(description) = &self.description {
            meta(out, "name", "description", description);
        }
        match (self.noindex, self.nofollow) {
            (true, true) => meta(out, "name", "robots", "noindex, nofollow"),
            (true, false) => meta(out, "name", "robots", "noindex"),
            (false, true) => meta(out, "name", "robots", "nofollow"),
            (false, false) => {}
        }
        if let Some(url) = &self.canonical {
            out.push_str("<link rel=\"canonical\"");
            write_attr(out, "href", url);
            out.push('>');
        }
        for (hreflang, url) in &self.alternates {
            out.push_str("<link rel=\"alternate\"");
            write_attr(out, "hreflang", hreflang.as_deref().unwrap_or("x-default"));
            write_attr(out, "href", url);
            out.push('>');
        }
        if let Some(card) = &self.open_graph {
            card.write(
                out,
                &self.title,
                self.description.as_deref(),
                self.canonical.as_deref(),
            );
        }
        for json in &self.json_ld {
            out.push_str("<script type=\"application/ld+json\">");
            out.push_str(json);
            out.push_str("</script>");
        }
    }

    /// The value `signal` starts at in the browser, for a test of a page handler.
    ///
    /// It is the value the page was built with, or for a
    /// [`computed`](Self::computed) signal, the value computed from the others.
    ///
    /// ```
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Cart {
    ///     price: f64,
    ///     quantity: i32,
    ///     total: f64,
    /// }
    ///
    /// let page = Page::new("Cart", Cart { price: 2.5, quantity: 3, total: 0.0 })
    ///     .computed(Cart::total(), Cart::price() * Cart::quantity().to_f64());
    /// assert_eq!(page.value_of(Cart::quantity()), 3);
    /// assert_eq!(page.value_of(Cart::total()), 7.5);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the signal struct fails to serialize to JSON, or if the value
    /// does not deserialize into `T`, which happens where a `Serialize` impl
    /// skips the field or disagrees with its `Deserialize` impl.
    #[must_use]
    pub fn value_of<T: DeserializeOwned>(&self, signal: Sig<S, T>) -> T {
        let root = self.initial_values();
        let value = root
            .root()
            .get_path(&signal.dotted())
            .map_or(Value::Null, Ref::to_value);
        serde_json::from_value(value)
            .expect("a signal's initial value deserializes into the signal's type")
    }

    /// Renders the document.
    ///
    /// # Panics
    ///
    /// Panics if the signal struct fails to serialize to JSON, which only
    /// happens for a `Serialize` impl that returns an error, or if two
    /// elements carry one [`NodeRef`](crate::NodeRef) name, one
    /// [`FileInput`](crate::FileInput) name, or one id, where the runtime can
    /// reach both, or if an element inside a list template has an id set with
    /// [`El::id`](crate::html::El::id). Each item of a list may carry the same
    /// name.
    #[must_use]
    pub fn render(&self) -> String {
        self.render_under(Base::ROOT)
    }

    /// Renders the document for a route under `base`, which the runtime's path starts with.
    pub(crate) fn render_under(&self, base: Base) -> String {
        check_names(&self.body);
        let root = self.initial_values();
        let mut out = String::from("<!doctype html><html");
        if let Some(lang) = &self.lang {
            write_attr(&mut out, "lang", lang);
        }
        out.push_str("><head><meta charset=\"utf-8\">");
        out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        out.push_str("<link rel=\"icon\"");
        write_attr(&mut out, "href", self.icon.unwrap_or("data:,"));
        out.push('>');
        let rules = RefCell::new(Vec::new());
        let ctx = Ctx {
            root: root.root(),
            scope: Vec::new(),
            copy: false,
            vars: Vec::new(),
            rules: self.nonce.as_ref().map(|_| &rules),
        };
        let title = self
            .bound_title
            .as_ref()
            .and_then(|title| ssr::evaluate(title, &ctx))
            .map_or(Cow::Borrowed(self.title.as_str()), |title| {
                Cow::Owned(ssr::js_string(&title))
            });
        out.push_str("<title>");
        escape_into(&mut out, &title, false);
        out.push_str("</title>");
        self.write_metadata(&mut out);
        let styles_at = out.len();
        self.write_styles(&mut out);
        out.push_str("<script type=\"module\"");
        let runtime: Cow<'_, str> = match self.runtime {
            Some(src) => Cow::Borrowed(src),
            None if base == Base::ROOT => Cow::Borrowed(crate::RUNTIME_PATH),
            None => Cow::Owned(base.join(crate::RUNTIME_PATH)),
        };
        write_attr(&mut out, "src", &runtime);
        self.write_nonce(&mut out);
        out.push_str("></script>");
        for src in &self.scripts {
            out.push_str("<script type=\"module\"");
            write_attr(&mut out, "src", src);
            self.write_nonce(&mut out);
            out.push_str("></script>");
        }
        out.push_str("</head><body");
        self.write_body_attrs(&mut out, &root);
        out.push('>');
        let body_at = out.len();
        for child in &self.body {
            child.write(&mut out, Some(&ctx));
        }
        let hidden = out[body_at..].contains(HIDE);
        let rules = rules.into_inner();
        if hidden || !rules.is_empty() {
            out.insert_str(styles_at, &self.inline_style(hidden, &rules));
        }
        out.push_str("</body></html>");
        out
    }

    /// The `<style>` that stands in for the `style` attributes of the body:
    /// the rule that hides each element with `HIDE`, if `hidden`, and one
    /// rule per entry of `rules`, which selects its element by `INLINE`.
    ///
    /// `!important` gives each rule the precedence an inline style would
    /// have over the page's other rules.
    fn inline_style(&self, hidden: bool, rules: &[String]) -> String {
        let mut style = String::from("<style");
        self.write_nonce(&mut style);
        style.push('>');
        if hidden {
            style.push('[');
            style.push_str(HIDE);
            style.push_str("]{display:none!important}");
        }
        for (at, rule) in rules.iter().enumerate() {
            style.push('[');
            style.push_str(INLINE);
            style.push_str("=\"");
            style.push_str(&at.to_string());
            style.push_str("\"]{");
            style.push_str(rule);
            style.push('}');
        }
        style.push_str("</style>");
        style
    }

    /// Writes the attributes of `<body>`, which the runtime reads when it starts.
    fn write_body_attrs(&self, out: &mut String, root: &Tape) {
        // `write_json_attr` reads all of `out` once, which here holds only
        // the head, because the body attributes come before the body.
        write_json_attr(out, "data-hs-signals", &root.root());
        if !self.computed.is_empty() {
            let computed: Vec<Value> = self
                .computed
                .iter()
                .map(|(path, value)| json!([path, value]))
                .collect();
            write_attr(out, "data-hs-computed", &Value::from(computed).to_string());
        }
        if let Some(title) = &self.bound_title {
            write_attr(out, "data-hs-title", &title.to_string());
        }
        if !self.keys.is_empty() {
            write_attr(out, "data-hs-on-key", &keys_json(&self.keys));
        }
    }

    /// Writes the `nonce` attribute, if the page has a nonce.
    fn write_nonce(&self, out: &mut String) {
        if let Some(nonce) = &self.nonce {
            write_attr(out, "nonce", nonce);
        }
    }

    /// Writes the global rules, the theme, the atomic classes of the body,
    /// and the page's stylesheets into `<head>`.
    fn write_styles(&self, out: &mut String) {
        if !self.global.is_empty() {
            out.push_str("<style data-hs-global");
            self.write_nonce(out);
            out.push('>');
            out.push_str(&self.global.to_css());
            out.push_str("</style>");
        }
        if !self.theme.is_empty() {
            out.push_str("<style data-hs-theme");
            self.write_nonce(out);
            out.push('>');
            out.push_str(&self.theme.to_css());
            out.push_str("</style>");
        }
        let mut sheet = Sheet::new();
        for child in &self.body {
            child.collect_css(&mut sheet);
        }
        for bucket in sheet.buckets() {
            out.push_str("<style");
            write_attr(out, "data-hs-css", bucket.sort_key());
            self.write_nonce(out);
            out.push('>');
            out.push_str(bucket.css());
            out.push_str("</style>");
        }
        for href in &self.stylesheets {
            out.push_str("<link rel=\"stylesheet\"");
            write_attr(out, "href", href);
            out.push('>');
        }
        for css in &self.styles {
            out.push_str("<style");
            self.write_nonce(out);
            out.push('>');
            out.push_str(css);
            out.push_str("</style>");
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::html::{Stylesheet, input, p};
    use crate::style::value::Color;
    use crate::style::{Global, Property, Style, Theme, Token};
    use crate::{NodeRef, Page, Signals};

    #[derive(Signals, Serialize, Deserialize, Default)]
    struct Search {
        query: String,
    }

    const FIELD: NodeRef<Search> = NodeRef::new("field");

    #[test]
    fn a_hidden_element_has_no_style_attribute_and_the_page_hides_it_by_rule() {
        let html = Page::new("Search", Search::default())
            .csp_nonce("abc+/=")
            .child(p().show(!Search::query().is_empty()).text("Results"))
            .render();
        assert!(
            html.contains(
                r#"<style nonce="abc+/=">[data-hs-hide]{display:none!important}</style>"#
            )
        );
        assert!(html.contains("<p data-hs-hide=\"\""));
        assert!(!html.contains("style=\""));
    }

    #[test]
    fn a_page_without_a_hidden_element_has_no_hide_rule() {
        let html = Page::new("Search", Search::default())
            .child(p().show(Search::query().is_empty()).text("Nothing yet"))
            .render();
        assert!(!html.contains("display:none"));
    }

    fn bars(nonce: Option<&str>) -> String {
        let page = Page::new(
            "Search",
            Search {
                query: "50%".to_owned(),
            },
        )
        .child(p().bind_style(Property::Width, Search::query()))
        .child(
            p().attr("style", "color: red")
                .bind_style(Property::Height, Search::query())
                .bind_token(&Token::new("ink", Color::RED), "blue".to_owned()),
        )
        .child(p().bind_style(Property::Width, "1px}body{color:red".to_owned()));
        match nonce {
            Some(nonce) => page.csp_nonce(nonce).render(),
            None => page.render(),
        }
    }

    #[test]
    fn without_a_nonce_bound_styles_are_inline() {
        let html = bars(None);
        assert!(html.contains(r#"style="width: 50%""#));
        assert!(html.contains(r#"style="color: red; height: 50%; --ink: blue""#));
        assert!(!html.contains("data-hs-inline"));
    }

    #[test]
    fn with_a_nonce_bound_styles_are_rules_of_the_nonced_style() {
        let html = bars(Some("n0nce"));
        assert!(html.contains(concat!(
            r#"<style nonce="n0nce">"#,
            r#"[data-hs-inline="0"]{width:50%!important;}"#,
            r#"[data-hs-inline="1"]{height:50%!important;--ink:blue!important;}"#,
            "</style>"
        )));
        assert!(html.contains(r#"data-hs-inline="0""#));
        assert!(html.contains(r#"style="color: red""#));
        assert_eq!(html.matches(" style=").count(), 1);
        assert!(!html.contains(r#"data-hs-inline="2""#));
    }

    #[test]
    fn the_nonce_is_on_every_style_and_the_runtime_script() {
        let html = Page::new("Search", Search::default())
            .csp_nonce("n0nce")
            .global(Global::preflight())
            .theme(Theme::new().set(&Token::new("ink", Color::RED), Color::BLUE))
            .style(Stylesheet::new(".x{}"))
            .child(p().css(Style::new().color(Color::RED)))
            .render();
        assert_eq!(
            html.matches("<style").count(),
            html.matches(r#" nonce="n0nce">"#).count() - 1
        );
        assert_eq!(html.matches("<style").count(), 4);
    }

    #[test]
    fn scripts_follow_the_runtime_in_order_with_the_nonce() {
        let html = Page::new("Search", Search::default())
            .csp_nonce("n0nce")
            .script("/map.js")
            .script("/editor.js")
            .render();
        let runtime = html.find(crate::RUNTIME_PATH).expect("the runtime script");
        let map = html
            .find(r#"<script type="module" src="/map.js" nonce="n0nce"></script>"#)
            .expect("the map script");
        let editor = html
            .find(r#"<script type="module" src="/editor.js" nonce="n0nce"></script>"#)
            .expect("the editor script");
        assert!(runtime < map && map < editor && editor < html.find("</head>").unwrap());
    }

    #[test]
    #[should_panic(expected = "a CSP nonce is base64")]
    fn a_nonce_that_could_end_the_attribute_panics() {
        let _ = Page::new("Search", Search::default()).csp_nonce("a\" onload=\"x");
    }

    #[test]
    fn robots_lists_only_what_the_page_asks() {
        let page = || Page::new("Search", Search::default());
        let robots = |html: String| {
            html.split_once(r#"<meta name="robots" content=""#)
                .map(|(_, rest)| rest.split_once('"').unwrap().0.to_owned())
        };
        assert_eq!(robots(page().render()), None);
        assert_eq!(
            robots(page().noindex().render()).as_deref(),
            Some("noindex")
        );
        assert_eq!(
            robots(page().nofollow().render()).as_deref(),
            Some("nofollow")
        );
    }

    #[test]
    fn json_ld_cannot_close_its_script_or_open_a_comment() {
        let html = Page::new("Search", Search::default())
            .json_ld(&["</script><script>alert(1)</script>", "<!--", "a & b"])
            .render();
        assert!(html.contains(
            r#"<script type="application/ld+json">["\u003c/script>\u003cscript>alert(1)\u003c/script>","\u003c!--","a & b"]</script>"#
        ));
    }

    #[test]
    #[should_panic(expected = "node ref name `field`")]
    fn one_node_ref_in_two_children_of_the_body_panics() {
        let _ = Page::new(
            "Search",
            Search {
                query: String::new(),
            },
        )
        .child(input().node_ref(FIELD))
        .child(input().node_ref(FIELD))
        .render();
    }
}
