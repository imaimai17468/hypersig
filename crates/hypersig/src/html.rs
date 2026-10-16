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
use std::fmt;
use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};

use crate::action::NodeRef;
use crate::expr::{Expr, IntoExpr, IntoStmt, Stmt};
use crate::signal::{Item, Sig, in_item_scope};
use crate::ssr::{self, Ctx};
use crate::style::{Property, Sheet, Style, Token, TokenValue};

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
    /// Adds the rules the elements in this node need to `sheet`.
    pub(crate) fn collect_css(&self, sheet: &mut Sheet) {
        match &self.0 {
            NodeKind::Element(el) => el.collect_css(sheet),
            NodeKind::Text(_) => {}
            NodeKind::Each { template, .. } | NodeKind::When { template, .. } => {
                template.collect_css(sheet);
            }
        }
    }

    pub(crate) fn write(&self, out: &mut String, ctx: Option<&Ctx<'_>>) {
        match &self.0 {
            NodeKind::Element(el) => el.write(out, ctx, None),
            NodeKind::Text(text) => escape_into(out, text, false),
            NodeKind::Each {
                list,
                depth,
                template,
            } => {
                out.push_str("<template");
                if !ctx.is_some_and(|ctx| ctx.copy) {
                    write_attr(out, "data-hs-each", &json!([list, depth]).to_string());
                }
                out.push('>');
                template.write(out, None, None);
                out.push_str("</template>");
                if let Some(ctx) = ctx {
                    template.write_items(out, ctx, list, usize::from(*depth));
                }
            }
            NodeKind::When { cond, template } => {
                out.push_str("<template");
                if !ctx.is_some_and(|ctx| ctx.copy) {
                    write_attr(out, "data-hs-if", &cond.to_json());
                }
                out.push('>');
                template.write(out, None, None);
                out.push_str("</template>");
                if let Some(ctx) = ctx
                    && ssr::evaluate(cond.as_json(), ctx).is_some_and(|v| ssr::truthy(&v))
                {
                    template.write(out, Some(&ctx.as_copy()), Some(""));
                }
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
/// [`Item`] that stands for whichever item the browser is rendering. Items are
/// matched to their elements by position. Use [`each_keyed`] where the list is
/// reordered or an item before another is removed.
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
///     li().bind_text(todo.at(Todo::title()))
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

/// [`each`], with each item's element identified by `key`.
///
/// An item keeps its element when the list is reordered or an item before it
/// is removed, so focus and the state of its inputs move with it.
///
/// ```
/// use hypersig::html::{each_keyed, li, ul};
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
/// let list = ul::<Todos>().child(each_keyed(
///     Todos::items(),
///     |todo| todo.at(Todo::id()),
///     |todo| li().bind_text(todo.at(Todo::title())),
/// ));
/// assert!(list.render().contains("data-hs-key"));
/// ```
#[must_use]
pub fn each_keyed<S, T, R, K, E: IntoExpr<S, K>>(
    list: Sig<S, Vec<T>, R>,
    key: impl FnOnce(Item<S, T>) -> E,
    render: impl FnOnce(Item<S, T>) -> El<S>,
) -> Node<S> {
    each(list, |item| {
        let mut template = render(item);
        template.key = Some(key(item).into_expr().into_json());
        template
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
    css: Style,
    css_names: Vec<String>,
    css_when: Vec<(Style, Vec<String>, Expr<S, bool>)>,
    attr_bind: Vec<(&'static str, Expr<S, String>)>,
    attr_when: Vec<(&'static str, Expr<S, bool>)>,
    style: Vec<(String, Expr<S, String>)>,
    key: Option<Value>,
    node_ref: Option<&'static str>,
    effects: Vec<Stmt<S>>,
    watches: Vec<(Value, Stmt<S>)>,
    interval: Option<(u128, Stmt<S>)>,
    on: BTreeMap<Event, Stmt<S>>,
    keys: Vec<(Key, Stmt<S>)>,
    roving: Option<Orientation>,
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

/// CSS text whose class selectors [`Stylesheet::class`] checks at compile time.
///
/// ```
/// use hypersig::html::{Class, Stylesheet, li};
/// use hypersig::{Page, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Todo {
///     done: bool,
/// }
///
/// const STYLE: Stylesheet = Stylesheet::new(".item { padding: 4px } .item.done { color: gray }");
/// const ITEM: Class = STYLE.class("item");
/// const DONE: Class = STYLE.class("done");
///
/// let html = Page::new("Todo", Todo { done: true })
///     .style(STYLE)
///     .child(li().class(ITEM).class_when(DONE, Todo::done()))
///     .render();
/// assert!(html.contains(r#"<li class="item done""#));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stylesheet(&'static str);

impl Stylesheet {
    /// Wraps the CSS text `css`, usually from `include_str!`.
    #[must_use]
    pub const fn new(css: &'static str) -> Self {
        Self(css)
    }

    /// The CSS text.
    #[must_use]
    pub const fn css(self) -> &'static str {
        self.0
    }

    /// The class `name`, which a selector in this stylesheet names as `.name`.
    ///
    /// The search skips comments and quoted strings, and reads a backslash
    /// escape such as `.w-1\/2` as the character it escapes. It does not read
    /// hex escapes, so a class the stylesheet writes only as `.\31 0` is not
    /// found. A `.name` inside an unquoted `url(...)` counts as found.
    ///
    /// # Panics
    ///
    /// Panics if no selector names `.name`, or for the names [`Class::new`]
    /// rejects. In a `const`, that is a compile error.
    #[must_use]
    pub const fn class(self, name: &'static str) -> Class {
        let class = Class::new(name);
        assert!(
            selects(self.0.as_bytes(), name.as_bytes()),
            "the stylesheet has no selector for this class"
        );
        class
    }
}

/// Whether `css` has a class selector `.name` outside comments and strings.
const fn selects(css: &[u8], name: &[u8]) -> bool {
    let mut i = 0;
    while i < css.len() {
        match css[i] {
            b'/' if i + 1 < css.len() && css[i + 1] == b'*' => {
                i += 2;
                while i + 1 < css.len() && !(css[i] == b'*' && css[i + 1] == b'/') {
                    i += 1;
                }
                i += 2;
            }
            quote @ (b'"' | b'\'') => {
                i += 1;
                while i < css.len() && css[i] != quote {
                    i += if css[i] == b'\\' { 2 } else { 1 };
                }
                i += 1;
            }
            b'\\' => i += 2,
            // A dot after a digit is a decimal point, as in `1.5rem`.
            b'.' if i == 0 || !css[i - 1].is_ascii_digit() => {
                if let Some(end) = match_ident(css, i + 1, name)
                    && (end == css.len() || !is_ident_byte(css[end]))
                {
                    return true;
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    false
}

/// The index just past `name` when the identifier at `css[at..]` spells it.
const fn match_ident(css: &[u8], at: usize, name: &[u8]) -> Option<usize> {
    let (mut i, mut j) = (at, 0);
    while j < name.len() {
        if i < css.len() && css[i] == b'\\' {
            i += 1;
        }
        if i >= css.len() || css[i] != name[j] {
            return None;
        }
        i += 1;
        j += 1;
    }
    Some(i)
}

/// Whether `b` continues a CSS identifier: an ASCII letter, digit, `-`, `_`,
/// an escape, or a byte of a non-ASCII character.
const fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'\\' || b >= 0x80
}

/// A class name that [`El::class`] and [`El::class_when`] put on an element.
///
/// Declare each class once as a `const` and use the constant everywhere, so a
/// misspelled class at a use site is an unknown name to the compiler. A class
/// from [`Stylesheet::class`] is also checked against the stylesheet. One from
/// [`Class::new`] is not, which suits a stylesheet the page loads from a URL.
///
/// ```
/// use hypersig::html::{Class, li};
/// use hypersig::{Sig, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Todo {
///     done: bool,
/// }
///
/// const ITEM: Class = Class::new("item");
/// const DONE: Class = Class::new("done");
/// let html = li::<Todo>().class(ITEM).class_when(DONE, Todo::done()).render();
/// assert!(html.starts_with(r#"<li class="item""#));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Class(&'static str);

impl Class {
    /// A class named `name`, unchecked against any stylesheet.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty or contains ASCII whitespace. In a `const`,
    /// that is a compile error.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        // The `class` attribute splits on ASCII whitespace, and
        // `classList.toggle` throws on a token that contains it.
        let bytes = name.as_bytes();
        assert!(!bytes.is_empty(), "a class name is never empty");
        let mut i = 0;
        while i < bytes.len() {
            assert!(
                !bytes[i].is_ascii_whitespace(),
                "a class name holds no whitespace; declare one `Class` per name"
            );
            i += 1;
        }
        Self(name)
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
    /// Home.
    Home,
    /// End.
    End,
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
            Self::Home => "Home".to_owned(),
            Self::End => "End".to_owned(),
            Self::Char(c) => c.to_string(),
        }
    }
}

/// A horizontal or vertical direction, which [`El::roving_focus`] reads as its arrow keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Orientation {
    /// Left and right, swapped where the element's `direction` is `rtl`.
    Horizontal,
    /// Up and down.
    Vertical,
}

impl Orientation {
    fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
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
            css: Style::new(),
            css_names: Vec::new(),
            css_when: Vec::new(),
            attr_bind: Vec::new(),
            attr_when: Vec::new(),
            style: Vec::new(),
            key: None,
            node_ref: None,
            effects: Vec::new(),
            watches: Vec::new(),
            interval: None,
            on: BTreeMap::new(),
            keys: Vec::new(),
            roving: None,
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
    ///
    /// The value is a string the browser parses, so nothing checks that it is
    /// valid for the property. A value the browser rejects leaves the property
    /// as the element's classes and stylesheets set it. Where the value comes
    /// from a fixed set, [`El::css_match`] keeps it typed.
    ///
    /// ```
    /// use hypersig::html::div;
    /// use hypersig::style::Property;
    /// use hypersig::Signals;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Meter {
    ///     level: i32,
    /// }
    ///
    /// let bar = div::<Meter>().bind_style(Property::Width, Meter::level().to_text().concat("%"));
    /// assert!(bar.render().contains("data-hs-style"));
    /// ```
    #[must_use]
    pub fn bind_style(mut self, property: Property, value: impl IntoExpr<S, String>) -> Self {
        self.style
            .push((property.name().to_owned(), value.into_expr()));
        self
    }

    /// Keeps `token` equal to `value` on this element and those inside it, as
    /// signals change. The value is not checked, as for [`El::bind_style`].
    #[must_use]
    pub fn bind_token<T: TokenValue>(
        mut self,
        token: &Token<T>,
        value: impl IntoExpr<S, String>,
    ) -> Self {
        self.style
            .push((format!("--{}", token.name()), value.into_expr()));
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
    pub fn class(mut self, class: Class) -> Self {
        self.classes.push(class.0);
        self
    }

    /// Adds `class` while `when` is true.
    #[must_use]
    pub fn class_when(mut self, class: Class, when: impl IntoExpr<S, bool>) -> Self {
        self.class_when.push((class.0, when.into_expr()));
        self
    }

    /// Adds the declarations of `style` as atomic classes.
    ///
    /// The page writes the rules for them into `<head>`, and a patch carries
    /// the rules for the fragment it sends. A later call wins over an earlier
    /// one for the same property, as [`Style::merge`] does. A plain stylesheet
    /// from [`Page::style`](crate::Page::style) sits outside the cascade layers
    /// these rules are in, so it overrides them.
    ///
    /// ```
    /// use hypersig::html::button;
    /// use hypersig::style::value::{Color, Cursor, px};
    /// use hypersig::style::{PseudoClass, Style};
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Shop {}
    ///
    /// let html = Page::new("Shop", Shop {})
    ///     .child(button().css(
    ///         Style::new()
    ///             .padding(px(8))
    ///             .cursor(Cursor::Pointer)
    ///             .on(PseudoClass::Hover, Style::new().color(Color::CRIMSON)),
    ///     ))
    ///     .render();
    /// assert!(html.contains(r#"<button class="padding_8px cursor_pointer hover:color_crimson">"#));
    /// assert!(html.contains(".hover\\:color_crimson:hover{color:crimson}"));
    /// ```
    #[must_use]
    pub fn css(mut self, style: Style) -> Self {
        self.css = std::mem::take(&mut self.css).merge(style);
        self.css_names = self.css.class_names().collect();
        self
    }

    /// Adds the declarations of `style` while `when` is true.
    ///
    /// Their rules sit in a cascade layer after those of [`El::css`], so while
    /// `when` is true they override this element's own declarations for the
    /// same property. Two styles switched on at once that set the same
    /// property are not checked against each other: the one whose rule comes
    /// later in the stylesheet wins, which depends on the conditions and the
    /// property, not on the order of the calls.
    #[must_use]
    pub fn css_when(mut self, style: Style, when: impl IntoExpr<S, bool>) -> Self {
        let names = style.toggled_class_names().collect();
        self.css_when.push((style, names, when.into_expr()));
        self
    }

    /// Adds the style of the case whose value `value` equals, as
    /// [`El::css_when`] does for each case.
    ///
    /// This is the variant half of a recipe: the base goes in [`El::css`], and
    /// each case adds what one value changes. At most one case applies at a
    /// time, so two cases never compete for a property. A value with no case
    /// adds nothing, and nothing checks that the cases cover every value of `T`.
    /// Where the value is known on the server, call a function that returns a
    /// [`Style`] with a `match` instead, which the compiler checks for
    /// exhaustiveness.
    ///
    /// ```
    /// use hypersig::html::button;
    /// use hypersig::style::Style;
    /// use hypersig::style::value::{Color, px};
    /// use hypersig::{Page, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
    /// enum Tone {
    ///     Calm,
    ///     Loud,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Alert {
    ///     tone: Tone,
    /// }
    ///
    /// let html = Page::new("Alert", Alert { tone: Tone::Loud })
    ///     .child(button().css(Style::new().padding(px(8))).css_match(
    ///         Alert::tone(),
    ///         [
    ///             (Tone::Calm, Style::new().color(Color::GRAY)),
    ///             (Tone::Loud, Style::new().color(Color::CRIMSON)),
    ///         ],
    ///     ))
    ///     .render();
    /// assert!(html.contains(r#"class="padding_8px when:color_crimson""#));
    /// ```
    #[must_use]
    pub fn css_match<T: PartialEq + Serialize>(
        mut self,
        value: impl IntoExpr<S, T>,
        cases: impl IntoIterator<Item = (T, Style)>,
    ) -> Self {
        let value = value.into_expr();
        for (case, style) in cases {
            self = self.css_when(style, value.clone().equals(&case));
        }
        self
    }

    /// Adds the rules this element and the elements inside it need to `sheet`.
    pub(crate) fn collect_css(&self, sheet: &mut Sheet) {
        sheet.add(&self.css);
        for (style, _, _) in &self.css_when {
            sheet.add_toggled(style);
        }
        for child in &self.children {
            child.collect_css(sheet);
        }
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

    /// Moves focus between the items inside this element on the arrow keys,
    /// Home, and End.
    ///
    /// An item is an element inside this one that has a `tabindex` attribute
    /// and is neither disabled nor `aria-disabled="true"`, unless a nearer
    /// element inside this one is also marked, in which case the item belongs
    /// to that one. The two arrow keys of `orientation` move to the next and
    /// the previous item, wrapping at either end, and Home and End move to the
    /// first and the last. The runtime calls `preventDefault` for a key it
    /// acts on, and leaves a key alone when a handler from [`El::on_key`]
    /// already took it or Alt, Control, or Meta is held.
    ///
    /// The runtime moves focus and changes nothing else, so each item keeps
    /// the `tabindex` it is given. A tab list sets `tabindex="0"` on the
    /// selected tab and `-1` on the others with [`El::bind_attr`], so that Tab
    /// enters the list at the selected tab, and selects a tab when it gains
    /// focus with [`Event::Focus`].
    ///
    /// ```
    /// use hypersig::html::{Event, Orientation, button, div};
    /// use hypersig::{Signals, if_else};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Tabs {
    ///     tab: i32,
    /// }
    ///
    /// let tab = Tabs::tab();
    /// let list = div::<Tabs>().attr("role", "tablist").roving_focus(Orientation::Horizontal).children(
    ///     (0..3).map(|i| {
    ///         button()
    ///             .attr("role", "tab")
    ///             .bind_attr("tabindex", if_else(tab.equals(i), "0", "-1"))
    ///             .on(Event::Focus, tab.set(i))
    ///     }),
    /// );
    /// assert!(list.render().contains(r#"data-hs-roving="horizontal""#));
    /// ```
    #[must_use]
    pub fn roving_focus(mut self, orientation: Orientation) -> Self {
        self.roving = Some(orientation);
        self
    }

    /// Runs `stmt` every `period` while the element is in the document.
    ///
    /// A second call replaces the first.
    #[must_use]
    pub fn on_interval(mut self, period: Duration, stmt: impl IntoStmt<S>) -> Self {
        self.interval = Some((period.as_millis(), stmt.into_stmt()));
        self
    }

    /// Runs `stmt` when the element enters the document and again whenever
    /// a signal that `stmt` reads changes.
    ///
    /// The runtime records which signals `stmt` reads as it runs, so there is
    /// no dependency list to keep in step. A signal read only after an
    /// action's response arrives is not recorded.
    #[must_use]
    pub fn effect(mut self, stmt: impl IntoStmt<S>) -> Self {
        self.effects.push(stmt.into_stmt());
        self
    }

    /// Runs `stmt` whenever `signal`, or any field inside it, changes.
    ///
    /// Use it where `stmt` does not read `signal` itself, such as an action
    /// call that saves a list each time the list changes.
    #[must_use]
    pub fn on_signal_change<T, R>(mut self, signal: Sig<S, T, R>, stmt: impl IntoStmt<S>) -> Self {
        self.watches
            .push((signal.raw_path().place_json(), stmt.into_stmt()));
        self
    }

    pub(crate) fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_owned());
        self
    }

    /// Renders the element and its children to an HTML string.
    ///
    /// The reactive parts are left for the runtime to fill in, because an
    /// element alone has no signal values. [`Page::render`](crate::Page::render) fills them in.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, None, None);
        out
    }

    /// Writes one copy of this template per item of `list`, each marked with
    /// the key the runtime will compute for it, so the runtime adopts them.
    fn write_items(&self, out: &mut String, ctx: &Ctx<'_>, list: &Value, depth: usize) {
        let Some(path) = ctx.resolve(list) else {
            return;
        };
        let len = ctx.get(&path).as_array().map_or(0, Vec::len);
        for index in 0..len {
            let item = ctx.with_item(depth, ssr::join(&path, &index.to_string()));
            let key = match &self.key {
                None => format!("#{index}"),
                Some(key) => match ssr::evaluate(key, &item) {
                    Some(value) => ssr::js_json(&value),
                    None => continue,
                },
            };
            self.write(out, Some(&item), Some(&key));
        }
    }

    fn has_attr(&self, name: &str) -> bool {
        self.attrs.iter().any(|(attr, _)| *attr == name)
    }

    /// Writes the element. With `ctx`, reactive attributes and text are
    /// computed from the signals as well. `ssr_key` marks the root of a list
    /// item or conditional copy rendered on the server.
    #[expect(
        clippy::too_many_lines,
        reason = "one attribute after another, in the order the runtime documents them"
    )]
    fn write(&self, out: &mut String, ctx: Option<&Ctx<'_>>, ssr_key: Option<&str>) {
        let eval_text =
            |expr: &Value| ctx.and_then(|ctx| ssr::evaluate(expr, ctx).map(|v| ssr::js_string(&v)));
        let eval_bool =
            |expr: &Value| ctx.and_then(|ctx| ssr::evaluate(expr, ctx).map(|v| ssr::truthy(&v)));
        let mut classes: Vec<&str> = self.classes.clone();
        for (class, when) in &self.class_when {
            if eval_bool(when.as_json()) == Some(true) {
                classes.push(class);
            }
        }
        classes.extend(self.css_names.iter().map(String::as_str));
        for (_, names, when) in &self.css_when {
            if eval_bool(when.as_json()) == Some(true) {
                classes.extend(names.iter().map(String::as_str));
            }
        }
        let mut style: Vec<String> = Vec::new();
        for (property, value) in &self.style {
            if let Some(value) = eval_text(value.as_json()) {
                style.push(format!("{property}: {value}"));
            }
        }
        if let Some(show) = &self.show
            && eval_bool(show.as_json()) == Some(false)
        {
            style.push("display: none".to_owned());
        }
        let mut computed: Vec<(&str, String)> = Vec::new();
        for (name, value) in &self.attr_bind {
            if let Some(value) = eval_text(value.as_json()) {
                computed.push((name, value));
            }
        }
        for (name, when) in &self.attr_when {
            if eval_bool(when.as_json()) == Some(true) {
                computed.push((name, String::new()));
            }
        }
        if let Some(checked) = &self.checked
            && eval_bool(checked.as_json()) == Some(true)
        {
            computed.push(("checked", String::new()));
        }
        let mut content: Option<String> = self
            .text
            .as_ref()
            .and_then(|text| eval_text(text.as_json()));
        let mut selected: Option<String> = None;
        if let (Some(ctx), Some(bind)) = (ctx, &self.bind)
            && let Some(path) = bind.get(0).and_then(|place| ctx.resolve(place))
        {
            let value = ctx.get(&path);
            match bind.get(1).and_then(Value::as_str) {
                Some("bool") if ssr::truthy(value) => computed.push(("checked", String::new())),
                Some("radio") if bind.get(2).is_some_and(|choice| ssr::equal(choice, value)) => {
                    computed.push(("checked", String::new()));
                }
                Some("json") => selected = Some(ssr::js_json(value)),
                Some("text" | "int" | "float") if !value.is_null() => {
                    let text = ssr::js_string(value);
                    match self.tag {
                        "textarea" => content = Some(text),
                        "select" => selected = Some(text),
                        _ => computed.push(("value", text)),
                    }
                }
                _ => {}
            }
        }

        out.push('<');
        out.push_str(self.tag);
        if let Some(id) = &self.id {
            write_attr(out, "id", id);
        }
        if !classes.is_empty() {
            write_attr(out, "class", &classes.join(" "));
        }
        for (name, value) in &self.attrs {
            if *name == "style" && !style.is_empty() {
                continue;
            }
            write_attr(out, name, value);
        }
        if !style.is_empty() {
            let own = self.attrs.iter().find(|(name, _)| *name == "style");
            let mut all: Vec<String> = own.map(|(_, value)| value.clone()).into_iter().collect();
            all.extend(style);
            write_attr(out, "style", &all.join("; "));
        }
        for (name, value) in &computed {
            if !self.has_attr(name) {
                write_attr(out, name, value);
            }
        }
        if let Some(key) = ssr_key {
            write_attr(out, "data-hs-ssr", key);
        }
        if !ctx.is_some_and(|ctx| ctx.copy) {
            self.write_bindings(out);
        }
        out.push('>');
        if self.void {
            return;
        }
        if let Some(text) = content {
            escape_into(out, &text, false);
        } else {
            for child in &self.children {
                match (&child.0, &selected) {
                    (NodeKind::Element(option), Some(value))
                        if option.tag == "option"
                            && option
                                .attrs
                                .iter()
                                .any(|(name, v)| *name == "value" && v == value) =>
                    {
                        option.write_selected(out, ctx);
                    }
                    _ => child.write(out, ctx),
                }
            }
        }
        out.push_str("</");
        out.push_str(self.tag);
        out.push('>');
    }

    /// Writes the `data-hs-*` attributes the runtime builds this element's bindings from.
    fn write_bindings(&self, out: &mut String) {
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
        if !self.class_when.is_empty() || !self.css_when.is_empty() {
            let mut pairs: Vec<(&str, &Value)> = self
                .class_when
                .iter()
                .map(|(class, when)| (*class, when.as_json()))
                .collect();
            for (_, names, when) in &self.css_when {
                pairs.extend(names.iter().map(|name| (name.as_str(), when.as_json())));
            }
            write_attr(out, "data-hs-class", &json_text(&pairs));
        }
        write_pairs(out, "data-hs-attr", &self.attr_bind);
        write_pairs(out, "data-hs-toggle", &self.attr_when);
        if !self.style.is_empty() {
            let pairs: Vec<(&str, &Value)> = self
                .style
                .iter()
                .map(|(property, value)| (property.as_str(), value.as_json()))
                .collect();
            write_attr(out, "data-hs-style", &json_text(&pairs));
        }
        if let Some(key) = &self.key {
            write_attr(out, "data-hs-key", &key.to_string());
        }
        if let Some(name) = self.node_ref {
            write_attr(out, "data-hs-ref", name);
        }
        if !self.effects.is_empty() {
            let effects: Vec<&Value> = self.effects.iter().map(Stmt::as_json).collect();
            write_attr(out, "data-hs-effect", &json_text(&effects));
        }
        if !self.watches.is_empty() {
            let watches: Vec<(&Value, &Value)> = self
                .watches
                .iter()
                .map(|(place, stmt)| (place, stmt.as_json()))
                .collect();
            write_attr(out, "data-hs-on-signal", &json_text(&watches));
        }
        if let Some((period, stmt)) = &self.interval {
            let period = u64::try_from(*period).unwrap_or(u64::MAX);
            write_attr(
                out,
                "data-hs-on-interval",
                &json_text(&(period, stmt.as_json())),
            );
        }
        for (event, stmt) in &self.on {
            write_attr(out, event.attribute(), &stmt.to_json());
        }
        if !self.keys.is_empty() {
            let keys: Vec<(String, &Value)> = self
                .keys
                .iter()
                .map(|(key, stmt)| (key.name(), stmt.as_json()))
                .collect();
            write_attr(out, "data-hs-on-key", &json_text(&keys));
        }
        if let Some(orientation) = self.roving {
            write_attr(out, "data-hs-roving", orientation.name());
        }
    }

    fn write_selected(&self, out: &mut String, ctx: Option<&Ctx<'_>>) {
        let mut out_el = String::new();
        self.write(&mut out_el, ctx, None);
        let open = format!("<{}", self.tag);
        out.push_str(&open);
        out.push_str(" selected=\"\"");
        out.push_str(&out_el[open.len()..]);
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
    let pairs: Vec<(&str, &Value)> = pairs
        .iter()
        .map(|(key, expr)| (*key, expr.as_json()))
        .collect();
    write_attr(out, name, &json_text(&pairs));
}

fn json_text(value: &impl Serialize) -> String {
    serde_json::to_string(value).expect("JSON values and strings serialize")
}

pub(crate) fn write_attr(out: &mut String, name: &str, value: &str) {
    out.push(' ');
    out.push_str(name);
    out.push_str("=\"");
    escape_into(out, value, true);
    out.push('"');
}

pub(crate) fn escape_into(out: &mut String, text: &str, attribute: bool) {
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

#[cfg(test)]
mod tests {
    use super::{Class, button, div, input, span};
    use crate::_private::sig;
    use crate::style::Token;
    use crate::{Sig, Stmt};

    struct Page;

    fn count() -> Sig<Page, i32> {
        sig("count")
    }

    fn name() -> Sig<Page, String> {
        sig("name")
    }

    #[test]
    fn finds_class_selectors_only() {
        let found = |css: &str, name: &str| super::selects(css.as_bytes(), name.as_bytes());
        assert!(found(".big { color: red }", "big"));
        assert!(found("li.done, .count.over:hover { }", "over"));
        assert!(found(".w-1\\/2 { width: 50% }", "w-1/2"));
        assert!(!found(".bigger { }", "big"));
        assert!(!found("p { margin: 1.5rem }", "5rem"));
        assert!(!found("/* .big */ p { }", "big"));
        assert!(!found("p::before { content: '.big' }", "big"));
    }

    #[test]
    fn binds_a_custom_property() {
        let accent = Token::new("accent", crate::style::value::Color::RED);
        let html = div::<Page>().bind_token(&accent, name()).render();
        assert!(
            html.contains(r#"data-hs-style="[[&quot;--accent&quot;,"#),
            "{html}"
        );
    }

    #[test]
    fn renders_typed_attributes() {
        let html = div::<Page>()
            .class(Class::new("box"))
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
        use crate::Item;
        use crate::html::{each, each_keyed, li, ul};

        struct Row;
        fn rows() -> Sig<Page, Vec<Row>> {
            sig("rows")
        }
        fn cells() -> Sig<Row, Vec<String>> {
            sig("cells")
        }
        let build = || {
            ul::<Page>()
                .child(each_keyed(rows(), Item::index, |row| {
                    li().child(each(row.at(cells()), |cell| span().bind_text(cell)))
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
