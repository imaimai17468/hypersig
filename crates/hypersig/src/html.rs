//! An HTML builder whose reactive attributes only accept typed values.
//!
//! Every element carries the signal struct `S` of the page it belongs to.
//! A child must share it, so a signal, expression, or action from another page
//! does not compile inside this one.
//!
//! The builder writes the `data-hs-*` attributes the runtime reads. Writing one
//! of them by hand through [`El::attr`] panics, because it would bypass the
//! types.
//!
//! A component is a function that returns an [`El`]. One that takes a
//! [`Sig`] to a nested signal struct can be placed several times on a page,
//! each time with its own state:
//!
//! ```
//! use hypersig::html::{El, button, div, p};
//! use hypersig::{Sig, Signals};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Signals, Serialize, Deserialize, Default)]
//! struct Disclosure {
//!     open: bool,
//! }
//!
//! fn disclosure<S>(state: Sig<S, Disclosure>, title: &str, body: &str) -> El<S> {
//!     let open = state.at(Disclosure::open());
//!     div()
//!         .child(button().on_click(open.toggle()).text(title))
//!         .child(p().show(open).text(body))
//! }
//!
//! #[derive(Signals, Serialize, Deserialize, Default)]
//! struct Faq {
//!     shipping: Disclosure,
//!     returns: Disclosure,
//! }
//!
//! let html = div()
//!     .child(disclosure(Faq::shipping(), "Shipping", "Two days."))
//!     .child(disclosure(Faq::returns(), "Returns", "Thirty days."))
//!     .render();
//! assert!(html.contains("shipping.open") && html.contains("returns.open"));
//! ```

use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};

use crate::action::NodeRef;
use crate::expr::{Expr, IntoExpr, IntoStmt, Stmt};
use crate::signal::{Item, Sig, Signals, in_item_scope};

const RESERVED_PREFIX: &str = "data-hs-";

/// A node in the tree: an element, a run of text, or a template the browser expands.
pub struct Node<S>(NodeKind<S>);

enum NodeKind<S> {
    Element(Box<El<S>>),
    Text(String),
    Each {
        list: Value,
        depth: u8,
        template: Box<El<S>>,
    },
    When {
        cond: Expr<S, bool>,
        template: Box<El<S>>,
    },
}

impl<S> fmt::Debug for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            NodeKind::Element(el) => el.fmt(f),
            NodeKind::Text(text) => f.debug_tuple("Text").field(text).finish(),
            NodeKind::Each { list, template, .. } => f
                .debug_struct("Each")
                .field("list", list)
                .field("template", template)
                .finish_non_exhaustive(),
            NodeKind::When { cond, template } => f
                .debug_struct("When")
                .field("cond", cond)
                .field("template", template)
                .finish(),
        }
    }
}

impl<S> Node<S> {
    fn write(&self, out: &mut String) {
        match &self.0 {
            NodeKind::Element(el) => el.write(out),
            NodeKind::Text(text) => escape_into(out, text, false),
            NodeKind::Each {
                list,
                depth,
                template,
            } => {
                out.push_str("<template");
                write_attr(out, "data-hs-each", &json!([list, depth]).to_string());
                out.push('>');
                template.write(out);
                out.push_str("</template>");
            }
            NodeKind::When { cond, template } => {
                out.push_str("<template");
                write_attr(out, "data-hs-if", &cond.to_json());
                out.push('>');
                template.write(out);
                out.push_str("</template>");
            }
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

/// Renders `render` once per item of `list`, in the browser, kept in step with the list.
///
/// `render` runs once on the server to build a template, receiving an
/// [`Item`] that stands for whichever item the browser is rendering. Give the
/// root element a [`El::key`] so an item keeps its element when the list is
/// reordered or an item before it is removed. Without a key, items are matched
/// by position.
///
/// ```
/// use hypersig::html::{each, li, ul};
/// use hypersig::Signals;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Todo {
///     id: i32,
///     title: String,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Todos {
///     items: Vec<Todo>,
/// }
///
/// let list = ul::<Todos>().child(each(Todos::items(), |todo| {
///     li().key(todo.at(Todo::id())).bind_text(todo.at(Todo::title()))
/// }));
/// assert!(list.render().contains("data-hs-each"));
/// ```
#[must_use]
pub fn each<S, T, R>(list: Sig<S, Vec<T>, R>, render: impl FnOnce(Item<S, T>) -> El<S>) -> Node<S> {
    let path = list.raw_path();
    in_item_scope(|depth| {
        let template = render(Item::new(path, depth));
        Node(NodeKind::Each {
            list: path.place_json(),
            depth,
            template: Box::new(template),
        })
    })
}

/// Puts `el` in the document only while `cond` is true.
///
/// Unlike [`El::show`], which hides the element, `when` removes it, so its
/// [`El::on_load`] runs again and its inputs start empty each time it returns.
#[must_use]
pub fn when<S>(cond: impl IntoExpr<S, bool>, el: El<S>) -> Node<S> {
    Node(NodeKind::When {
        cond: cond.into_expr(),
        template: Box::new(el),
    })
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
    checked: Option<Expr<S, bool>>,
    bind: Option<Value>,
    class_when: Vec<(&'static str, Expr<S, bool>)>,
    attr_bind: Vec<(&'static str, Expr<S, String>)>,
    attr_when: Vec<(&'static str, Expr<S, bool>)>,
    style: Vec<(&'static str, Expr<S, String>)>,
    key: Option<Value>,
    node_ref: Option<&'static str>,
    effects: Vec<(Value, Stmt<S>)>,
    every: Option<(u128, Stmt<S>)>,
    on: BTreeMap<Event, Stmt<S>>,
    keys: Vec<(Key, Stmt<S>)>,
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
    /// `dblclick`.
    DoubleClick,
    /// `input`, fired on every keystroke in a text field.
    Input,
    /// `change`.
    Change,
    /// `submit`. The runtime calls `preventDefault` first.
    Submit,
    /// `keydown`, for any key. Use [`El::on_key`] for one key.
    KeyDown,
    /// The element or one inside it gains focus (`focusin`).
    Focus,
    /// The element or one inside it loses focus (`focusout`).
    Blur,
    /// The pointer enters the element (`mouseenter`).
    MouseEnter,
    /// The pointer leaves the element (`mouseleave`).
    MouseLeave,
    /// Runs once when the element enters the document.
    Load,
}

impl Event {
    fn attribute(self) -> &'static str {
        match self {
            Self::Click => "data-hs-on-click",
            Self::DoubleClick => "data-hs-on-dblclick",
            Self::Input => "data-hs-on-input",
            Self::Change => "data-hs-on-change",
            Self::Submit => "data-hs-on-submit",
            Self::KeyDown => "data-hs-on-keydown",
            Self::Focus => "data-hs-on-focusin",
            Self::Blur => "data-hs-on-focusout",
            Self::MouseEnter => "data-hs-on-mouseenter",
            Self::MouseLeave => "data-hs-on-mouseleave",
            Self::Load => "data-hs-on-load",
        }
    }
}

/// A key that [`El::on_key`] filters `keydown` by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Key {
    /// Enter or Return.
    Enter,
    /// Escape.
    Escape,
    /// Tab.
    Tab,
    /// The space bar.
    Space,
    /// Backspace.
    Backspace,
    /// Delete.
    Delete,
    /// Up arrow.
    ArrowUp,
    /// Down arrow.
    ArrowDown,
    /// Left arrow.
    ArrowLeft,
    /// Right arrow.
    ArrowRight,
    /// A key that produces this character, as `KeyboardEvent.key` reports it.
    Char(char),
}

impl Key {
    fn name(self) -> String {
        match self {
            Self::Enter => "Enter".to_owned(),
            Self::Escape => "Escape".to_owned(),
            Self::Tab => "Tab".to_owned(),
            Self::Space => " ".to_owned(),
            Self::Backspace => "Backspace".to_owned(),
            Self::Delete => "Delete".to_owned(),
            Self::ArrowUp => "ArrowUp".to_owned(),
            Self::ArrowDown => "ArrowDown".to_owned(),
            Self::ArrowLeft => "ArrowLeft".to_owned(),
            Self::ArrowRight => "ArrowRight".to_owned(),
            Self::Char(c) => c.to_string(),
        }
    }
}

/// A field type that a form control can be bound to with [`El::bind`].
///
/// `String` binds to the value of a text field, `<textarea>`, or `<select>`,
/// `bool` to `checked`, and `i32` and `f64` to a number or range field.
#[diagnostic::on_unimplemented(
    message = "a form control cannot be bound to a `{Self}` signal",
    note = "bind `String`, `bool`, `i32`, or `f64`; for other types use `bind_select` or `bind_radio`"
)]
pub trait Bindable: sealed::Sealed {}
impl Bindable for String {}
impl Bindable for bool {}
impl Bindable for i32 {}
impl Bindable for f64 {}

mod sealed {
    pub trait Sealed {
        const KIND: &'static str;
    }
    impl Sealed for String {
        const KIND: &'static str = "text";
    }
    impl Sealed for bool {
        const KIND: &'static str = "bool";
    }
    impl Sealed for i32 {
        const KIND: &'static str = "int";
    }
    impl Sealed for f64 {
        const KIND: &'static str = "float";
    }
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

macro_rules! void_tags {
    ($($name:ident),* $(,)?) => {$(
        #[doc = concat!("Creates a `<", stringify!($name), ">` element, which has no children.")]
        #[must_use]
        pub fn $name<S>() -> El<S> {
            El::new(stringify!($name), true)
        }
    )*};
}

tags!(
    a, article, aside, b, button, code, details, dialog, div, em, fieldset, footer, form, h1, h2,
    h3, h4, header, i, label, legend, li, main, nav, ol, option, p, pre, progress, section, select,
    small, span, strong, summary, table, tbody, td, textarea, th, thead, time, tr, ul,
);

void_tags!(br, hr, img, input);

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
            checked: None,
            bind: None,
            class_when: Vec::new(),
            attr_bind: Vec::new(),
            attr_when: Vec::new(),
            style: Vec::new(),
            key: None,
            node_ref: None,
            effects: Vec::new(),
            every: None,
            on: BTreeMap::new(),
            keys: Vec::new(),
            children: Vec::new(),
        }
    }

    fn check_attr_name(name: &str) {
        assert!(
            !name.starts_with(RESERVED_PREFIX),
            "`{name}` is written by hypersig's typed methods, not by `attr`"
        );
        assert!(
            name != "id" && name != "class",
            "use `El::id` or `El::class` to set `{name}`"
        );
    }

    /// Sets a plain attribute.
    ///
    /// # Panics
    ///
    /// Panics if `name` is `id` or `class`, which have their own methods, or
    /// starts with `data-hs-`, which only the typed methods may write.
    #[must_use]
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        Self::check_attr_name(name);
        self.attrs.push((name, value.into()));
        self
    }

    /// Keeps attribute `name` equal to `value` as signals change.
    ///
    /// # Panics
    ///
    /// Panics for the same names as [`El::attr`].
    #[must_use]
    pub fn bind_attr(mut self, name: &'static str, value: impl IntoExpr<S, String>) -> Self {
        Self::check_attr_name(name);
        self.attr_bind.push((name, value.into_expr()));
        self
    }

    /// Sets the boolean attribute `name`, such as `disabled`, while `when` is true.
    ///
    /// # Panics
    ///
    /// Panics for the same names as [`El::attr`].
    #[must_use]
    pub fn attr_when(mut self, name: &'static str, when: impl IntoExpr<S, bool>) -> Self {
        Self::check_attr_name(name);
        self.attr_when.push((name, when.into_expr()));
        self
    }

    /// Keeps the CSS property `property` equal to `value` as signals change.
    #[must_use]
    pub fn bind_style(mut self, property: &'static str, value: impl IntoExpr<S, String>) -> Self {
        self.style.push((property, value.into_expr()));
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

    /// Identifies this element among the items of an [`each`] by `key`.
    ///
    /// Only the root element of an `each` template reads its key.
    #[must_use]
    pub fn key<K>(mut self, key: impl IntoExpr<S, K>) -> Self {
        self.key = Some(key.into_expr().into_json());
        self
    }

    /// Attaches `node_ref`, so its statements act on this element.
    #[must_use]
    pub fn node_ref(mut self, node_ref: NodeRef<S>) -> Self {
        self.node_ref = Some(node_ref.name());
        self
    }

    /// Appends a child element, text, or template.
    #[must_use]
    pub fn child(mut self, child: impl Into<Node<S>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends several children. An `Option` counts as zero or one child.
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

    /// Keeps a checkbox's `checked` state equal to `when`, in one direction.
    ///
    /// Use it for a checkbox whose state is derived, such as "all done".
    /// A checkbox that edits a signal uses [`El::bind`] instead.
    #[must_use]
    pub fn bind_checked(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.checked = Some(when.into_expr());
        self
    }

    /// Binds a form control to a signal in both directions.
    ///
    /// The signal can be a page signal or a field of a list item.
    #[must_use]
    pub fn bind<T: Bindable, R>(mut self, signal: Sig<S, T, R>) -> Self {
        self.bind = Some(json!([signal.raw_path().place_json(), T::KIND]));
        self
    }

    /// Makes this radio button set `signal` to `value` when checked, and
    /// checks it while `signal` equals `value`.
    ///
    /// # Panics
    ///
    /// Panics if `value` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn bind_radio<T: Serialize + PartialEq, R>(
        mut self,
        signal: Sig<S, T, R>,
        value: &T,
    ) -> Self {
        let value = serde_json::to_value(value).expect("a radio value serializes to JSON");
        self.attrs.push(("type", "radio".to_owned()));
        self.bind = Some(json!([signal.raw_path().place_json(), "radio", value]));
        self
    }

    /// Fills this `<select>` with `options` and binds it to `signal`.
    ///
    /// Each option is a value and its label. The option values are written as
    /// JSON, so `T` can be an enum.
    ///
    /// # Panics
    ///
    /// Panics if a value fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn bind_select<T: Serialize + PartialEq, R, L: Into<String>>(
        mut self,
        signal: Sig<S, T, R>,
        options: impl IntoIterator<Item = (T, L)>,
    ) -> Self {
        for (value, label) in options {
            let value = serde_json::to_string(&value).expect("an option value serializes to JSON");
            self.children
                .push(option().attr("value", value).text(label).into());
        }
        self.bind = Some(json!([signal.raw_path().place_json(), "json"]));
        self
    }

    /// Runs `stmt` on `event`. A second call for the same event runs after the first.
    #[must_use]
    pub fn on(mut self, event: Event, stmt: impl IntoStmt<S>) -> Self {
        let stmt = match self.on.remove(&event) {
            Some(existing) => existing.then(stmt),
            None => stmt.into_stmt(),
        };
        self.on.insert(event, stmt);
        self
    }

    /// Runs `stmt` once `event` has stopped firing for `quiet`.
    ///
    /// Use it for search-as-you-type, so a server action runs once per pause
    /// rather than once per keystroke.
    #[must_use]
    pub fn on_debounced(self, event: Event, quiet: Duration, stmt: impl IntoStmt<S>) -> Self {
        let wrapped = Stmt::new(json!([
            "debounce",
            quiet.as_millis(),
            stmt.into_stmt().into_json()
        ]));
        self.on(event, wrapped)
    }

    /// Runs `stmt` on click.
    #[must_use]
    pub fn on_click(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Click, stmt)
    }

    /// Runs `stmt` on double click.
    #[must_use]
    pub fn on_double_click(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::DoubleClick, stmt)
    }

    /// Runs `stmt` when a form is submitted, after `preventDefault`.
    #[must_use]
    pub fn on_submit(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Submit, stmt)
    }

    /// Runs `stmt` on every edit of a text field.
    #[must_use]
    pub fn on_input(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Input, stmt)
    }

    /// Runs `stmt` on `change`.
    #[must_use]
    pub fn on_change(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Change, stmt)
    }

    /// Runs `stmt` when focus leaves the element.
    #[must_use]
    pub fn on_blur(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Blur, stmt)
    }

    /// Runs `stmt` once when the element enters the document.
    #[must_use]
    pub fn on_load(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Load, stmt)
    }

    /// Runs `stmt` when `key` is pressed while focus is in this element.
    ///
    /// The runtime calls `preventDefault` for a matched key.
    #[must_use]
    pub fn on_key(mut self, key: Key, stmt: impl IntoStmt<S>) -> Self {
        self.keys.push((key, stmt.into_stmt()));
        self
    }

    /// Runs `stmt` every `period` while the element is in the document.
    ///
    /// A second call replaces the first.
    #[must_use]
    pub fn every(mut self, period: Duration, stmt: impl IntoStmt<S>) -> Self {
        self.every = Some((period.as_millis(), stmt.into_stmt()));
        self
    }

    /// Runs `stmt` when the element enters the document and again whenever
    /// the value of `deps` changes.
    ///
    /// This is React's `useEffect` with a dependency list: pass a tuple-like
    /// expression such as a signal, or a record built with
    /// [`Expr::with`](crate::Expr::with), as `deps`.
    #[must_use]
    pub fn effect<T>(mut self, deps: impl IntoExpr<S, T>, stmt: impl IntoStmt<S>) -> Self {
        self.effects
            .push((deps.into_expr().into_json(), stmt.into_stmt()));
        self
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
        if let Some(checked) = &self.checked {
            write_attr(out, "data-hs-checked", &checked.to_json());
        }
        if let Some(bind) = &self.bind {
            write_attr(out, "data-hs-bind", &bind.to_string());
        }
        write_pairs(out, "data-hs-class", &self.class_when);
        write_pairs(out, "data-hs-attr", &self.attr_bind);
        write_pairs(out, "data-hs-toggle", &self.attr_when);
        write_pairs(out, "data-hs-style", &self.style);
        if let Some(key) = &self.key {
            write_attr(out, "data-hs-key", &key.to_string());
        }
        if let Some(name) = self.node_ref {
            write_attr(out, "data-hs-ref", name);
        }
        if !self.effects.is_empty() {
            let effects: Vec<Value> = self
                .effects
                .iter()
                .map(|(deps, stmt)| json!([deps, stmt.clone().into_json()]))
                .collect();
            write_attr(out, "data-hs-effect", &Value::from(effects).to_string());
        }
        if let Some((period, stmt)) = &self.every {
            let every = json!([
                u64::try_from(*period).unwrap_or(u64::MAX),
                stmt.clone().into_json()
            ]);
            write_attr(out, "data-hs-every", &every.to_string());
        }
        for (event, stmt) in &self.on {
            write_attr(out, event.attribute(), &stmt.to_json());
        }
        if !self.keys.is_empty() {
            let keys: Vec<Value> = self
                .keys
                .iter()
                .map(|(key, stmt)| json!([key.name(), stmt.clone().into_json()]))
                .collect();
            write_attr(out, "data-hs-on-key", &Value::from(keys).to_string());
        }
        out.push('>');
        if self.void {
            return;
        }
        for child in &self.children {
            child.write(out);
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

fn write_pairs<S, T>(out: &mut String, name: &str, pairs: &[(&'static str, Expr<S, T>)]) {
    if pairs.is_empty() {
        return;
    }
    let pairs: Vec<Value> = pairs
        .iter()
        .map(|(key, expr)| json!([key, expr.clone().into_json()]))
        .collect();
    write_attr(out, name, &Value::from(pairs).to_string());
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
        out.push_str("<link rel=\"icon\" href=\"data:,\">");
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
            child.write(&mut out);
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
                r#"<input data-hs-bind="[&quot;name&quot;,&quot;text&quot;]">"#,
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
    fn nested_lists_render_the_same_twice() {
        use crate::html::{each, li, ul};

        struct Row;
        fn rows() -> Sig<Page, Vec<Row>> {
            sig("rows")
        }
        fn cells() -> Sig<Row, Vec<String>> {
            sig("cells")
        }
        let build = || {
            ul::<Page>()
                .child(each(rows(), |row| {
                    li().key(row.index())
                        .child(each(row.at(cells()), |cell| span().bind_text(cell)))
                }))
                .render()
        };
        let html = build();
        assert_eq!(html, build());
        assert!(
            html.contains(r#"data-hs-each="[&quot;rows&quot;,0]""#),
            "{html}"
        );
        assert!(
            html.contains(r#"data-hs-each="[[&quot;item&quot;,0,&quot;cells&quot;],1]""#),
            "{html}"
        );
        assert!(
            html.contains(r#"data-hs-text="[&quot;item&quot;,1,&quot;&quot;]""#),
            "{html}"
        );
    }

    #[test]
    #[should_panic(expected = "typed methods")]
    fn reserved_attribute_panics() {
        let _ = div::<Page>().attr("data-hs-show", "true");
    }
}
