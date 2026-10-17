//! Signal structs and typed references to their fields.

use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::expr::{Comparable, Expr, IntoExpr, Stmt};
use crate::patch::SignalPatch;

/// A struct whose fields are signals.
///
/// Derive it with `#[derive(Signals)]` next to serde's `Serialize` and
/// `Deserialize`. The derive adds one associated function per field, named
/// after the field, that returns a [`Sig`] for it:
///
/// ```
/// use hypersig::{Sig, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Counter {
///     count: i32,
/// }
///
/// let count: Sig<Counter, i32> = Counter::count();
/// assert_eq!(count.path(), "count");
/// ```
///
/// A page has one signal struct, which the browser keeps as one JSON object.
/// An action receives only the arguments its call passes, so a handler that
/// needs a signal's value takes it as an argument. A field may itself be a
/// `Signals` struct or a `Vec` of one; [`Sig::at`] reaches into it.
pub trait Signals: Serialize + DeserializeOwned + Send + 'static {}

/// The deepest chain of fields a [`Sig`] can name.
pub const MAX_DEPTH: usize = 8;

/// Marks a [`Sig`] whose path starts at the page's signal struct.
///
/// Only a page-rooted signal has a fixed path, so only it can be patched from
/// the server.
#[derive(Debug, Clone, Copy)]
pub enum Root {}

/// Marks a [`Sig`] whose path starts at the current item of an
/// [`each`](crate::html::each) or a list predicate. Its path is resolved in the browser per item.
#[derive(Debug, Clone, Copy)]
pub enum Local {}

/// A chain of field names, either from the page root or from a list item.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Path {
    item: Option<u8>,
    segments: [&'static str; MAX_DEPTH],
    len: u8,
}

impl Path {
    pub(crate) fn field(name: &'static str) -> Self {
        assert!(!name.is_empty(), "a signal path is never empty");
        let mut segments = [""; MAX_DEPTH];
        segments[0] = name;
        Self {
            item: None,
            segments,
            len: 1,
        }
    }

    pub(crate) fn item(depth: u8) -> Self {
        Self {
            item: Some(depth),
            segments: [""; MAX_DEPTH],
            len: 0,
        }
    }

    fn join(mut self, tail: Self) -> Self {
        let total = usize::from(self.len) + usize::from(tail.len);
        assert!(
            total <= MAX_DEPTH,
            "a signal path is at most {MAX_DEPTH} fields deep"
        );
        for segment in tail.segments() {
            self.segments[usize::from(self.len)] = segment;
            self.len += 1;
        }
        self
    }

    fn segments(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.segments[..usize::from(self.len)].iter().copied()
    }

    pub(crate) fn dotted(&self) -> String {
        self.segments().collect::<Vec<_>>().join(".")
    }

    /// The JSON the runtime reads this path's value from.
    pub(crate) fn expr_json(&self) -> Value {
        match self.item {
            None => json!(["sig", self.dotted()]),
            Some(depth) => json!(["item", depth, self.dotted()]),
        }
    }

    /// The JSON the runtime resolves to a place it can write to.
    pub(crate) fn place_json(&self) -> Value {
        match self.item {
            None => Value::from(self.dotted()),
            Some(depth) => json!(["item", depth, self.dotted()]),
        }
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.item {
            None => write!(f, "{}", self.dotted()),
            Some(depth) => write!(f, "item[{depth}].{}", self.dotted()),
        }
    }
}

thread_local! {
    static ITEM_DEPTH: Cell<u8> = const { Cell::new(0) };
}

/// Runs `body` with the depth of a new item scope, one deeper than the
/// enclosing one, so nested lists name their items without colliding.
///
/// The depth counts enclosing scopes, so rendering the same tree twice writes
/// the same attributes.
pub(crate) fn in_item_scope<R>(body: impl FnOnce(u8) -> R) -> R {
    struct Restore(u8);
    impl Drop for Restore {
        fn drop(&mut self) {
            ITEM_DEPTH.set(self.0);
        }
    }
    let depth = ITEM_DEPTH.get();
    let _restore = Restore(depth);
    ITEM_DEPTH.set(
        depth
            .checked_add(1)
            .expect("item scopes nest fewer than 256 deep"),
    );
    body(depth)
}

type Marker<S, T, R> = fn() -> (S, T, R);

/// A typed reference to a signal of type `T` inside the signal struct `S`.
///
/// `R` is [`Root`] for a signal reached from the page root, and [`Local`] for
/// one reached from a list item. Values come only from the accessor functions
/// that `#[derive(Signals)]` generates, combined with [`Sig::at`], so the path
/// always names a real field.
pub struct Sig<S, T, R = Root> {
    path: Path,
    marker: PhantomData<Marker<S, T, R>>,
}

impl<S, T, R> Sig<S, T, R> {
    pub(crate) fn from_path(path: Path) -> Self {
        Self {
            path,
            marker: PhantomData,
        }
    }

    pub(crate) fn raw_path(self) -> Path {
        self.path
    }

    /// The field `field` of the struct this signal holds.
    ///
    /// ```
    /// use hypersig::{Sig, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Panel {
    ///     open: bool,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Faq {
    ///     shipping: Panel,
    /// }
    ///
    /// let open: Sig<Faq, bool> = Faq::shipping().at(Panel::open());
    /// assert_eq!(open.path(), "shipping.open");
    /// ```
    #[must_use]
    pub fn at<U>(self, field: Sig<T, U>) -> Sig<S, U, R> {
        Sig::from_path(self.path.join(field.path))
    }

    /// A statement that assigns `value` to this signal in the browser.
    #[must_use]
    pub fn set(self, value: impl IntoExpr<S, T>) -> Stmt<S> {
        Stmt::set(self.path.place_json(), value.into_expr())
    }

    pub(crate) fn expr(self) -> Expr<S, T> {
        Expr::from_json(self.path.expr_json())
    }
}

impl<S, T> Sig<S, T, Root> {
    /// The dotted field path this signal is stored under.
    #[must_use]
    pub fn path(self) -> String {
        self.path.dotted()
    }

    /// A patch that sets this signal to `value` when the server sends it.
    ///
    /// # Panics
    ///
    /// Panics if `value` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn patch(self, value: T) -> SignalPatch<S>
    where
        T: Serialize,
    {
        SignalPatch::single(self.path.dotted(), &value)
    }
}

impl<S, R> Sig<S, bool, R> {
    /// A statement that flips this flag.
    #[must_use]
    pub fn toggle(self) -> Stmt<S> {
        self.set(!self)
    }
}

impl<S, T, R> Sig<S, Vec<T>, R> {
    /// A statement that appends `value` to this list.
    #[must_use]
    pub fn push(self, value: impl IntoExpr<S, T>) -> Stmt<S> {
        Stmt::list(
            "push",
            self.path.place_json(),
            value.into_expr().into_json(),
        )
    }

    /// A statement that removes the item at `index`, if there is one.
    #[must_use]
    pub fn remove_at(self, index: impl IntoExpr<S, i32>) -> Stmt<S> {
        Stmt::list(
            "remove",
            self.path.place_json(),
            index.into_expr().into_json(),
        )
    }

    /// A statement that empties this list.
    #[must_use]
    pub fn clear(self) -> Stmt<S> {
        Stmt::set(
            self.path.place_json(),
            Expr::<S, Vec<T>>::from_json(json!(["lit", []])),
        )
    }

    /// A statement that keeps only the items for which `keep` is true.
    ///
    /// `keep` receives the item being tested and builds the predicate from it.
    #[must_use]
    pub fn retain<P: IntoExpr<S, bool>>(self, keep: impl FnOnce(Item<S, T>) -> P) -> Stmt<S> {
        let (depth, predicate) = self.predicate(keep);
        Stmt::new(json!(["retain", self.path.place_json(), depth, predicate]))
    }

    /// The number of items for which `matches` is true.
    #[must_use]
    pub fn count<P: IntoExpr<S, bool>>(
        self,
        matches: impl FnOnce(Item<S, T>) -> P,
    ) -> Expr<S, i32> {
        let (depth, predicate) = self.predicate(matches);
        Expr::from_json(json!(["count", self.path.place_json(), depth, predicate]))
    }

    /// A statement that sets `field` to `value` on every item.
    #[must_use]
    pub fn set_all<U>(self, field: Sig<T, U>, value: impl IntoExpr<S, U>) -> Stmt<S> {
        Stmt::new(json!([
            "set_all",
            self.path.place_json(),
            field.path.dotted(),
            value.into_expr().into_json()
        ]))
    }

    /// A statement that appends `count` items, building each from its position
    /// `i`, which runs from 0 up to `count`.
    #[must_use]
    pub fn extend<E: IntoExpr<S, T>>(
        self,
        count: impl IntoExpr<S, i32>,
        build: impl FnOnce(Expr<S, i32>) -> E,
    ) -> Stmt<S> {
        let (depth, item) = in_item_scope(|depth| {
            let index = Expr::from_json(json!(["index", depth]));
            (depth, build(index).into_expr().into_json())
        });
        Stmt::new(json!([
            "extend",
            self.path.place_json(),
            count.into_expr().into_json(),
            depth,
            item
        ]))
    }

    /// A statement that sets `field` to `value` on the items for which `when` is true.
    ///
    /// Both closures receive the item being updated, so `value` can read it.
    #[must_use]
    pub fn update_where<U, P: IntoExpr<S, bool>, E: IntoExpr<S, U>>(
        self,
        when: impl FnOnce(Item<S, T>) -> P,
        field: Sig<T, U>,
        value: impl FnOnce(Item<S, T>) -> E,
    ) -> Stmt<S> {
        let (depth, when, value) = in_item_scope(|depth| {
            let item = Item::new(self.path, depth);
            (
                depth,
                when(item).into_expr().into_json(),
                value(item).into_expr().into_json(),
            )
        });
        Stmt::new(json!([
            "update",
            self.path.place_json(),
            depth,
            when,
            field.path.dotted(),
            value
        ]))
    }

    /// A statement that exchanges the items at `a` and `b`, if both exist.
    #[must_use]
    pub fn swap(self, a: impl IntoExpr<S, i32>, b: impl IntoExpr<S, i32>) -> Stmt<S> {
        Stmt::new(json!([
            "swap",
            self.path.place_json(),
            a.into_expr().into_json(),
            b.into_expr().into_json()
        ]))
    }

    /// A statement that orders the items by `key`, ascending, keeping the
    /// order of items with equal keys.
    #[must_use]
    pub fn sort_by<K: Comparable, E: IntoExpr<S, K>>(
        self,
        key: impl FnOnce(Item<S, T>) -> E,
    ) -> Stmt<S> {
        let (depth, key) = in_item_scope(|depth| {
            let item = Item::new(self.path, depth);
            (depth, key(item).into_expr().into_json())
        });
        Stmt::new(json!(["sort", self.path.place_json(), depth, key]))
    }

    fn predicate<P: IntoExpr<S, bool>>(self, build: impl FnOnce(Item<S, T>) -> P) -> (u8, Value) {
        in_item_scope(|depth| {
            let item = Item::new(self.path, depth);
            (depth, build(item).into_expr().into_json())
        })
    }
}

impl<S, T, R> Clone for Sig<S, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T, R> Copy for Sig<S, T, R> {}

impl<S, T, R> fmt::Debug for Sig<S, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Sig").field(&self.path).finish()
    }
}

/// The current item of a list, inside [`each`](crate::html::each) or a list
/// predicate such as [`Sig::count`].
///
/// An `Item` belongs to the closure it was passed to. The browser resolves it
/// against the item being rendered or tested, so an expression built from it
/// means nothing outside that closure. The type system does not stop one from
/// being moved out.
pub struct Item<S, T> {
    list: Path,
    depth: u8,
    marker: PhantomData<fn() -> (S, T)>,
}

impl<S, T> Item<S, T> {
    pub(crate) fn new(list: Path, depth: u8) -> Self {
        Self {
            list,
            depth,
            marker: PhantomData,
        }
    }

    /// The item itself, as a signal that can be read and written.
    #[must_use]
    pub fn value(self) -> Sig<S, T, Local> {
        Sig::from_path(Path::item(self.depth))
    }

    /// The field `field` of the item.
    #[must_use]
    pub fn at<U>(self, field: Sig<T, U>) -> Sig<S, U, Local> {
        self.value().at(field)
    }

    /// The item's position in its list, starting at 0.
    #[must_use]
    pub fn index(self) -> Expr<S, i32> {
        Expr::from_json(json!(["index", self.depth]))
    }

    /// A statement that removes this item from its list.
    #[must_use]
    pub fn remove(self) -> Stmt<S> {
        Stmt::list("remove", self.list.place_json(), self.index().into_json())
    }
}

impl<S, T> Clone for Item<S, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T> Copy for Item<S, T> {}

impl<S, T> fmt::Debug for Item<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Item")
            .field("list", &self.list)
            .field("depth", &self.depth)
            .finish()
    }
}

impl<S, T> IntoExpr<S, T> for Item<S, T> {
    fn into_expr(self) -> Expr<S, T> {
        self.value().expr()
    }
}
