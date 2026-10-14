//! An HTML builder whose reactive attributes only accept typed values.
//!
//! Every element carries the signal struct `S` of the page it belongs to.
//! A child must share it, so a signal, expression, or action from another page
//! does not compile inside this one.
//!
//! The builder writes the `data-hs-*` attributes the runtime reads. Writing one
//! of them by hand through [`El::attr`] panics, because it would bypass the
//! types.

use std::collections::BTreeMap;
use std::fmt::{self, Write as _};

use crate::expr::{Expr, IntoExpr, Stmt};
use crate::signal::{Sig, Signals};

const RESERVED_PREFIX: &str = "data-hs-";

/// A node in the tree: an element or a run of text.
pub struct Node<S>(NodeKind<S>);

enum NodeKind<S> {
    Element(Box<El<S>>),
    Text(String),
}

impl<S> fmt::Debug for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            NodeKind::Element(el) => el.fmt(f),
            NodeKind::Text(text) => f.debug_tuple("Text").field(text).finish(),
        }
    }
}

impl<S> From<El<S>> for Node<S> {
    fn from(el: El<S>) -> Self {
        Self(NodeKind::Element(Box::new(el)))
    }
}

impl<S> From<&str> for Node<S> {
    fn from(text: &str) -> Self {
        Self(NodeKind::Text(text.to_owned()))
    }
}

impl<S> From<String> for Node<S> {
    fn from(text: String) -> Self {
        Self(NodeKind::Text(text))
    }
}

/// An HTML element belonging to a page whose signals are `S`.
pub struct El<S> {
    tag: &'static str,
    void: bool,
    id: Option<String>,
    attrs: Vec<(&'static str, String)>,
    classes: Vec<&'static str>,
    show: Option<Expr<S, bool>>,
    text: Option<Expr<S, String>>,
    bind: Option<&'static str>,
    class_when: Vec<(&'static str, Expr<S, bool>)>,
    on: BTreeMap<Event, Stmt<S>>,
    children: Vec<Node<S>>,
}

impl<S> fmt::Debug for El<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("El")
            .field("tag", &self.tag)
            .field("id", &self.id)
            .field("children", &self.children.len())
            .finish_non_exhaustive()
    }
}

/// A DOM event a statement can run on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Event {
    /// `click`.
    Click,
    /// `input`, fired on every keystroke in a text field.
    Input,
    /// `change`.
    Change,
    /// `submit`. The runtime calls `preventDefault` first.
    Submit,
    /// Runs once when the runtime first sees the element.
    Load,
}

impl Event {
    fn attribute(self) -> &'static str {
        match self {
            Self::Click => "data-hs-on-click",
            Self::Input => "data-hs-on-input",
            Self::Change => "data-hs-on-change",
            Self::Submit => "data-hs-on-submit",
            Self::Load => "data-hs-on-load",
        }
    }
}

/// A field type that an `<input>` can be bound to with [`El::bind`].
///
/// `String` binds to the text value and `bool` to `checked`.
#[diagnostic::on_unimplemented(
    message = "an `<input>` cannot be bound to a `{Self}` signal",
    note = "bind a `String` signal to a text field or a `bool` signal to a checkbox"
)]
pub trait Bindable: sealed::Sealed {}
impl Bindable for String {}
impl Bindable for bool {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for String {}
    impl Sealed for bool {}
}

/// Creates an element with any tag.
///
/// Use the named constructors such as [`div`] where one exists.
#[must_use]
pub fn el<S>(tag: &'static str) -> El<S> {
    El::new(tag, false)
}

macro_rules! tags {
    ($($name:ident),* $(,)?) => {$(
        #[doc = concat!("Creates a `<", stringify!($name), ">` element.")]
        #[must_use]
        pub fn $name<S>() -> El<S> {
            El::new(stringify!($name), false)
        }
    )*};
}

tags!(
    a, article, aside, button, code, div, footer, form, h1, h2, h3, header, label, li, main, nav,
    ol, p, pre, section, small, span, strong, table, tbody, td, th, thead, tr, ul,
);

/// Creates an `<input>` element.
#[must_use]
pub fn input<S>() -> El<S> {
    El::new("input", true)
}

impl<S> El<S> {
    fn new(tag: &'static str, void: bool) -> Self {
        Self {
            tag,
            void,
            id: None,
            attrs: Vec::new(),
            classes: Vec::new(),
            show: None,
            text: None,
            bind: None,
            class_when: Vec::new(),
            on: BTreeMap::new(),
            children: Vec::new(),
        }
    }

    /// Sets a plain attribute.
    ///
    /// # Panics
    ///
    /// Panics if `name` is `id` or `class`, which have their own methods, or
    /// starts with `data-hs-`, which only the typed methods may write.
    #[must_use]
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        assert!(
            !name.starts_with(RESERVED_PREFIX),
            "`{name}` is written by hypersig's typed methods, not by `attr`"
        );
        assert!(
            name != "id" && name != "class",
            "use `El::id` or `El::class` to set `{name}`"
        );
        self.attrs.push((name, value.into()));
        self
    }

    /// Sets the element's `id`.
    ///
    /// An element rendered through a [`Fragment`](crate::Fragment) gets the
    /// fragment's id instead.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds a static class.
    #[must_use]
    pub fn class(mut self, class: &'static str) -> Self {
        self.classes.push(class);
        self
    }

    /// Adds `class` while `when` is true.
    #[must_use]
    pub fn class_when(mut self, class: &'static str, when: impl IntoExpr<S, bool>) -> Self {
        self.class_when.push((class, when.into_expr()));
        self
    }

    /// Appends a child element or text.
    #[must_use]
    pub fn child(mut self, child: impl Into<Node<S>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends several children.
    #[must_use]
    pub fn children<N: Into<Node<S>>>(mut self, children: impl IntoIterator<Item = N>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    /// Appends static text. Shorthand for `.child(text)`.
    #[must_use]
    pub fn text(self, text: impl Into<String>) -> Self {
        self.child(Node(NodeKind::Text(text.into())))
    }

    /// Shows the element only while `when` is true.
    #[must_use]
    pub fn show(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.show = Some(when.into_expr());
        self
    }

    /// Replaces the element's text with `text`, kept up to date as signals change.
    #[must_use]
    pub fn bind_text(mut self, text: impl IntoExpr<S, String>) -> Self {
        self.text = Some(text.into_expr());
        self
    }

    /// Binds an `<input>` to a signal in both directions.
    #[must_use]
    pub fn bind<T: Bindable>(mut self, signal: Sig<S, T>) -> Self {
        self.bind = Some(signal.path());
        self
    }

    /// Runs `stmt` on `event`. A second call for the same event runs after the first.
    #[must_use]
    pub fn on(mut self, event: Event, stmt: Stmt<S>) -> Self {
        let stmt = match self.on.remove(&event) {
            Some(existing) => existing.then(stmt),
            None => stmt,
        };
        self.on.insert(event, stmt);
        self
    }

    /// Runs `stmt` on click.
    #[must_use]
    pub fn on_click(self, stmt: Stmt<S>) -> Self {
        self.on(Event::Click, stmt)
    }

    /// Runs `stmt` when a form is submitted, after `preventDefault`.
    #[must_use]
    pub fn on_submit(self, stmt: Stmt<S>) -> Self {
        self.on(Event::Submit, stmt)
    }

    /// Runs `stmt` on every edit of a text field.
    #[must_use]
    pub fn on_input(self, stmt: Stmt<S>) -> Self {
        self.on(Event::Input, stmt)
    }

    /// Runs `stmt` on `change`.
    #[must_use]
    pub fn on_change(self, stmt: Stmt<S>) -> Self {
        self.on(Event::Change, stmt)
    }

    /// Runs `stmt` once when the runtime first sees the element.
    #[must_use]
    pub fn on_load(self, stmt: Stmt<S>) -> Self {
        self.on(Event::Load, stmt)
    }

    pub(crate) fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_owned());
        self
    }

    /// Renders the element and its children to an HTML string.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        out.push('<');
        out.push_str(self.tag);
        if let Some(id) = &self.id {
            write_attr(out, "id", id);
        }
        if !self.classes.is_empty() {
            write_attr(out, "class", &self.classes.join(" "));
        }
        for (name, value) in &self.attrs {
            write_attr(out, name, value);
        }
        if let Some(show) = &self.show {
            write_attr(out, "data-hs-show", &show.to_json());
        }
        if let Some(text) = &self.text {
            write_attr(out, "data-hs-text", &text.to_json());
        }
        if let Some(path) = self.bind {
            write_attr(out, "data-hs-bind", path);
        }
        if !self.class_when.is_empty() {
            let pairs: Vec<String> = self
                .class_when
                .iter()
                .map(|(class, when)| {
                    format!("[{},{}]", serde_json::Value::from(*class), when.to_json())
                })
                .collect();
            write_attr(out, "data-hs-class", &format!("[{}]", pairs.join(",")));
        }
        for (event, stmt) in &self.on {
            write_attr(out, event.attribute(), &stmt.to_json());
        }
        out.push('>');
        if self.void {
            return;
        }
        for child in &self.children {
            match &child.0 {
                NodeKind::Element(el) => el.write(out),
                NodeKind::Text(text) => escape_into(out, text, false),
            }
        }
        // `fmt::Write` for `String` never fails.
        let _ = write!(out, "</{}>", self.tag);
    }
}

impl<S> fmt::Display for El<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.render())
    }
}

fn write_attr(out: &mut String, name: &str, value: &str) {
    out.push(' ');
    out.push_str(name);
    out.push_str("=\"");
    escape_into(out, value, true);
    out.push('"');
}

fn escape_into(out: &mut String, text: &str, attribute: bool) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' if attribute => out.push_str("&quot;"),
            '\'' if attribute => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
}

/// A full HTML document for a page whose signals are `S`.
///
/// The document loads the runtime from [`RUNTIME_PATH`](crate::RUNTIME_PATH)
/// and puts the initial signal values on `<body>`.
pub struct Page<S> {
    title: String,
    signals: S,
    styles: Vec<&'static str>,
    body: Vec<Node<S>>,
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
            styles: Vec::new(),
            body: Vec::new(),
        }
    }

    /// Adds a `<style>` block to `<head>`.
    #[must_use]
    pub fn style(mut self, css: &'static str) -> Self {
        self.styles.push(css);
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
        let signals = serde_json::to_string(&self.signals)
            .expect("a Signals struct serializes to a JSON object");
        let mut out = String::from("<!doctype html><html><head><meta charset=\"utf-8\">");
        out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        out.push_str("<title>");
        escape_into(&mut out, &self.title, false);
        out.push_str("</title>");
        for css in &self.styles {
            out.push_str("<style>");
            out.push_str(css);
            out.push_str("</style>");
        }
        out.push_str("<script type=\"module\"");
        write_attr(&mut out, "src", crate::RUNTIME_PATH);
        out.push_str("></script></head><body");
        write_attr(&mut out, "data-hs-signals", &signals);
        out.push('>');
        for child in &self.body {
            match &child.0 {
                NodeKind::Element(el) => el.write(&mut out),
                NodeKind::Text(text) => escape_into(&mut out, text, false),
            }
        }
        out.push_str("</body></html>");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{button, div, input, span};
    use crate::_private::sig;
    use crate::{Sig, Stmt};

    struct Page;

    fn count() -> Sig<Page, i32> {
        sig("count")
    }

    fn name() -> Sig<Page, String> {
        sig("name")
    }

    #[test]
    fn renders_typed_attributes() {
        let html = div::<Page>()
            .class("box")
            .show(count().gt(0))
            .child(span().bind_text(count().to_text()))
            .child(input().bind(name()))
            .child(button().on_click(count().set(count() + 1)).text("+"))
            .render();
        assert_eq!(
            html,
            concat!(
                r#"<div class="box" data-hs-show="[&quot;gt&quot;,[&quot;sig&quot;,&quot;count&quot;],[&quot;lit&quot;,0]]">"#,
                r#"<span data-hs-text="[&quot;str&quot;,[&quot;sig&quot;,&quot;count&quot;]]"></span>"#,
                r#"<input data-hs-bind="name">"#,
                r#"<button data-hs-on-click="[&quot;set&quot;,&quot;count&quot;,[&quot;add&quot;,[&quot;sig&quot;,&quot;count&quot;],[&quot;lit&quot;,1]]]">+</button>"#,
                "</div>"
            )
        );
    }

    #[test]
    fn escapes_text_and_attributes() {
        let html = div::<Page>()
            .attr("title", "a\"b<")
            .text("<script>&")
            .render();
        assert_eq!(
            html,
            r#"<div title="a&quot;b&lt;">&lt;script&gt;&amp;</div>"#
        );
    }

    #[test]
    fn second_handler_runs_after_the_first() {
        let first: Stmt<Page> = count().set(1);
        let html = button().on_click(first).on_click(count().set(2)).render();
        assert!(html.contains("seq"), "{html}");
    }

    #[test]
    #[should_panic(expected = "typed methods")]
    fn reserved_attribute_panics() {
        let _ = div::<Page>().attr("data-hs-show", "true");
    }
}
