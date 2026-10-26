//! An HTML builder whose reactive attributes only accept typed values.
//!
//! Every element carries the signal struct `S` of the page it belongs to.
//! A child must share it, so a signal, expression, or action from another page
//! does not compile inside this one.
//!
//! The builder writes the `data-hs-*` attributes the runtime reads. No
//! [`Attr`] names one of them, so [`El::attr`] cannot write one by hand and
//! bypass the types.
//!
//! A component is a function that returns an [`El`]. One that takes a
//! [`Sig`] to a nested signal struct can be placed several times on a page,
//! each time with its own state:
//!
//! ```
//! use hypersig::html::{El, Role, button, div, p};
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

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};

use crate::action::NodeRef;
use crate::exact;
use crate::expr::{Expr, IntoExpr, IntoStmt, Stmt, in_let_scope};
use crate::href::{IntoHref, SafeUrl};
use crate::signal::{Item, Part, Sig, Writable, in_item_scope, in_part};
use crate::ssr::{self, Ctx, Val};
use crate::style::{Property, Sheet, Style, Token, TokenValue};
use crate::tape::Ref;

const RESERVED_PREFIX: &str = "data-hs-";

pub mod attr;
mod id;
mod role;

pub use id::Id;
pub use role::Role;

/// The name of an attribute that [`El::attr`], [`El::bind_attr`], and
/// [`El::attr_when`] write.
///
/// The [`attr`] module holds a constant for each attribute of HTML, SVG, and
/// ARIA that `Attr` can name, and [`attr!`](crate::attr!) builds one at
/// compile time for any other name, such as `data-lat` or an attribute of a
/// custom element.
///
/// An `Attr` never names an attribute that hypersig writes itself (`id`,
/// `class`, and those starting with `data-hs-`), nor `role`, which
/// [`El::role`] sets from a [`Role`], nor an attribute whose value names
/// another element, such as `for` or `aria-describedby`, which the methods
/// that take an [`Id`] set, nor one whose value can run a
/// script or load a URL: `href`, `src`, `poster`, and `srcset`, which
/// [`El::href`], [`El::src`], [`El::poster`], and [`El::srcset`] set from a
/// value that cannot, an event handler `on*`, `srcdoc`, `action`,
/// `formaction`, `xlink:href`, `data`, which `<object>` loads, `http-equiv`,
/// whose `refresh` loads the URL in `content`, `attributeName`, with which
/// SVG's `<animate>` and `<set>` could write into a link, `ping`, to which a
/// click on a link sends a request, `imagesrcset`, `background`,
/// `manifest`, `codebase`, `archive`, `classid`, and `longdesc`. Nor does it
/// name `style`, whose CSS [`El::css`] and [`El::bind_style`] write. Case is
/// ignored, as the browser ignores it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Attr(&'static str);

impl Attr {
    /// Checks `name` against the rules of [`Attr`].
    ///
    /// # Panics
    ///
    /// Panics for a name that breaks them. In a `const`, the panic is a
    /// compile error.
    pub(crate) const fn new(name: &'static str) -> Self {
        let bytes = name.as_bytes();
        // A space, a quote, `=`, or `>` would end the name and start another
        // attribute.
        assert!(
            !bytes.is_empty(),
            "an attribute name is letters, digits, `-`, `_`, `.`, and `:`"
        );
        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];
            assert!(
                b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'),
                "an attribute name is letters, digits, `-`, `_`, `.`, and `:`"
            );
            i += 1;
        }
        assert!(
            !starts_with(bytes, RESERVED_PREFIX),
            "hypersig's typed methods write the `data-hs-` attributes"
        );
        assert!(
            !is(bytes, "id") && !is(bytes, "class"),
            "`El::id` and `El::class` set `id` and `class`"
        );
        assert!(!is(bytes, "role"), "`El::role` sets `role` from a `Role`");
        assert!(
            !points_at_an_element(bytes),
            "this attribute names an element; set it with the `El` method that takes an `Id`"
        );
        assert!(
            !is(bytes, "href") && !is(bytes, "src") && !is(bytes, "poster"),
            "`El::href`, `El::src`, and `El::poster` set `href`, `src`, and `poster` from a `Link` or a `SafeUrl`"
        );
        assert!(
            !is(bytes, "srcset"),
            "`El::srcset` sets `srcset` from `SafeUrl`s"
        );
        assert!(
            !is(bytes, "style"),
            "`El::css` and `El::bind_style` write the element's CSS"
        );
        assert!(
            !starts_with(bytes, "on"),
            "an `on*` attribute runs a script; attach a handler with the `on_*` methods"
        );
        assert!(
            !is(bytes, "srcdoc")
                && !is(bytes, "action")
                && !is(bytes, "formaction")
                && !is(bytes, "xlink:href")
                && !is(bytes, "data")
                && !is(bytes, "http-equiv")
                && !is(bytes, "attributename")
                && !is(bytes, "ping")
                && !is(bytes, "imagesrcset")
                && !is(bytes, "background")
                && !is(bytes, "manifest")
                && !is(bytes, "codebase")
                && !is(bytes, "archive")
                && !is(bytes, "classid")
                && !is(bytes, "longdesc"),
            "the value of this attribute can run a script or load a URL"
        );
        Self(name)
    }

    /// The attribute's name, as the HTML holds it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.0
    }
}

/// Whether `name` is an attribute whose value is the id of another element,
/// which the `El` methods that take an [`Id`] write.
const fn points_at_an_element(name: &[u8]) -> bool {
    is(name, "for")
        || is(name, "aria-labelledby")
        || is(name, "aria-describedby")
        || is(name, "aria-controls")
        || is(name, "aria-owns")
        || is(name, "aria-details")
        || is(name, "aria-flowto")
        || is(name, "aria-errormessage")
        || is(name, "aria-activedescendant")
        || is(name, "commandfor")
        || is(name, "popovertarget")
        || is(name, "list")
        || is(name, "form")
        || is(name, "headers")
        || is(name, "itemref")
}

/// Whether `name` is `lower`, ignoring ASCII case.
const fn is(name: &[u8], lower: &str) -> bool {
    name.eq_ignore_ascii_case(lower.as_bytes())
}

/// Whether `name` starts with `lower`, ignoring ASCII case.
const fn starts_with(name: &[u8], lower: &str) -> bool {
    let prefix = lower.as_bytes();
    name.len() >= prefix.len() && name.split_at(prefix.len()).0.eq_ignore_ascii_case(prefix)
}

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
            NodeKind::Text(text) => escape_into(out, text, Escape::Text),
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

/// Panics if two elements in `nodes` carry one node ref name, one file
/// input name, or one id the program wrote, where the runtime could find
/// either, or if an element inside a list template carries a fixed id.
///
/// The runtime looks a name up in the list items and conditional copies
/// around the statement first, innermost first, and then in the whole page
/// (`#findNamed` in `runtime/src/runtime.ts`). Two elements collide when
/// they sit in one template or one of them sits in a template that encloses
/// the other, counting the page as the outermost template, because one
/// lookup can then reach both. Elements in sibling templates never meet, so
/// every item of a list may carry the same name. Ids follow the same rule,
/// except that every item of a list writes the template's id again, so a
/// fixed id inside a list template is a duplicate on its own.
pub(crate) fn check_names<S>(nodes: &[Node<S>]) {
    let mut names = Names::default();
    for node in nodes {
        names.node(node, None);
    }
    names.check();
}

/// Panics if an expression or statement in `nodes` reads a list item that no
/// list around it renders.
///
/// [`Item`] checks that its closure is still running when it is read, so the
/// item can reach elsewhere only inside an expression or statement built in
/// the closure and attached outside it. The runtime finds no item there, or
/// the item of whichever list encloses the element at that depth.
pub(crate) fn check_item_reads<S>(nodes: &[Node<S>]) {
    for node in nodes {
        node_item_reads(node, 0);
    }
}

/// Checks `node` under the lists whose depths are the bits of `open`.
fn node_item_reads<S>(node: &Node<S>, open: u128) {
    match &node.0 {
        NodeKind::Element(el) => el_item_reads(el, open),
        NodeKind::Text(_) => {}
        NodeKind::Each {
            list,
            depth,
            template,
        } => {
            assert_items_open(list, open);
            el_item_reads(template, open | depth_bit(u64::from(*depth)));
        }
        NodeKind::When { cond, template } => {
            assert_items_open(cond.as_json(), open);
            el_item_reads(template, open);
        }
    }
}

fn el_item_reads<S>(el: &El<S>, open: u128) {
    let check = |ast: &Value| assert_items_open(ast, open);
    let exprs = el
        .show
        .iter()
        .chain(&el.checked)
        .chain(el.class_when.iter().map(|(_, expr)| expr))
        .chain(el.css_when.iter().map(|(_, _, expr)| expr))
        .chain(el.attr_when.iter().map(|(_, expr)| expr));
    exprs.for_each(|expr| check(expr.as_json()));
    let texts = el
        .text
        .iter()
        .chain(el.attr_bind.iter().map(|(_, expr)| expr))
        .chain(el.style.iter().map(|(_, expr)| expr));
    texts.for_each(|expr| check(expr.as_json()));
    el.bind
        .iter()
        .chain(&el.key)
        .chain(el.watches.iter().map(|(place, _)| place))
        .chain(el.active.iter().map(|(_, place)| place))
        .for_each(check);
    let stmts = el
        .effects
        .iter()
        .chain(el.watches.iter().map(|(_, stmt)| stmt))
        .chain(el.interval.iter().map(|(_, stmt)| stmt))
        .chain(el.timeout.iter().map(|(_, stmt)| stmt))
        .chain(el.on.values())
        .chain(el.custom.iter().map(|(_, _, stmt)| stmt))
        .chain(el.keys.iter().map(|(_, stmt)| stmt));
    stmts.for_each(|stmt| check(stmt.as_json()));
    for child in &el.children {
        node_item_reads(child, open);
    }
}

/// The bit of `open` that marks the list at `depth` as open. A depth past
/// 127 has none, so it is never reported.
fn depth_bit(depth: u64) -> u128 {
    u32::try_from(depth)
        .ok()
        .and_then(|depth| 1_u128.checked_shl(depth))
        .unwrap_or(0)
}

/// Panics if `ast` reads `["item",depth,..]` or `["index",depth]` where
/// neither a list whose depth is a bit of `open` nor an operation inside
/// `ast` that runs per item binds `depth`.
pub(crate) fn assert_items_open(ast: &Value, open: u128) {
    if let Some(depth) = unbound_item(ast, open) {
        panic!(
            "an expression or statement reads the item at depth {depth} of a list that does \
             not enclose it: build it inside the closure of the `each`, or the list method, \
             that received the item, and attach it to an element that list renders"
        );
    }
}

/// The depth of the first item `ast` reads that `open` leaves unbound.
fn unbound_item(ast: &Value, open: u128) -> Option<u64> {
    let parts = ast.as_array()?;
    let Some(op) = parts.first().and_then(Value::as_str) else {
        return parts.iter().find_map(|part| unbound_item(part, open));
    };
    // The index of the depth that binds the item in the parts after it, in
    // the operations and statements that run an expression per item, which
    // `scoped` and the list statements in `signal.rs` build.
    let bound_at = match op {
        "lit" => return None,
        "item" | "index" => {
            let depth = parts.get(1)?.as_u64()?;
            let bit = depth_bit(depth);
            return (bit != 0 && open & bit == 0).then_some(depth);
        }
        "count" | "sum_of" | "max_of" | "min_of" | "map" | "retain" | "update" | "sort" => 2,
        "extend" => 3,
        _ => 0,
    };
    let rest = parts.get(1..)?;
    match parts.get(bound_at).and_then(Value::as_u64) {
        Some(depth) if bound_at > 0 => {
            let (before, after) = rest.split_at(bound_at - 1);
            let inner = open | depth_bit(depth);
            before
                .iter()
                .find_map(|part| unbound_item(part, open))
                .or_else(|| after.iter().find_map(|part| unbound_item(part, inner)))
        }
        _ => rest.iter().find_map(|part| unbound_item(part, open)),
    }
}

/// The names that elements carry, each with the templates it sits in,
/// outermost first, numbered in the order the walk enters them, and whether
/// the program wrote it as a fixed string.
#[derive(Default)]
struct Names<'a> {
    found: Vec<(&'static str, &'a str, Vec<u32>, bool)>,
    templates: u32,
}

/// The template the walk is in and the templates around it, kept on the
/// call stack so that a page whose elements carry no name allocates nothing.
struct Scope<'a> {
    template: u32,
    each: bool,
    outer: Option<&'a Scope<'a>>,
}

impl Scope<'_> {
    /// The templates from the outermost to `scope`.
    fn path(scope: Option<&Self>) -> Vec<u32> {
        let mut path = Vec::new();
        let mut at = scope;
        while let Some(scope) = at {
            path.push(scope.template);
            at = scope.outer;
        }
        path.reverse();
        path
    }
}

impl<'a> Names<'a> {
    fn el<S>(&mut self, el: &'a El<S>, scope: Option<&Scope<'_>>) {
        if let Some(name) = el.node_ref {
            self.found
                .push(("node ref name", name, Scope::path(scope), true));
        }
        if let Some(name) = el.file_input {
            self.found
                .push(("file input name", name, Scope::path(scope), true));
        }
        if let Some(id) = &el.id {
            let mut at = scope;
            while let Some(scope) = at {
                assert!(
                    !scope.each,
                    "an element inside a list template carries the fixed id `{id}`, which \
                     every item of the list repeats: bind the id to the item with `bind_attr`, \
                     or move the element out of the list"
                );
                at = scope.outer;
            }
            self.found
                .push(("id", id.as_str(), Scope::path(scope), id.fixed));
        }
        for child in &el.children {
            self.node(child, scope);
        }
    }

    fn node<S>(&mut self, node: &'a Node<S>, scope: Option<&Scope<'_>>) {
        match &node.0 {
            NodeKind::Element(el) => self.el(el, scope),
            NodeKind::Text(_) => {}
            NodeKind::Each { template, .. } | NodeKind::When { template, .. } => {
                self.templates += 1;
                let inner = Scope {
                    template: self.templates,
                    each: matches!(node.0, NodeKind::Each { .. }),
                    outer: scope,
                };
                self.el(template, Some(&inner));
            }
        }
    }

    fn check(mut self) {
        self.found
            .sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
        for group in self.found.chunk_by(|a, b| (a.0, a.1) == (b.0, b.1)) {
            // An id from `Id::from_text` may come from data a visitor stored,
            // and a panic would stop the page for every visitor until the
            // data changes, so only two ids the program wrote alike panic.
            for (i, (kind, name, a, fixed)) in group.iter().enumerate() {
                let meets = |(_, _, b, other): &(_, _, Vec<u32>, bool)| {
                    *fixed && *other && (a.starts_with(b) || b.starts_with(a))
                };
                assert!(
                    !group[i + 1..].iter().any(meets),
                    "two elements carry the {kind} `{name}` where the runtime can \
                     reach both, so its statements act on whichever comes first: give one \
                     of them another name"
                );
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

/// A value [`El::child`] takes as a child on the page whose signals are `S`.
///
/// [`El::children`] and [`Page::child`](crate::Page::child) take it too.
/// It is implemented for every type that converts into a [`Node<S>`]: an
/// [`El<S>`], a `Node<S>` such as [`each`] and [`when`] return, and text as
/// a `&str` or a `String`. An element of another page carries that page's
/// signals, so it does not convert.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a child of an element on the page whose signals are `{S}`",
    note = "a child is an `El<{S}>`, a `Node<{S}>`, or text; an element built for another page's signals cannot sit inside this page"
)]
pub trait IntoNode<S> {
    /// Converts `self` into a node.
    fn into_node(self) -> Node<S>;
}

// Without `do_not_recommend`, rustc reports the failed `From` bound below
// this impl and lists every `From` impl of `Node`, in place of the message
// above.
#[diagnostic::do_not_recommend]
impl<S, N: Into<Node<S>>> IntoNode<S> for N {
    fn into_node(self) -> Node<S> {
        self.into()
    }
}

/// Renders `render` once per item of `list` in the browser, and updates the
/// rendered elements when the list changes.
///
/// `render` runs once on the server to build a template, receiving an
/// [`Item`] that stands for whichever item the browser is rendering. Items are
/// matched to their elements by position. Use [`each_keyed`] where the list is
/// reordered or an item before another is removed.
///
/// ```
/// use hypersig::html::{Role, each, li, ul};
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
/// use hypersig::html::{Role, each_keyed, li, ul};
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

/// Embeds the [`Part`] whose signals are the field `at` of this page.
///
/// The part renders with each of its paths written under `at`, and each
/// call it makes tells the server `at`, so the patches its handler answers
/// with land there too. `at` starts at the page root, because an item of a
/// list has no path the server can patch.
///
/// The result is a [`Node`], so nothing of the page can be added inside
/// the part's element, where it would be read as the part's.
#[must_use]
pub fn part<S, P: Part>(at: Sig<S, P>) -> Node<S> {
    let field: Arc<str> = at.path().into();
    let el = in_part(Some(field), P::render);
    Node(NodeKind::Element(Box::new(el.retype())))
}

impl<S> Node<S> {
    fn retype<S2>(self) -> Node<S2> {
        Node(match self.0 {
            NodeKind::Element(el) => NodeKind::Element(Box::new(el.retype())),
            NodeKind::Text(text) => NodeKind::Text(text),
            NodeKind::Each {
                list,
                depth,
                template,
            } => NodeKind::Each {
                list,
                depth,
                template: Box::new(template.retype()),
            },
            NodeKind::When { cond, template } => NodeKind::When {
                cond: retype_expr(cond),
                template: Box::new(template.retype()),
            },
        })
    }
}

fn retype_expr<S, S2, T>(expr: Expr<S, T>) -> Expr<S2, T> {
    Expr::from_json(expr.into_json())
}

fn retype_stmt<S, S2>(stmt: Stmt<S>) -> Stmt<S2> {
    Stmt::new(stmt.into_json())
}

impl<S> El<S> {
    /// The same element over the signals `S2`.
    ///
    /// Only [`part`] calls it, on an element whose paths were written under
    /// the field of `S2` that holds `S`.
    fn retype<S2>(self) -> El<S2> {
        El {
            tag: self.tag,
            void: self.void,
            id: self.id,
            attrs: self.attrs,
            classes: self.classes,
            show: self.show.map(retype_expr),
            text: self.text.map(retype_expr),
            checked: self.checked.map(retype_expr),
            bind: self.bind,
            class_when: retype_pairs(self.class_when),
            css: self.css,
            css_names: self.css_names,
            css_when: self
                .css_when
                .into_iter()
                .map(|(style, names, cond)| (style, names, retype_expr(cond)))
                .collect(),
            attr_bind: retype_pairs(self.attr_bind),
            attr_when: retype_pairs(self.attr_when),
            style: retype_pairs(self.style),
            key: self.key,
            node_ref: self.node_ref,
            foreign: self.foreign,
            file_input: self.file_input,
            effects: self.effects.into_iter().map(retype_stmt).collect(),
            watches: retype_stmts(self.watches),
            interval: self.interval.map(|(ms, stmt)| (ms, retype_stmt(stmt))),
            timeout: self.timeout.map(|(ms, stmt)| (ms, retype_stmt(stmt))),
            on: retype_stmts(self.on),
            custom: self
                .custom
                .into_iter()
                .map(|(name, depth, stmt)| (name, depth, retype_stmt(stmt)))
                .collect(),
            keys: retype_stmts(self.keys),
            roving: self.roving,
            active: self.active,
            children: self.children.into_iter().map(Node::retype).collect(),
        }
    }
}

fn retype_pairs<K, S, S2, T, C>(pairs: Vec<(K, Expr<S, T>)>) -> C
where
    C: FromIterator<(K, Expr<S2, T>)>,
{
    pairs
        .into_iter()
        .map(|(key, expr)| (key, retype_expr(expr)))
        .collect()
}

fn retype_stmts<K, S, S2, C>(pairs: impl IntoIterator<Item = (K, Stmt<S>)>) -> C
where
    C: FromIterator<(K, Stmt<S2>)>,
{
    pairs
        .into_iter()
        .map(|(key, stmt)| (key, retype_stmt(stmt)))
        .collect()
}

/// An HTML element belonging to a page whose signals are `S`.
pub struct El<S> {
    tag: &'static str,
    void: bool,
    id: Option<Id>,
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
    foreign: bool,
    file_input: Option<&'static str>,
    effects: Vec<Stmt<S>>,
    watches: Vec<(Value, Stmt<S>)>,
    interval: Option<(u32, Stmt<S>)>,
    timeout: Option<(u32, Stmt<S>)>,
    on: BTreeMap<Event, Stmt<S>>,
    custom: Vec<(&'static str, u8, Stmt<S>)>,
    keys: Vec<(Shortcut, Stmt<S>)>,
    roving: Option<Orientation>,
    active: Option<(&'static str, Value)>,
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
/// use hypersig::html::{Class, Role, Stylesheet, li};
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

/// Whether `css` holds `</style` in any case, which ends a `<style>` element.
const fn closes_style(css: &[u8]) -> bool {
    const END: &[u8] = b"</style";
    let mut start = 0;
    while start + END.len() <= css.len() {
        let mut i = 0;
        while i < END.len() && css[start + i].to_ascii_lowercase() == END[i] {
            i += 1;
        }
        if i == END.len() {
            return true;
        }
        start += 1;
    }
    false
}

impl Stylesheet {
    /// Wraps the CSS text `css`, usually from `include_str!`.
    ///
    /// # Panics
    ///
    /// Panics if `css` holds `</style`, in any case, which would close the
    /// `<style>` element the page writes it into. In a `const` the panic is a
    /// compile error.
    #[must_use]
    pub const fn new(css: &'static str) -> Self {
        assert!(
            !closes_style(css.as_bytes()),
            "a stylesheet holds no `</style`, which would end the `<style>` element"
        );
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
/// use hypersig::html::{Class, Role, li};
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
    /// Focus enters the element or one inside it from outside (`focusin`).
    ///
    /// Focus moving between two elements inside it does not run it again.
    Focus,
    /// Focus leaves the element and everything inside it (`focusout`).
    ///
    /// Focus moving between two elements inside it, such as an arrow key
    /// moving between radios in a group, does not run it.
    Blur,
    /// The pointer enters the element (`mouseenter`).
    MouseEnter,
    /// The pointer leaves the element (`mouseleave`).
    MouseLeave,
    /// Runs once when the element enters the document.
    Load,
    /// The element comes into the viewport. [`El::on_visible`] has the details.
    Visible,
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
            Self::Visible => "data-hs-on-visible",
        }
    }
}

/// An event that a script, such as a custom element, dispatches on an element.
///
/// Declare it once as a `const` next to the element that dispatches it, and
/// run a statement on it with [`El::on_custom`], which hands the statement
/// the event's `detail` as an [`Expr`] of type `T`.
///
/// The name and `T` describe code outside hypersig, so nothing checks them:
/// an event the script never dispatches runs nothing, and a `detail` of
/// another shape reaches the statement as it is, where reading a field it
/// lacks gives `null`. A [`SafeUrl`] in `T` is not checked
/// either, so the runtime checks the `href`, `src`, or `poster` a signal
/// gives an element, and removes the attribute where the URL has a scheme
/// a `SafeUrl` refuses. For a library that ships a Custom Elements Manifest,
/// `cargo hypersig elements` writes these constants from the manifest.
///
/// ```
/// use hypersig::html::{CustomEvent, Role, el};
/// use hypersig::{Page, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Rating {
///     stars: i32,
/// }
///
/// const RATE: CustomEvent<i32> = CustomEvent::new("rate");
///
/// let html = Page::new("Rating", Rating { stars: 0 })
///     .child(el(hypersig::tag!("star-rating")).on_custom(RATE, |stars| Rating::stars().set(stars)))
///     .render();
/// assert!(html.contains("data-hs-on-custom"));
/// ```
pub struct CustomEvent<T> {
    name: &'static str,
    marker: PhantomData<fn() -> T>,
}

impl<T> CustomEvent<T> {
    /// An event of type `name`, as the script passes it to `new CustomEvent`.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty or holds whitespace. In a `const`, that is a
    /// compile error.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        let bytes = name.as_bytes();
        assert!(!bytes.is_empty(), "an event name is never empty");
        let mut i = 0;
        while i < bytes.len() {
            assert!(
                !bytes[i].is_ascii_whitespace(),
                "an event name holds no whitespace"
            );
            i += 1;
        }
        Self {
            name,
            marker: PhantomData,
        }
    }

    /// The event's type, as the script dispatches it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }
}

impl<T> Clone for CustomEvent<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for CustomEvent<T> {}

impl<T> fmt::Debug for CustomEvent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CustomEvent").field(&self.name).finish()
    }
}

/// A key that [`El::on_key`] filters `keydown` by.
///
/// A key alone matches only while Ctrl, Alt, and Meta are all released. Its
/// methods [`ctrl`](Key::ctrl), [`alt`](Key::alt), [`meta`](Key::meta),
/// [`shift`](Key::shift), and [`primary`](Key::primary) make a [`Shortcut`]
/// that requires a modifier.
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
    ///
    /// The character already says whether Shift was held, as `'K'` and `'?'`
    /// do on a US layout, so Shift is not compared unless the shortcut asks
    /// for it with [`Shortcut::shift`].
    Char(char),
}

impl Key {
    /// This key with Ctrl held.
    #[must_use]
    pub fn ctrl(self) -> Shortcut {
        Shortcut::from(self).ctrl()
    }

    /// This key with Alt held, which is Option on a Mac.
    #[must_use]
    pub fn alt(self) -> Shortcut {
        Shortcut::from(self).alt()
    }

    /// This key with Meta held, which is Command on a Mac and the Windows key elsewhere.
    #[must_use]
    pub fn meta(self) -> Shortcut {
        Shortcut::from(self).meta()
    }

    /// This key with Shift held.
    #[must_use]
    pub fn shift(self) -> Shortcut {
        Shortcut::from(self).shift()
    }

    /// This key with Command on Apple platforms and Ctrl elsewhere.
    #[must_use]
    pub fn primary(self) -> Shortcut {
        Shortcut::from(self).primary()
    }
}

/// A [`Key`] and the modifiers held with it, which [`El::on_key`] and
/// [`Page::on_key`](crate::Page::on_key) match.
///
/// Ctrl, Alt, and Meta match exactly: `Key::Char('k').ctrl()` runs on Ctrl+K
/// and not on Ctrl+Alt+K. Shift matches exactly for a named key, so
/// `Key::Enter` leaves Shift+Enter to a `<textarea>`. For a [`Key::Char`],
/// Shift is compared only where [`shift`](Shortcut::shift) asks for it, and
/// the character is then compared without regard to case, since Shift
/// turns `k` into `K`.
///
/// The runtime decides [`primary`](Shortcut::primary) from
/// `navigator.platform`. On macOS, Option changes the character a key
/// produces, so a [`Key::Char`] with [`alt`](Shortcut::alt) does not match
/// there.
///
/// ```
/// use hypersig::html::{Key, Role, Shortcut};
///
/// let palette: Shortcut = Key::Char('k').primary();
/// let redo: Shortcut = Key::Char('z').primary().shift();
/// # let _ = (palette, redo);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shortcut {
    key: Key,
    /// The modifiers held, as bits indexed like [`MODIFIERS`].
    held: u8,
}

/// The names the runtime reads for the modifiers, in the order of the bits of `Shortcut::held`.
const MODIFIERS: [&str; 5] = ["ctrl", "alt", "meta", "shift", "primary"];

impl From<Key> for Shortcut {
    fn from(key: Key) -> Self {
        Self { key, held: 0 }
    }
}

impl Shortcut {
    fn with(mut self, modifier: &str) -> Self {
        let bit = MODIFIERS
            .iter()
            .position(|name| *name == modifier)
            .expect("a modifier `MODIFIERS` names");
        self.held |= 1 << bit;
        self
    }

    /// This shortcut with Ctrl held too.
    #[must_use]
    pub fn ctrl(self) -> Self {
        self.with("ctrl")
    }

    /// This shortcut with Alt held too, which is Option on a Mac.
    #[must_use]
    pub fn alt(self) -> Self {
        self.with("alt")
    }

    /// This shortcut with Meta held too, which is Command on a Mac.
    #[must_use]
    pub fn meta(self) -> Self {
        self.with("meta")
    }

    /// This shortcut with Shift held too.
    #[must_use]
    pub fn shift(self) -> Self {
        self.with("shift")
    }

    /// This shortcut with Command held on Apple platforms and Ctrl elsewhere.
    #[must_use]
    pub fn primary(self) -> Self {
        self.with("primary")
    }

    /// The entry of a `data-hs-on-key` list: the key, the statement, and the
    /// names of the modifiers where any is held.
    fn json(self, stmt: &Value) -> Value {
        if self.held == 0 {
            return json!([self.key.name(), stmt]);
        }
        let held: Vec<&str> = MODIFIERS
            .iter()
            .enumerate()
            .filter(|(bit, _)| self.held & (1 << bit) != 0)
            .map(|(_, name)| *name)
            .collect();
        json!([self.key.name(), stmt, held])
    }
}

/// The value of `data-hs-on-key` for `keys`.
pub(crate) fn keys_json<S>(keys: &[(Shortcut, Stmt<S>)]) -> String {
    let keys: Vec<Value> = keys
        .iter()
        .map(|(shortcut, stmt)| shortcut.json(stmt.as_json()))
        .collect();
    json_text(&keys)
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

/// A horizontal or vertical direction, which decides the arrow keys
/// [`El::roving_focus`] uses.
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
/// `bool` to `checked`, and `i32`, `i64`, and `f64` to a number or range
/// field.
///
/// An `Option` of `String`, `i32`, `i64`, or `f64` binds as the type it
/// holds, with an empty field standing for `None` in both directions: `None`
/// empties the field, and emptying the field sets `None`. A number field
/// whose text is not a number leaves the signal as it was.
#[diagnostic::on_unimplemented(
    message = "a form control cannot be bound to a `{Self}` signal",
    note = "bind `String`, `bool`, `i32`, `i64`, `f64`, or an `Option` of one other than `bool`; for other types use `bind_select` or `bind_radio`"
)]
pub trait Bindable: sealed::Sealed {}
impl Bindable for String {}
impl Bindable for bool {}
impl Bindable for i32 {}
impl Bindable for i64 {}
impl Bindable for f64 {}
impl Bindable for Option<String> {}
impl Bindable for Option<i32> {}
impl Bindable for Option<i64> {}
impl Bindable for Option<f64> {}

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
    impl Sealed for i64 {
        const KIND: &'static str = "int";
    }
    impl Sealed for f64 {
        const KIND: &'static str = "float";
    }
    impl Sealed for Option<String> {
        const KIND: &'static str = "opt_text";
    }
    impl Sealed for Option<i32> {
        const KIND: &'static str = "opt_int";
    }
    impl Sealed for Option<i64> {
        const KIND: &'static str = "opt_int";
    }
    impl Sealed for Option<f64> {
        const KIND: &'static str = "opt_float";
    }
}

/// The attribute a server-rendered element hidden by [`El::show`] carries.
pub(crate) const HIDE: &str = "data-hs-hide";

/// The attribute that selects the rule holding a server-rendered element's
/// [`El::bind_style`] values, on a page with a CSP nonce.
pub(crate) const INLINE: &str = "data-hs-inline";

/// Whether `value` can sit inside a declaration in a `<style>` element
/// without ending the declaration, the rule, a string, or the element.
///
/// An unclosed `(` would let a `url(` token run on into the next rules, so
/// the parentheses have to balance.
fn plain_css(value: &str) -> bool {
    let mut depth = 0_usize;
    for c in value.chars() {
        match c {
            '(' => depth += 1,
            ')' => match depth.checked_sub(1) {
                Some(less) => depth = less,
                None => return false,
            },
            '{' | '}' | ';' | '<' | '>' | '\\' | '"' | '\'' => return false,
            c if c.is_control() => return false,
            _ => {}
        }
    }
    depth == 0 && !value.contains("/*")
}

/// Creates an element with a tag that has no named constructor, such as an
/// SVG element or a custom element.
///
/// Use the named constructors such as [`div`] where one exists, and build
/// `tag` with [`tag!`](hypersig::tag!). A tag the HTML standard lists as void,
/// such as `source` or `meta`, is written without children or a closing tag,
/// as the browser parses it.
#[must_use]
pub fn el<S>(tag: Tag) -> El<S> {
    El::new(tag.0, VOID.contains(&tag.0))
}

/// The name of an element that [`el`] creates.
///
/// [`tag!`](hypersig::tag!) builds one at compile time. A tag is ASCII letters,
/// digits, `-`, `_`, `.`, and `:`, starting with a letter, so the name cannot
/// end the tag and start an attribute. It is never `script`, `style`,
/// `base`, or `link`, in any case: the first two run their text as code,
/// `base` moves every relative URL of the page, the action URLs among them,
/// to another origin, and `link` loads a stylesheet or a preload from the
/// host its `href` names. [`Page::stylesheet`](crate::Page::stylesheet) and
/// [`Page::icon`](crate::Page::icon) write the `<link>`s a page needs, from a
/// URL written in the program.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tag(&'static str);

impl Tag {
    /// Checks `name` against the rules of [`Tag`].
    ///
    /// # Panics
    ///
    /// Panics for a name that breaks them. In a `const`, the panic is a
    /// compile error.
    pub(crate) const fn new(name: &'static str) -> Self {
        let bytes = name.as_bytes();
        assert!(
            !bytes.is_empty() && bytes[0].is_ascii_alphabetic(),
            "a tag starts with a letter"
        );
        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];
            assert!(
                b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'),
                "a tag is letters, digits, `-`, `_`, `.`, and `:`"
            );
            i += 1;
        }
        assert!(
            !is(bytes, "script") && !is(bytes, "style") && !is(bytes, "base"),
            "`script` and `style` run their text as code, and `base` moves the page's URLs"
        );
        assert!(
            !is(bytes, "link"),
            "`link` loads from the host its `href` names; `Page::stylesheet` and `Page::icon` write the page's `<link>`s"
        );
        Self(name)
    }

    /// The tag's name, as the HTML holds it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.0
    }
}

/// What an image of [`El::srcset`] is drawn for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Candidate {
    /// The image's width in pixels, written as `480w`.
    Width(u32),
    /// The device pixels per CSS pixel it suits, written as `2x`.
    Density(f64),
}

/// `duration` in whole milliseconds for a browser timer, which `method` sets.
///
/// A browser runs a timer longer than 2^31 - 1 milliseconds at once, as if
/// it had no delay.
fn timer_millis(duration: Duration, method: &str) -> u32 {
    const LONGEST: u32 = (1 << 31) - 1;
    u32::try_from(duration.as_millis())
        .ok()
        .filter(|millis| *millis <= LONGEST)
        .unwrap_or_else(|| {
            panic!("the time `{method}` waits is at most 2^31 - 1 milliseconds, about 24.8 days")
        })
}

/// The tags the HTML standard defines as void elements, which take no
/// children or closing tag.
const VOID: [&str; 13] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

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
    a, abbr, address, article, aside, audio, b, blockquote, button, caption, cite, code, colgroup,
    dd, del, details, dialog, div, dl, dt, em, fieldset, figcaption, figure, footer, form, h1, h2,
    h3, h4, h5, h6, header, i, ins, kbd, label, legend, li, main, mark, meter, nav, ol, optgroup,
    option, output, p, picture, pre, progress, section, select, small, span, strong, sub, summary,
    sup, table, tbody, td, textarea, tfoot, th, thead, time, tr, ul, video,
);

void_tags!(br, col, hr, img, input, source, track, wbr);

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
            foreign: false,
            file_input: None,
            effects: Vec::new(),
            watches: Vec::new(),
            interval: None,
            timeout: None,
            on: BTreeMap::new(),
            custom: Vec::new(),
            keys: Vec::new(),
            roving: None,
            active: None,
            children: Vec::new(),
        }
    }

    /// Sets a plain attribute.
    ///
    /// `name` is a constant of [`attr`] or a name built with
    /// [`attr!`](crate::attr!), neither of which can name an attribute whose
    /// value runs a script.
    ///
    /// # Panics
    ///
    /// Panics if the element already has the attribute `name`, from this
    /// method or from one that sets it, such as [`El::role`]. The browser
    /// reads the first of two and ignores the second.
    #[must_use]
    pub fn attr(mut self, name: Attr, value: impl Into<String>) -> Self {
        self.set_attr(name.0, value.into());
        self
    }

    /// Sets the WAI-ARIA role a screen reader announces the element by.
    ///
    /// ```
    /// use hypersig::html::{Role, div};
    ///
    /// let html = div::<()>().role(Role::Status).text("Saved").render();
    /// assert_eq!(html, r#"<div role="status">Saved</div>"#);
    /// ```
    #[must_use]
    pub fn role(mut self, role: Role) -> Self {
        self.set_attr("role", role.name().to_owned());
        self
    }

    /// Keeps attribute `name` equal to `value` as signals change.
    #[must_use]
    pub fn bind_attr(mut self, name: Attr, value: impl IntoExpr<S, String>) -> Self {
        self.attr_bind.push((name.0, value.into_expr()));
        self
    }

    /// Sets the boolean attribute `name`, such as `disabled`, while `when` is true.
    #[must_use]
    pub fn attr_when(mut self, name: Attr, when: impl IntoExpr<S, bool>) -> Self {
        self.attr_when.push((name.0, when.into_expr()));
        self
    }

    /// Disables the control while `when` is true, through `disabled`.
    ///
    /// The browser skips a disabled control in the tab order, ignores clicks
    /// on it, and leaves it out of a submitted form. That holds on
    /// `<button>`, `<input>`, `<select>`, `<textarea>`, `<fieldset>`,
    /// `<option>`, and `<optgroup>`. Nothing checks the tag, and on another
    /// element the attribute changes nothing.
    ///
    /// ```
    /// use hypersig::html::button;
    /// use hypersig::Signals;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Save {
    ///     saving: bool,
    /// }
    ///
    /// let save = button::<Save>().disabled(Save::saving()).text("Save");
    /// assert!(save.render().contains("data-hs-toggle"));
    /// ```
    #[must_use]
    pub fn disabled(self, when: impl IntoExpr<S, bool>) -> Self {
        self.attr_when(attr::DISABLED, when)
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

    /// Sets the element's `href`: a link to a page the router serves, or a
    /// [`SafeUrl`] for an address it does not.
    ///
    /// A fixed URL is written as a plain attribute. One that reads signals is
    /// kept up to date as they change.
    #[must_use]
    pub fn href(self, url: impl IntoHref<S>) -> Self {
        self.url_attr("href", url)
    }

    /// Sets the element's `src`, the address of the image, frame, or script
    /// it loads, as [`El::href`] sets `href`.
    ///
    /// A [`SafeUrl`] may name any host, so the `src` of an
    /// `<iframe>`, an `<embed>`, or a `<frame>`, which shows the page at that
    /// URL inside this one, comes from the program, never from a visitor.
    ///
    /// # Panics
    ///
    /// Panics on an `<iframe>`, an `<embed>`, or a `<frame>` if `url` reads
    /// signals, which the browser could change to a page of any host.
    #[must_use]
    pub fn src(self, url: impl IntoHref<S>) -> Self {
        self.url_attr("src", url)
    }

    /// Sets the image a `<video>` shows before it plays, as [`El::src`]
    /// sets `src`.
    #[must_use]
    pub fn poster(self, url: impl IntoHref<S>) -> Self {
        self.url_attr("poster", url)
    }

    /// Sets the images the browser picks from for an `<img>` or a
    /// `<source>`, each a URL and the width or pixel density it is drawn for.
    ///
    /// ```
    /// use hypersig::SafeUrl;
    /// use hypersig::html::{Candidate, img};
    ///
    /// let html = img::<()>()
    ///     .src(SafeUrl::new("/a-480.png"))
    ///     .srcset([
    ///         (SafeUrl::new("/a-480.png"), Candidate::Width(480)),
    ///         (SafeUrl::new("/a-960.png"), Candidate::Width(960)),
    ///     ])
    ///     .render();
    /// assert_eq!(
    ///     html,
    ///     r#"<img src="/a-480.png" srcset="/a-480.png 480w, /a-960.png 960w">"#
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if a [`Candidate::Density`] is not a finite number above 0.
    #[must_use]
    pub fn srcset(mut self, candidates: impl IntoIterator<Item = (SafeUrl, Candidate)>) -> Self {
        let set: Vec<String> = candidates
            .into_iter()
            .map(|(url, candidate)| match candidate {
                Candidate::Width(width) => format!("{url} {width}w"),
                Candidate::Density(density) => {
                    assert!(
                        density.is_finite() && density > 0.0,
                        "a pixel density is a finite number above 0, not {density}"
                    );
                    format!("{url} {density}x")
                }
            })
            .collect();
        self.set_attr("srcset", set.join(", "));
        self
    }

    /// Adds the static attribute `name`.
    ///
    /// # Panics
    ///
    /// Panics if the element already has `name`, because the browser reads
    /// only the first of two.
    fn set_attr(&mut self, name: &'static str, value: String) {
        assert!(
            !self
                .attrs
                .iter()
                .any(|(set, _)| set.eq_ignore_ascii_case(name)),
            "the attribute `{name}` is set twice on one `<{}>`, and the browser keeps only the first",
            self.tag
        );
        self.attrs.push((name, value));
    }

    fn url_attr(mut self, name: &'static str, url: impl IntoHref<S>) -> Self {
        match url.into_parts() {
            Ok(url) => self.set_attr(name, url),
            Err(expr) => {
                assert!(
                    !["iframe", "embed", "frame"]
                        .iter()
                        .any(|frame| self.tag.eq_ignore_ascii_case(frame)),
                    "the `{name}` of a `<{}>` shows a page inside this one, so it comes from \
                     the program and reads no signal",
                    self.tag
                );
                self.attr_bind.push((name, expr));
            }
        }
        self
    }

    /// Sets the element's `id`.
    ///
    /// An element rendered through a [`Fragment`](crate::Fragment) gets the
    /// fragment's id instead. Rendering panics if the element sits inside a
    /// list template, whose every item would repeat it, and if another
    /// element the runtime can reach carries the same id from [`Id::new`],
    /// with or without the keys of [`Id::with_key`].
    ///
    /// Two ids alike where either comes from [`Id::from_text`] render,
    /// because the text may come from data visitors stored. An anchor, a
    /// label's `for`, an `aria-*` reference, or a fragment's patch to that id
    /// then reaches the first of them.
    ///
    /// # Panics
    ///
    /// Panics if the element already has an id from an earlier call.
    #[must_use]
    pub fn id(mut self, id: Id) -> Self {
        assert!(
            self.id.is_none(),
            "`El::id` is called twice on one `<{}>`, which holds one id",
            self.tag
        );
        self.id = Some(id);
        self
    }

    /// Points a `<label>` or an `<output>` at the control it names, through `for`.
    #[must_use]
    pub fn label_for(mut self, control: &Id) -> Self {
        self.set_attr("for", control.as_str().to_owned());
        self
    }

    /// Names the elements whose text labels this one, through `aria-labelledby`.
    #[must_use]
    pub fn aria_labelledby(mut self, labels: &[&Id]) -> Self {
        self.set_attr("aria-labelledby", id::list(labels));
        self
    }

    /// Names the elements whose text describes this one, through `aria-describedby`.
    #[must_use]
    pub fn aria_describedby(mut self, descriptions: &[&Id]) -> Self {
        self.attrs
            .push(("aria-describedby", id::list(descriptions)));
        self
    }

    /// Names the elements this one shows, hides, or changes, through `aria-controls`.
    #[must_use]
    pub fn aria_controls(mut self, controlled: &[&Id]) -> Self {
        self.set_attr("aria-controls", id::list(controlled));
        self
    }

    /// Names elements outside this one that belong to it, through `aria-owns`.
    #[must_use]
    pub fn aria_owns(mut self, owned: &[&Id]) -> Self {
        self.set_attr("aria-owns", id::list(owned));
        self
    }

    /// Names the elements that hold more detail on this one, through `aria-details`.
    #[must_use]
    pub fn aria_details(mut self, details: &[&Id]) -> Self {
        self.set_attr("aria-details", id::list(details));
        self
    }

    /// Names the elements a reader goes to next, through `aria-flowto`.
    #[must_use]
    pub fn aria_flowto(mut self, next: &[&Id]) -> Self {
        self.set_attr("aria-flowto", id::list(next));
        self
    }

    /// Names the elements that hold this control's error, through `aria-errormessage`.
    ///
    /// A screen reader reads them while `aria-invalid` is `true`.
    #[must_use]
    pub fn aria_errormessage(mut self, messages: &[&Id]) -> Self {
        self.set_attr("aria-errormessage", id::list(messages));
        self
    }

    /// Names the descendant that has the focus, through `aria-activedescendant`.
    ///
    /// For a value that follows a signal, [`El::active_descendant`] binds it.
    #[must_use]
    pub fn aria_activedescendant(mut self, active: &Id) -> Self {
        self.attrs
            .push(("aria-activedescendant", active.as_str().to_owned()));
        self
    }

    /// Names the element a `command` of this button acts on, through `commandfor`.
    #[must_use]
    pub fn command_for(mut self, target: &Id) -> Self {
        self.set_attr("commandfor", target.as_str().to_owned());
        self
    }

    /// Names the popover this button shows and hides, through `popovertarget`.
    #[must_use]
    pub fn popover_target(mut self, popover: &Id) -> Self {
        self.attrs
            .push(("popovertarget", popover.as_str().to_owned()));
        self
    }

    /// Names the `<datalist>` whose options this `<input>` suggests, through `list`.
    #[must_use]
    pub fn datalist(mut self, options: &Id) -> Self {
        self.set_attr("list", options.as_str().to_owned());
        self
    }

    /// Names the `<form>` this control belongs to from outside it, through `form`.
    #[must_use]
    pub fn form_owner(mut self, form: &Id) -> Self {
        self.set_attr("form", form.as_str().to_owned());
        self
    }

    /// Names the header cells of this table cell, through `headers`.
    #[must_use]
    pub fn headers(mut self, cells: &[&Id]) -> Self {
        self.set_attr("headers", id::list(cells));
        self
    }

    /// Names the elements whose microdata properties this item adds, through `itemref`.
    #[must_use]
    pub fn itemref(mut self, elements: &[&Id]) -> Self {
        self.set_attr("itemref", id::list(elements));
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
            self = self.css_when(style, value.equals(&case));
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
    ///
    /// # Panics
    ///
    /// Panics if the element already has a node ref from this method or
    /// [`El::foreign`].
    #[must_use]
    pub fn node_ref(mut self, node_ref: NodeRef<S>) -> Self {
        self.set_node_ref(node_ref);
        self
    }

    fn set_node_ref(&mut self, node_ref: NodeRef<S>) {
        assert!(
            self.node_ref.is_none(),
            "a `<{}>` gets a node ref twice, from `El::node_ref` or `El::foreign`, and holds one",
            self.tag
        );
        self.node_ref = Some(node_ref.name());
    }

    /// Leaves this element's attributes and children to a script, and names it `node_ref`.
    ///
    /// A library such as a map or a text editor adds elements, classes, and
    /// inline styles to the element it is given. A server patch that reaches
    /// this element keeps all of them and keeps the element as it is, so the
    /// page renders its children and its static attributes once. Its bindings,
    /// such as [`El::bind_attr`] and [`El::on_custom`], keep running, so the
    /// values a script reads change through signals. Among its siblings a
    /// patch finds it by its id, or by the name of `node_ref` where it has none,
    /// so a sibling added before it does not take its place. A script finds
    /// it by an [`Id`] it carries, since the name `node_ref!` writes follows
    /// the constant's path and changes when the module is renamed. A list
    /// item or a condition that holds it is built anew with it when the
    /// runtime renders the item or the condition again.
    ///
    /// ```
    /// use hypersig::html::{El, Role, div};
    /// use hypersig::{NodeRef, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Place {
    ///     zoom: i32,
    /// }
    ///
    /// hypersig::node_ref!(const MAP: NodeRef<Place>);
    ///
    /// # fn main() {
    /// let map: El<Place> = div().foreign(MAP).bind_attr(hypersig::attr!("data-zoom"), Place::zoom().to_text());
    /// assert!(map.render().contains(&format!(r#"data-hs-foreign="{}::MAP""#, module_path!())));
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the element already has a node ref.
    #[must_use]
    pub fn foreign(mut self, node_ref: NodeRef<S>) -> Self {
        self.set_node_ref(node_ref);
        self.foreign = true;
        self
    }

    /// Names this element as the file input `name`, which uploads read.
    pub(crate) fn file_input(mut self, name: &'static str) -> Self {
        self.file_input = Some(name);
        self
    }

    /// Appends a child element, text, or template.
    #[must_use]
    pub fn child(mut self, child: impl IntoNode<S>) -> Self {
        self.children.push(child.into_node());
        self
    }

    /// Appends several children. An `Option` counts as zero or one child.
    #[must_use]
    pub fn children<N: IntoNode<S>>(mut self, children: impl IntoIterator<Item = N>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoNode::into_node));
        self
    }

    /// Appends static text. Shorthand for `.child(text)`.
    #[must_use]
    pub fn text(self, text: impl Into<String>) -> Self {
        self.child(Node(NodeKind::Text(text.into())))
    }

    /// Shows the element only while `when` is true.
    ///
    /// # Panics
    ///
    /// Panics if the element already has a condition from an earlier call.
    /// Join two conditions with `and` into one.
    #[must_use]
    pub fn show(mut self, when: impl IntoExpr<S, bool>) -> Self {
        assert!(
            self.show.is_none(),
            "`El::show` is called twice on one `<{}>`; join the conditions with `and`",
            self.tag
        );
        self.show = Some(when.into_expr());
        self
    }

    /// Replaces the element's text with `text`, kept up to date as signals change.
    ///
    /// # Panics
    ///
    /// Panics if the element already has its text bound by an earlier call.
    #[must_use]
    pub fn bind_text(mut self, text: impl IntoExpr<S, String>) -> Self {
        assert!(
            self.text.is_none(),
            "`El::bind_text` is called twice on one `<{}>`, whose text is one expression",
            self.tag
        );
        self.text = Some(text.into_expr());
        self
    }

    /// Keeps a checkbox's `checked` state equal to `when`, in one direction.
    ///
    /// Use it for a checkbox whose state is derived, such as "all done".
    /// A checkbox that edits a signal uses [`El::bind`] instead.
    ///
    /// # Panics
    ///
    /// Panics on an element other than `<input>`, whose `checked` the
    /// runtime cannot write, or if `checked` is already bound.
    #[must_use]
    pub fn bind_checked(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.check_control("bind_checked", &["input"]);
        assert!(
            self.checked.is_none(),
            "`El::bind_checked` is called twice on one `<input>`"
        );
        self.checked = Some(when.into_expr());
        self
    }

    /// Binds a form control to a signal in both directions.
    ///
    /// The signal can be a page signal or a field of a list item.
    ///
    /// # Panics
    ///
    /// Panics on an element other than `<input>`, `<textarea>`, and
    /// `<select>`, which the runtime cannot bind, or if the element is
    /// already bound by this method, [`El::bind_radio`], or
    /// [`El::bind_select`].
    #[must_use]
    pub fn bind<T: Bindable, R: Writable>(mut self, signal: Sig<S, T, R>) -> Self {
        self.set_bind(
            "bind",
            &["input", "textarea", "select"],
            json!([signal.raw_path().place_json(), T::KIND]),
        );
        self
    }

    /// Binds the control to `bind`, which `method` writes.
    ///
    /// # Panics
    ///
    /// Panics if the element is none of `controls`, or is already bound.
    fn set_bind(&mut self, method: &str, controls: &[&str], bind: Value) {
        self.check_control(method, controls);
        assert!(
            self.bind.is_none(),
            "a `<{}>` is bound twice, by `El::{method}` and an earlier `bind` method, and binds one signal",
            self.tag
        );
        self.bind = Some(bind);
    }

    /// Panics unless the element is one of `controls`, the elements the
    /// runtime binds for `method`.
    fn check_control(&self, method: &str, controls: &[&str]) {
        assert!(
            controls
                .iter()
                .any(|tag| self.tag.eq_ignore_ascii_case(tag)),
            "`El::{method}` binds {}, not a `<{}>`, which the runtime cannot bind",
            controls
                .iter()
                .map(|tag| format!("`<{tag}>`"))
                .collect::<Vec<_>>()
                .join(" or "),
            self.tag
        );
    }

    /// Makes this radio button set `signal` to `value` when checked, and
    /// checks it while `signal` equals `value`.
    ///
    /// # Panics
    ///
    /// Panics if `value` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error, or if it holds an integer outside -2^53 to 2^53,
    /// which the browser would round. Panics on an element other than
    /// `<input>`, or one already bound or given a `type`.
    #[must_use]
    pub fn bind_radio<T: Serialize + PartialEq, R: Writable>(
        mut self,
        signal: Sig<S, T, R>,
        value: &T,
    ) -> Self {
        let value = exact::to_value(value, "a radio value does not serialize to JSON");
        self.set_attr("type", "radio".to_owned());
        self.set_bind(
            "bind_radio",
            &["input"],
            json!([signal.raw_path().place_json(), "radio", value]),
        );
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
    /// `Serialize` impl that returns an error, or if it holds an integer outside -2^53 to 2^53,
    /// which the browser would round. Panics on an element other than
    /// `<select>`, or one already bound.
    #[must_use]
    pub fn bind_select<T: Serialize + PartialEq, R: Writable, L: Into<String>>(
        mut self,
        signal: Sig<S, T, R>,
        options: impl IntoIterator<Item = (T, L)>,
    ) -> Self {
        for (value, label) in options {
            let value =
                exact::to_value(&value, "an option value does not serialize to JSON").to_string();
            self.children
                .push(option().attr(attr::VALUE, value).text(label).into());
        }
        self.set_bind(
            "bind_select",
            &["select"],
            json!([signal.raw_path().place_json(), "json"]),
        );
        self
    }

    /// Runs `stmt` on `event`. When `on` is called twice for the same event, the
    /// second statement runs after the first.
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
    ///
    /// # Panics
    ///
    /// Panics if `quiet` is longer than 2^31 - 1 milliseconds, about 24.8
    /// days, which the browser's timer cannot wait.
    #[must_use]
    pub fn on_debounced(self, event: Event, quiet: Duration, stmt: impl IntoStmt<S>) -> Self {
        let wrapped = Stmt::new(json!([
            "debounce",
            timer_millis(quiet, "on_debounced"),
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

    /// Runs `stmt` when focus leaves the element and everything inside it.
    #[must_use]
    pub fn on_blur(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Blur, stmt)
    }

    /// Runs the statement `handle` builds whenever `event` reaches this element.
    ///
    /// `handle` receives the event's `detail`. The runtime listens on this
    /// element itself, so an event dispatched without `bubbles` runs it when
    /// the script dispatches it on this element, and one with `bubbles` also
    /// when it is dispatched inside. Calling it twice for one event runs both
    /// statements in order. [`CustomEvent`] has an example.
    #[must_use]
    pub fn on_custom<T, B: IntoStmt<S>>(
        mut self,
        event: CustomEvent<T>,
        handle: impl FnOnce(Expr<S, T>) -> B,
    ) -> Self {
        let (depth, stmt) = in_let_scope(|depth| {
            let detail = Expr::from_json(json!(["var", depth]));
            (depth, handle(detail).into_stmt())
        });
        self.custom.push((event.name, depth, stmt));
        self
    }

    /// Runs `stmt` once when the element is added to the document.
    #[must_use]
    pub fn on_load(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Load, stmt)
    }

    /// Runs `stmt` each time the element comes into the viewport.
    ///
    /// It runs when the element is added in view, and again each time it
    /// enters the viewport after leaving it. An element hidden by
    /// [`El::show`] is out of view, so showing it again in view runs `stmt`.
    /// The browser decides with an `IntersectionObserver`, and a browser
    /// without one never runs `stmt`.
    ///
    /// For a list that loads more as the visitor scrolls, put an element
    /// after the items and hide it while a load is running and once nothing
    /// is left. When the new items leave it in view, showing it again loads
    /// the next page.
    ///
    /// ```
    /// use hypersig::html::{Role, div, each, li, ul};
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Feed {
    ///     items: Vec<String>,
    ///     loading: bool,
    ///     more: bool,
    /// }
    ///
    /// hypersig::routes! {
    ///     fn actions(Router<()>);
    ///     // The handler appends the items after the first `i32` and sets `more`.
    ///     action MORE: Action<Feed, i32> = get "/more" => more;
    /// }
    /// # async fn more((): (), _: i32) -> hypersig::Patches<Feed> { hypersig::Patches::new() }
    /// let feed = div::<Feed>()
    ///     .child(ul().child(each(Feed::items(), |item| li().bind_text(item))))
    ///     .child(
    ///         div()
    ///             .show(Feed::more().and(!Feed::loading()))
    ///             .on_visible(MORE.call_with(Feed::items().len()).indicator(Feed::loading())),
    ///     );
    /// assert!(feed.render().contains("data-hs-on-visible"));
    /// ```
    #[must_use]
    pub fn on_visible(self, stmt: impl IntoStmt<S>) -> Self {
        self.on(Event::Visible, stmt)
    }

    /// Runs `stmt` when `shortcut` is pressed while focus is in this element.
    ///
    /// `shortcut` is a [`Key`] alone, or a [`Shortcut`] such as
    /// `Key::Char('k').primary()`, and its modifiers match as [`Shortcut`]
    /// describes. The runtime calls `preventDefault` for a matched key.
    ///
    /// A text field inside this element keeps the keys it edits text with,
    /// unless Ctrl, Alt, Meta, or [`primary`](Shortcut::primary) is part of
    /// the shortcut. Those keys are a character, Space, Backspace, Delete,
    /// the left and right arrows, Home, and End, and in a `<textarea>` or an
    /// editable element also Enter and the up and down arrows. A handler on
    /// the field itself still takes them, so put `on_key` on the field to
    /// act on one of them there.
    ///
    /// Where several elements around the focus have handlers, the nearest one
    /// with a shortcut that matches runs, and the ones further out do not.
    /// [`Page::on_key`](crate::Page::on_key) is the outermost.
    ///
    /// ```
    /// use hypersig::html::{Key, Role, div, input};
    /// use hypersig::Signals;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Editor {
    ///     text: String,
    ///     saved: bool,
    /// }
    ///
    /// // Typing an `s` in the field types it, and Ctrl+S or Command+S saves.
    /// let editor = div::<Editor>()
    ///     .on_key(Key::Char('s').primary(), Editor::saved().set(true))
    ///     .child(input().bind(Editor::text()));
    /// assert!(editor.render().contains(r#"[&quot;s&quot;,[&quot;set&quot;"#));
    /// ```
    #[must_use]
    pub fn on_key(mut self, shortcut: impl Into<Shortcut>, stmt: impl IntoStmt<S>) -> Self {
        self.keys.push((shortcut.into(), stmt.into_stmt()));
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
    /// use hypersig::html::{Event, Orientation, Role, attr, button, div};
    /// use hypersig::{Signals, if_else};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Tabs {
    ///     tab: i32,
    /// }
    ///
    /// let tab = Tabs::tab();
    /// let list = div::<Tabs>().role(Role::TabList).roving_focus(Orientation::Horizontal).children(
    ///     (0..3).map(|i| {
    ///         button()
    ///             .role(Role::Tab)
    ///             .bind_attr(attr::TABINDEX, if_else(tab.equals(i), "0", "-1"))
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

    /// Tracks the active option of `listbox` in `active` on the up and down
    /// arrows, while focus stays on this element.
    ///
    /// This is the keyboard half of a combobox whose focus stays in its text
    /// field. An option is an element inside `listbox` with `role="option"`
    /// and an `id`, which is not `aria-disabled="true"` and is not hidden by
    /// [`El::show`]. The down arrow sets `active` to the id of the next shown
    /// option and the up arrow to the previous one, wrapping at either end
    /// and starting from the first or the last where `active` names no shown
    /// option. Enter clicks the option `active` names. The runtime calls
    /// `preventDefault` for a key it acts on, scrolls the new option into
    /// view, and leaves a key alone while Alt, Control, or Meta is held.
    ///
    /// It runs before the handlers of [`El::on_key`], which then still see
    /// the key. It also binds `aria-activedescendant` to `active`, so a
    /// screen reader reads the option `active` names. Style that option
    /// yourself, since the runtime changes nothing else.
    ///
    /// ```
    /// use hypersig::html::{Role, div, input};
    /// use hypersig::{NodeRef, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Search {
    ///     query: String,
    ///     active: String,
    /// }
    ///
    /// hypersig::node_ref!(const RESULTS: NodeRef<Search>);
    ///
    /// # fn main() {
    /// let field = input::<Search>()
    ///     .role(Role::Combobox)
    ///     .bind(Search::query())
    ///     .active_descendant(RESULTS, Search::active());
    /// let results = div::<Search>().role(Role::ListBox).node_ref(RESULTS);
    /// assert!(field.render().contains(&format!(r#"data-hs-active="[&quot;{}::RESULTS&quot;"#, module_path!())));
    /// # let _ = results;
    /// # }
    /// ```
    #[must_use]
    pub fn active_descendant(mut self, listbox: NodeRef<S>, active: Sig<S, String>) -> Self {
        self.active = Some((listbox.name(), active.raw_path().place_json()));
        self.attr_bind
            .push(("aria-activedescendant", active.into_expr()));
        self
    }

    /// Runs `stmt` every `period` while the element is in the document.
    ///
    /// A second call replaces the first.
    ///
    /// # Panics
    ///
    /// Panics if `period` is under 1 millisecond, which would run `stmt` as
    /// often as the browser lets a timer run, or longer than 2^31 - 1
    /// milliseconds, about 24.8 days, which the browser's timer cannot wait.
    #[must_use]
    pub fn on_interval(mut self, period: Duration, stmt: impl IntoStmt<S>) -> Self {
        let millis = timer_millis(period, "on_interval");
        assert!(
            millis >= 1,
            "the period of `on_interval` is at least 1 millisecond"
        );
        self.interval = Some((millis, stmt.into_stmt()));
        self
    }

    /// Runs `stmt` once, `delay` after the element is added to the document, if
    /// the element is still in it then.
    ///
    /// A second call replaces the first.
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// use hypersig::html::{Role, each_keyed, li, ul};
    /// use hypersig::Signals;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Notice {
    ///     id: i32,
    ///     text: String,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Page {
    ///     notices: Vec<Notice>,
    /// }
    ///
    /// let list = ul::<Page>().child(each_keyed(
    ///     Page::notices(),
    ///     |notice| notice.at(Notice::id()),
    ///     |notice| {
    ///         li().bind_text(notice.at(Notice::text()))
    ///             .on_timeout(Duration::from_secs(5), notice.remove())
    ///     },
    /// ));
    /// assert!(list.render().contains("data-hs-on-timeout"));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `delay` is longer than 2^31 - 1 milliseconds, about 24.8
    /// days, which the browser's timer cannot wait.
    #[must_use]
    pub fn on_timeout(mut self, delay: Duration, stmt: impl IntoStmt<S>) -> Self {
        self.timeout = Some((timer_millis(delay, "on_timeout"), stmt.into_stmt()));
        self
    }

    /// Runs `stmt` when the element is added to the document and whenever a
    /// signal `stmt` reads changes.
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

    pub(crate) fn with_id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Renders the element and its children to an HTML string.
    ///
    /// The reactive parts are left for the runtime to fill in, because an
    /// element alone has no signal values. [`Page::render`](crate::Page::render) fills them in.
    ///
    /// # Panics
    ///
    /// Panics if two elements carry one [`NodeRef`] name, one
    /// [`FileInput`](crate::FileInput) name, or one `&'static str` id, where
    /// the runtime can reach both, or if an element inside a list template has
    /// an id set with [`El::id`]. Each item of a list may carry the same name.
    #[must_use]
    pub fn render(&self) -> String {
        let mut names = Names::default();
        names.el(self, None);
        names.check();
        el_item_reads(self, 0);
        let mut out = String::new();
        self.write(&mut out, None, None);
        out
    }

    /// Writes one copy of this template per item of `list`, each marked with
    /// the key the runtime will compute for it, so the runtime adopts them.
    fn write_items(&self, out: &mut String, ctx: &Ctx<'_>, list: &Value, depth: usize) {
        let Some(items) = ctx.lookup(list).and_then(Ref::items) else {
            return;
        };
        let mut key = String::new();
        ctx.for_each_item(depth, items, |item, index| {
            key.clear();
            match &self.key {
                None => ssr::write_into(&mut key, format_args!("#{index}")),
                Some(expr) => match ssr::evaluate(expr, item) {
                    Some(value) => ssr::write_js_json(&mut key, &value),
                    None => return Some(()),
                },
            }
            self.write(out, Some(item), Some(&key));
            Some(())
        });
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
        fn eval_text<'v>(expr: &'v Value, ctx: Option<&Ctx<'v>>) -> Option<Cow<'v, str>> {
            ctx.and_then(|ctx| ssr::evaluate(expr, ctx))
                .map(ssr::js_str)
        }
        let eval_bool =
            |expr: &Value| ctx.and_then(|ctx| ssr::evaluate(expr, ctx).map(|v| ssr::truthy(&v)));
        let mut style: Vec<String> = Vec::new();
        let mut rule = String::new();
        let rules = ctx.and_then(|ctx| ctx.rules);
        for (property, value) in &self.style {
            match eval_text(value.as_json(), ctx) {
                // A value that could end the declaration, the rule, or the
                // `<style>` element is left for the runtime to set, and the
                // browser's `setProperty` drops one it cannot read as the
                // property's value.
                Some(value) if !plain_css(&value) => {}
                Some(value) if rules.is_none() => style.push(format!("{property}: {value}")),
                Some(value) => {
                    rule.push_str(property);
                    rule.push(':');
                    rule.push_str(&value);
                    rule.push_str("!important;");
                }
                _ => {}
            }
        }
        let mut computed: Vec<(&str, Cow<'_, str>)> = Vec::new();
        if let Some(rules) = rules
            && !rule.is_empty()
        {
            let mut rules = rules.borrow_mut();
            computed.push((INLINE, Cow::Owned(rules.len().to_string())));
            rules.push(rule);
        }
        // A strict Content-Security-Policy ignores a `style` attribute, so a
        // hidden element carries `HIDE`, which a rule `Page` writes hides.
        if let Some(show) = &self.show
            && eval_bool(show.as_json()) == Some(false)
        {
            computed.push((HIDE, Cow::Borrowed("")));
        }
        for (name, value) in &self.attr_bind {
            if let Some(value) = eval_text(value.as_json(), ctx) {
                computed.push((name, value));
            }
        }
        for (name, when) in &self.attr_when {
            if eval_bool(when.as_json()) == Some(true) {
                computed.push((name, Cow::Borrowed("")));
            }
        }
        if let Some(checked) = &self.checked
            && eval_bool(checked.as_json()) == Some(true)
        {
            computed.push(("checked", Cow::Borrowed("")));
        }
        let mut content: Option<Cow<'_, str>> = self
            .text
            .as_ref()
            .and_then(|text| eval_text(text.as_json(), ctx));
        let mut selected: Option<String> = None;
        if let (Some(ctx), Some(bind)) = (ctx, &self.bind)
            && let Some(value) = bind.get(0).and_then(|place| ctx.lookup(place))
        {
            match bind.get(1).and_then(Value::as_str) {
                Some("bool") if ssr::truthy(value) => {
                    computed.push(("checked", Cow::Borrowed("")));
                }
                Some("radio") if bind.get(2).is_some_and(|choice| ssr::equal(choice, value)) => {
                    computed.push(("checked", Cow::Borrowed("")));
                }
                Some("json") => selected = Some(ssr::js_json(value)),
                Some("text" | "int" | "float" | "opt_text" | "opt_int" | "opt_float")
                    if !value.is_null() =>
                {
                    let text = ssr::js_str(Val::Ref(value));
                    match self.tag {
                        "textarea" => content = Some(text),
                        "select" => selected = Some(text.into_owned()),
                        _ => computed.push(("value", text)),
                    }
                }
                _ => {}
            }
        }

        out.push('<');
        out.push_str(self.tag);
        if let Some(id) = &self.id {
            write_attr(out, "id", id.as_str());
        }
        let classes = self
            .classes
            .iter()
            .copied()
            .chain(
                self.class_when
                    .iter()
                    .filter(|(_, when)| eval_bool(when.as_json()) == Some(true))
                    .map(|(class, _)| *class),
            )
            .chain(self.css_names.iter().map(String::as_str))
            .chain(
                self.css_when
                    .iter()
                    .filter(|(_, _, when)| eval_bool(when.as_json()) == Some(true))
                    .flat_map(|(_, names, _)| names.iter().map(String::as_str)),
            );
        let mut any_class = false;
        for class in classes {
            out.push_str(if any_class { " " } else { " class=\"" });
            escape_into(out, class, Escape::Attribute);
            any_class = true;
        }
        if any_class {
            out.push('"');
        }
        for (name, value) in &self.attrs {
            write_attr(out, name, value);
        }
        if !style.is_empty() {
            write_attr(out, "style", &style.join("; "));
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
            escape_into(out, &text, Escape::Text);
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
        if let (true, Some(name)) = (self.foreign, self.node_ref) {
            write_attr(out, "data-hs-foreign", name);
        }
        if let Some(name) = self.file_input {
            write_attr(out, "data-hs-files", name);
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
            write_attr(
                out,
                "data-hs-on-interval",
                &json_text(&(period, stmt.as_json())),
            );
        }
        if let Some((delay, stmt)) = &self.timeout {
            write_attr(
                out,
                "data-hs-on-timeout",
                &json_text(&(delay, stmt.as_json())),
            );
        }
        for (event, stmt) in &self.on {
            write_attr(out, event.attribute(), &stmt.to_json());
        }
        if !self.custom.is_empty() {
            let custom: Vec<(&str, u8, &Value)> = self
                .custom
                .iter()
                .map(|(name, depth, stmt)| (*name, *depth, stmt.as_json()))
                .collect();
            write_attr(out, "data-hs-on-custom", &json_text(&custom));
        }
        if !self.keys.is_empty() {
            write_attr(out, "data-hs-on-key", &keys_json(&self.keys));
        }
        if let Some(orientation) = self.roving {
            write_attr(out, "data-hs-roving", orientation.name());
        }
        if let Some((listbox, place)) = &self.active {
            write_attr(out, "data-hs-active", &json_text(&(listbox, place)));
        }
    }

    fn write_selected(&self, out: &mut String, ctx: Option<&Ctx<'_>>) {
        let mut out_el = String::new();
        self.write(&mut out_el, ctx, None);
        let open = '<'.len_utf8() + self.tag.len();
        out.push_str(&out_el[..open]);
        out.push_str(" selected=\"\"");
        out.push_str(&out_el[open..]);
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
    escape_into(out, value, Escape::Attribute);
    out.push('"');
}

/// Writes an attribute whose value is `value` as JSON, already escaped for
/// HTML, without building the unescaped text first.
pub(crate) fn write_json_attr(out: &mut String, name: &str, value: &impl Serialize) {
    out.push(' ');
    out.push_str(name);
    out.push_str("=\"");
    // `serde_json` writes bytes, so `out` is lent to it as a `Vec<u8>` and
    // checked as UTF-8 once when it comes back. The check reads all of
    // `out`, so its cost grows with what the caller has already written.
    let mut bytes = std::mem::take(out).into_bytes();
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, AttrJson);
    value
        .serialize(&mut serializer)
        .expect("JSON values and strings serialize");
    *out = String::from_utf8(bytes).expect("serde_json and AttrJson write UTF-8");
    out.push('"');
}

/// Compact JSON with the HTML escapes of an attribute value written in
/// place: the quotes around a string as `&quot;`, and `&`, `<`, `>`, and
/// `'` inside it as entities. JSON escapes a `"` inside a string as `\"`,
/// so that one is written as `\&quot;`.
struct AttrJson;

impl serde_json::ser::Formatter for AttrJson {
    fn begin_string<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"&quot;")
    }

    fn end_string<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"&quot;")
    }

    fn write_string_fragment<W: ?Sized + io::Write>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> io::Result<()> {
        let mut start = 0;
        for (index, byte) in fragment.bytes().enumerate() {
            let entity: &[u8] = match byte {
                b'&' => b"&amp;",
                b'<' => b"&lt;",
                b'>' => b"&gt;",
                b'\'' => b"&#39;",
                _ => continue,
            };
            writer.write_all(&fragment.as_bytes()[start..index])?;
            writer.write_all(entity)?;
            start = index + 1;
        }
        writer.write_all(&fragment.as_bytes()[start..])
    }

    fn write_char_escape<W: ?Sized + io::Write>(
        &mut self,
        writer: &mut W,
        char_escape: serde_json::ser::CharEscape,
    ) -> io::Result<()> {
        match char_escape {
            serde_json::ser::CharEscape::Quote => writer.write_all(b"\\&quot;"),
            other => serde_json::ser::CompactFormatter.write_char_escape(writer, other),
        }
    }
}

/// Where escaped text lands, which decides whether quotes are escaped.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Escape {
    /// The content of an element.
    Text,
    /// A quoted attribute value.
    Attribute,
}

pub(crate) fn escape_into(out: &mut String, text: &str, to: Escape) {
    let attribute = to == Escape::Attribute;
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
    use super::{Attr, Candidate, Class, attr, button, div, el, img, input, picture, source, span};
    use crate::_private::sig;
    use crate::style::Token;
    use std::time::Duration;

    use crate::{SafeUrl, Sig, Stmt};

    struct Page;

    fn count() -> Sig<Page, i32> {
        sig("count")
    }

    fn site() -> Sig<Page, SafeUrl> {
        sig("site")
    }

    #[test]
    #[should_panic(expected = "the attribute `title` is set twice on one `<div>`")]
    fn an_attribute_set_twice_panics() {
        let _ = div::<Page>().attr(attr::TITLE, "a").attr(attr::TITLE, "b");
    }

    #[test]
    #[should_panic(expected = "`El::show` is called twice on one `<div>`")]
    fn a_second_show_panics() {
        let _ = div::<Page>().show(true).show(false);
    }

    #[test]
    #[should_panic(expected = "`El::bind_text` is called twice on one `<p>`")]
    fn a_second_text_binding_panics() {
        let _ = super::p::<Page>().bind_text(name()).bind_text(name());
    }

    #[test]
    #[should_panic(expected = "`El::id` is called twice on one `<div>`")]
    fn a_second_id_panics() {
        let _ = div::<Page>()
            .id(super::Id::new("a"))
            .id(super::Id::new("b"));
    }

    #[test]
    #[should_panic(expected = "a `<input>` is bound twice, by `El::bind`")]
    fn a_second_binding_panics() {
        let _ = input::<Page>().bind(name()).bind(name());
    }

    #[test]
    #[should_panic(
        expected = "`El::bind` binds `<input>` or `<textarea>` or `<select>`, not a `<div>`"
    )]
    fn a_binding_on_an_element_the_runtime_cannot_bind_panics() {
        let _ = div::<Page>().bind(name());
    }

    #[test]
    fn a_control_built_by_its_tag_binds() {
        let html = el::<Page>(crate::tag!("TEXTAREA")).bind(name()).render();
        assert!(html.contains("data-hs-bind"), "{html}");
    }

    #[test]
    fn a_timer_waits_up_to_2_to_the_31_milliseconds() {
        let longest = Duration::from_millis((1 << 31) - 1);
        let html = div::<Page>()
            .on_timeout(longest, Stmt::new(serde_json::json!(["seq", []])))
            .render();
        assert!(html.contains("2147483647"), "{html}");
    }

    #[test]
    #[should_panic(expected = "the time `on_timeout` waits is at most 2^31 - 1 milliseconds")]
    fn a_timeout_the_browser_would_run_at_once_is_refused() {
        let _ = div::<Page>().on_timeout(
            Duration::from_millis(1 << 31),
            Stmt::new(serde_json::json!(["seq", []])),
        );
    }

    #[test]
    #[should_panic(expected = "the time `on_debounced` waits is at most 2^31 - 1 milliseconds")]
    fn a_debounce_past_any_timer_is_refused() {
        let _ = div::<Page>().on_debounced(
            super::Event::Input,
            Duration::MAX,
            Stmt::new(serde_json::json!(["seq", []])),
        );
    }

    #[test]
    #[should_panic(expected = "the period of `on_interval` is at least 1 millisecond")]
    fn an_interval_of_no_time_is_refused() {
        let _ =
            div::<Page>().on_interval(Duration::ZERO, Stmt::new(serde_json::json!(["seq", []])));
    }

    #[test]
    fn a_link_or_an_image_takes_its_url_from_a_signal() {
        let html = super::a::<Page>().href(site()).render();
        assert!(html.contains("data-hs-attr"), "{html}");
        let html = super::video::<Page>().poster(site()).render();
        assert!(html.contains("data-hs-attr"), "{html}");
    }

    #[test]
    #[should_panic(expected = "the `src` of a `<IFrame>` shows a page inside this one")]
    fn a_frame_takes_no_url_from_a_signal() {
        let _ = el::<Page>(crate::tag!("IFrame")).src(site());
    }

    #[test]
    fn a_frame_takes_a_fixed_url() {
        let html = el::<Page>(crate::tag!("iframe"))
            .src(SafeUrl::new("https://example.com/embed"))
            .render();
        assert_eq!(html, r#"<iframe src="https://example.com/embed"></iframe>"#);
    }

    #[test]
    fn srcset_writes_each_url_with_its_width_or_density() {
        let html = img::<Page>()
            .srcset([
                (SafeUrl::new("/a.png"), Candidate::Density(1.0)),
                (SafeUrl::new("/a@1.5.png"), Candidate::Density(1.5)),
            ])
            .render();
        assert_eq!(html, r#"<img srcset="/a.png 1x, /a@1.5.png 1.5x">"#);
    }

    #[test]
    #[should_panic(expected = "a pixel density is a finite number above 0, not NaN")]
    fn srcset_refuses_a_density_that_is_not_a_number() {
        let _ = img::<Page>().srcset([(SafeUrl::new("/a.png"), Candidate::Density(f64::NAN))]);
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
    fn a_void_tag_has_no_closing_tag_from_its_constructor_or_from_el() {
        let html = picture::<Page>()
            .child(
                source().srcset([(crate::SafeUrl::new("/a.avif"), super::Candidate::Width(480))]),
            )
            .child(el(crate::tag!("meta")).attr(attr::ITEMPROP, "name"))
            .child(img().src(crate::SafeUrl::new("/a.png")))
            .render();
        assert_eq!(
            html,
            r#"<picture><source srcset="/a.avif 480w"><meta itemprop="name"><img src="/a.png"></picture>"#
        );
    }

    #[test]
    fn a_stylesheet_cannot_close_its_style_element() {
        assert!(super::closes_style(b"a{}</STYLE><script>"));
        assert!(!super::closes_style(b"a{content:\"</styl\"}"));
    }

    #[test]
    #[should_panic(expected = "`El::href`, `El::src`, and `El::poster` set")]
    fn an_attr_cannot_name_href() {
        let _ = Attr::new("href");
    }

    #[test]
    #[should_panic(expected = "`El::href`, `El::src`, and `El::poster` set")]
    fn an_attr_cannot_name_src_in_any_case() {
        let _ = Attr::new("SRC");
    }

    #[test]
    #[should_panic(expected = "write the `data-hs-` attributes")]
    fn an_attr_cannot_name_a_runtime_attribute_in_any_case() {
        let _ = Attr::new("DATA-HS-ON-CLICK");
    }

    #[test]
    #[should_panic(expected = "an attribute name is letters")]
    fn an_attr_cannot_start_another_attribute() {
        let _ = Attr::new("x onerror");
    }

    #[test]
    #[should_panic(expected = "an `on*` attribute runs a script")]
    fn an_attr_cannot_name_an_event_handler_in_any_case() {
        let _ = Attr::new("onError");
    }

    #[test]
    fn a_plain_css_value_cannot_leave_its_declaration() {
        for value in [
            "50%",
            "calc(100% - var(--a, 1px))",
            "rgb(0 0 0 / 50%)",
            "#fff",
        ] {
            assert!(super::plain_css(value), "{value}");
        }
        for value in [
            "1px}a{b:c",
            "a;b:c",
            "</style>",
            "url(x",
            "a)",
            "\"",
            "/* x",
            "a\\b",
            "a\nb",
        ] {
            assert!(!super::plain_css(value), "{value}");
        }
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
    fn writes_json_attributes_as_escaping_the_json_text_would() {
        let value = serde_json::json!({
            "a\"b": ["<&>'", "\\ \u{1}\n\t", "日本😀", 1.5, -3, null, true],
            "": { "x": "</script>" },
        });
        let mut streamed = String::new();
        super::write_json_attr(&mut streamed, "data-x", &value);
        let mut escaped = String::new();
        super::write_attr(&mut escaped, "data-x", &value.to_string());
        assert_eq!(streamed, escaped);
    }

    #[test]
    fn escapes_text_and_attributes() {
        let html = div::<Page>()
            .attr(attr::TITLE, "a\"b<")
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

    mod item_reads {
        use super::super::{div, each, li, span, ul};
        use super::{Page, sig};
        use crate::{Expr, Sig, Stmt};

        struct Task;

        fn tasks() -> Sig<Page, Vec<Task>> {
            sig("tasks")
        }

        fn done() -> Sig<Page, Vec<Task>> {
            sig("done")
        }

        fn title() -> Sig<Task, String> {
            sig("title")
        }

        fn tags() -> Sig<Task, Vec<String>> {
            sig("tags")
        }

        #[test]
        #[should_panic(expected = "reads the item at depth 0 of a list that does not enclose it")]
        fn an_expression_built_in_a_closure_and_bound_outside_its_list_panics() {
            let mut kept: Option<Expr<Page, String>> = None;
            let list = each(tasks(), |task| {
                kept = Some(task.at(title()).expr());
                li()
            });
            let text = kept.expect("`each` runs its closure before it returns");
            let _ = div::<Page>()
                .child(list)
                .child(span().bind_text(text))
                .render();
        }

        #[test]
        #[should_panic(expected = "reads the item at depth 0 of a list that does not enclose it")]
        fn a_statement_built_in_a_closure_and_run_outside_its_list_panics() {
            let mut kept: Option<Stmt<Page>> = None;
            let list = each(tasks(), |task| {
                kept = Some(task.remove());
                li()
            });
            let remove = kept.expect("`each` runs its closure before it returns");
            let _ = div::<Page>()
                .child(ul().child(list))
                .child(super::button().on_click(remove))
                .render();
        }

        #[test]
        #[should_panic(expected = "reads the item at depth 1 of a list that does not enclose it")]
        fn an_inner_item_used_in_the_outer_list_panics() {
            let mut kept: Option<Expr<Page, String>> = None;
            let _ = ul::<Page>()
                .child(each(tasks(), |task| {
                    let tags = each(task.at(tags()), |tag| {
                        kept = Some(tag.value().expr());
                        span()
                    });
                    li().child(tags)
                        .child(span().bind_text(kept.take().unwrap()))
                }))
                .render();
        }

        #[test]
        fn items_read_inside_their_lists_and_list_operations_render() {
            let html = ul::<Page>()
                .child(each(tasks(), |task| {
                    let tagged = task.at(tags()).count(|tag| !tag.value().is_empty());
                    li().bind_text(task.at(title()))
                        .child(each(task.at(tags()), |tag| {
                            span()
                                .bind_text(tag.value().concat(task.at(title())))
                                .on_click(
                                    task.at(tags())
                                        .retain(|other| !other.value().equals(tag.value())),
                                )
                        }))
                        .child(span().bind_text(tagged.to_text()))
                        .on_click(task.remove())
                }))
                .child(span().on_click(done().sort_by(|task| task.at(title()))))
                .render();
            assert!(html.contains("data-hs-each"), "{html}");
        }
    }

    mod names {
        use super::super::{Id, each, li, ul, when};
        use super::{Page, count, div, input, sig};
        use crate::{FileInput, NodeRef, Sig};

        const FIELD: NodeRef<Page> = NodeRef::new("field");
        const PICTURE: FileInput<Page> = FileInput::new("field");

        fn rows() -> Sig<Page, Vec<String>> {
            sig("rows")
        }

        #[test]
        #[should_panic(expected = "node ref name `field`")]
        fn one_node_ref_on_two_elements_of_the_page_panics() {
            let _ = div::<Page>()
                .child(input().node_ref(FIELD))
                .child(input().node_ref(FIELD))
                .render();
        }

        #[test]
        #[should_panic(expected = "file input name `field`")]
        fn one_file_input_twice_panics() {
            let _ = div::<Page>()
                .child(PICTURE.input())
                .child(PICTURE.input())
                .render();
        }

        #[test]
        fn each_item_and_sibling_lists_may_carry_one_name() {
            let item = || each(rows(), |_| li().child(input().node_ref(FIELD)));
            let html = div::<Page>()
                .child(ul().child(item()))
                .child(ul().child(item()))
                .render();
            assert!(html.contains("data-hs-ref"), "{html}");
        }

        #[test]
        fn a_node_ref_and_a_file_input_may_share_a_name() {
            let _ = div::<Page>()
                .child(input().node_ref(FIELD))
                .child(PICTURE.input())
                .render();
        }

        #[test]
        #[should_panic(expected = "node ref name `field`")]
        fn a_name_in_a_list_and_on_the_page_around_it_panics() {
            let _ = div::<Page>()
                .child(ul().child(each(rows(), |_| li().child(input().node_ref(FIELD)))))
                .child(input().node_ref(FIELD))
                .render();
        }

        #[test]
        #[should_panic(expected = "node ref name `field`")]
        fn one_name_twice_in_one_item_panics() {
            let _ = ul::<Page>()
                .child(each(rows(), |_| {
                    li().child(input().node_ref(FIELD))
                        .child(input().node_ref(FIELD))
                }))
                .render();
        }

        #[test]
        #[should_panic(expected = "node ref name `field`")]
        fn a_conditional_copy_and_the_page_around_it_panics() {
            let _ = div::<Page>()
                .child(when(count().gt(0), input().node_ref(FIELD)))
                .child(input().node_ref(FIELD))
                .render();
        }

        #[test]
        #[should_panic(expected = "id `total`")]
        fn one_id_twice_panics() {
            let _ = div::<Page>()
                .child(div().id(Id::new("total")))
                .child(div().id(Id::new("total")))
                .render();
        }

        #[test]
        #[should_panic(expected = "list template carries the fixed id `row`")]
        fn a_fixed_id_in_a_list_template_panics() {
            let _ = ul::<Page>()
                .child(each(rows(), |_| li().id(Id::new("row"))))
                .render();
        }

        #[test]
        #[should_panic(expected = "carry the id `row-7`")]
        fn an_id_with_a_key_next_to_the_same_id_written_whole_panics() {
            const ROW: Id = Id::new("row");
            let _ = div::<Page>()
                .child(li().id(Id::new("row-7")))
                .child(li().id(ROW.with_key(7)))
                .render();
        }

        #[test]
        #[should_panic(expected = "carry the id `tag-a`")]
        fn two_ids_with_one_key_panic() {
            const TAG: Id = Id::new("tag");
            let _ = div::<Page>()
                .child(li().id(TAG.with_key("a")))
                .child(li().id(TAG.with_key("a")))
                .render();
        }

        #[test]
        fn ids_from_text_may_repeat() {
            let html = div::<Page>()
                .child(div().id(Id::from_text("Intro")))
                .child(div().id(Id::from_text("Intro").with_key(1)))
                .child(div().id(Id::from_text("Intro").with_key(1)))
                .child(div().id(Id::from_text("Intro")))
                .render();
            assert_eq!(html.matches(r#"id="Intro""#).count(), 2, "{html}");
        }

        #[test]
        fn sibling_conditional_copies_may_carry_one_id() {
            let _ = div::<Page>()
                .child(when(count().gt(0), div().id(Id::new("total"))))
                .child(when(count().le(0), div().id(Id::new("total"))))
                .render();
        }
    }

    #[test]
    #[should_panic(expected = "can run a script or load a URL")]
    fn an_attr_cannot_name_an_animation_target() {
        let _ = Attr::new("attributeName");
    }

    #[test]
    #[should_panic(expected = "can run a script or load a URL")]
    fn an_attr_cannot_name_http_equiv_in_any_case() {
        let _ = Attr::new("HTTP-EQUIV");
    }

    #[test]
    fn an_attr_keeps_the_case_it_was_given() {
        let html = el::<Page>(crate::tag!("animate"))
            .attr(attr::TO, "0")
            .render();
        assert_eq!(html, r#"<animate to="0"></animate>"#);
        assert_eq!(attr::VIEW_BOX.name(), "viewBox");
    }
}
