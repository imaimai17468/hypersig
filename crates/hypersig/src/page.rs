//! The full HTML document a page is served as.

use std::borrow::Cow;
use std::fmt;

use serde_json::{Value, json};

use crate::expr::IntoExpr;
use crate::html::{Node, Stylesheet, escape_into, write_attr};
use crate::signal::{Sig, Signals};
use crate::ssr::{self, Ctx};
use crate::style::Sheet;

/// A full HTML document for a page whose signals are `S`.
///
/// The document loads the runtime from [`RUNTIME_PATH`](crate::RUNTIME_PATH),
/// or from [`Page::runtime_src`], and puts the initial signal values on
/// `<body>`. It is rendered with those
/// values already applied: text, visibility, classes, attributes, form
/// values, list items, and conditional elements show before the runtime
/// loads, and the runtime adopts the elements instead of rebuilding them.
pub struct Page<S> {
    title: String,
    signals: S,
    stylesheets: Vec<&'static str>,
    styles: Vec<&'static str>,
    computed: Vec<(String, Value)>,
    body: Vec<Node<S>>,
    runtime: &'static str,
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
            signals,
            stylesheets: Vec::new(),
            styles: Vec::new(),
            computed: Vec::new(),
            body: Vec::new(),
            runtime: crate::RUNTIME_PATH,
        }
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

    /// Loads the runtime from `src` instead of [`RUNTIME_PATH`](crate::RUNTIME_PATH),
    /// for a page served as a static file next to a copy of
    /// [`RUNTIME_JS`](crate::RUNTIME_JS).
    #[must_use]
    pub fn runtime_src(mut self, src: &'static str) -> Self {
        self.runtime = src;
        self
    }

    /// Adds a `<link rel="stylesheet">` for the file at `href` to `<head>`.
    #[must_use]
    pub fn stylesheet(mut self, href: &'static str) -> Self {
        self.stylesheets.push(href);
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

    /// Appends a node to `<body>`.
    #[must_use]
    pub fn child(mut self, child: impl Into<Node<S>>) -> Self {
        self.body.push(child.into());
        self
    }

    /// Renders the document.
    ///
    /// # Panics
    ///
    /// Panics if the signal struct fails to serialize to JSON, which only
    /// happens for a `Serialize` impl that returns an error.
    #[must_use]
    pub fn render(&self) -> String {
        let mut root = serde_json::to_value(&self.signals)
            .expect("a Signals struct serializes to a JSON object");
        for (path, value) in &self.computed {
            let ctx = Ctx {
                root: &root,
                scope: Vec::new(),
                copy: false,
            };
            if let Some(value) = ssr::evaluate(value, &ctx).map(Cow::into_owned) {
                root = ssr::set_path(&root, path, value);
            }
        }
        let mut out = String::from("<!doctype html><html><head><meta charset=\"utf-8\">");
        out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        out.push_str("<link rel=\"icon\" href=\"data:,\">");
        out.push_str("<title>");
        escape_into(&mut out, &self.title, false);
        out.push_str("</title>");
        let mut sheet = Sheet::new();
        for child in &self.body {
            child.collect_css(&mut sheet);
        }
        for bucket in sheet.buckets() {
            out.push_str("<style");
            write_attr(&mut out, "data-hs-css", &bucket.sort_key());
            out.push('>');
            out.push_str(bucket.css());
            out.push_str("</style>");
        }
        for href in &self.stylesheets {
            out.push_str("<link rel=\"stylesheet\"");
            write_attr(&mut out, "href", href);
            out.push('>');
        }
        for css in &self.styles {
            out.push_str("<style>");
            out.push_str(css);
            out.push_str("</style>");
        }
        out.push_str("<script type=\"module\"");
        write_attr(&mut out, "src", self.runtime);
        out.push_str("></script></head><body");
        write_attr(&mut out, "data-hs-signals", &root.to_string());
        if !self.computed.is_empty() {
            let computed: Vec<Value> = self
                .computed
                .iter()
                .map(|(path, value)| json!([path, value]))
                .collect();
            write_attr(
                &mut out,
                "data-hs-computed",
                &Value::from(computed).to_string(),
            );
        }
        out.push('>');
        let ctx = Ctx {
            root: &root,
            scope: Vec::new(),
            copy: false,
        };
        for child in &self.body {
            child.write(&mut out, Some(&ctx));
        }
        out.push_str("</body></html>");
        out
    }
}
