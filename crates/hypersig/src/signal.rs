//! Signal structs and typed references to their fields.

use std::cell::{Cell, RefCell};
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::expr::{Comparable, Expr, IntoExpr, IntoStmt, Number, Stmt};
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
/// A field marked `#[signals(computed)]` gets a `Sig<Self, T, Computed>`
/// instead, for a value that [`Page::computed`](crate::Page::computed) keeps
/// equal to an expression ([`Computed`]).
///
/// A page has one signal struct, which the browser keeps as one JSON object.
/// An action receives only the arguments its call passes, so a handler that
/// needs a signal's value takes it as an argument. A field may itself be a
/// `Signals` struct or a `Vec` of one; [`Sig::at`] reaches into it.
pub trait Signals: Serialize + DeserializeOwned + Send + 'static {
    /// The struct's field names and types, which `#[derive(Signals)]` writes.
    ///
    /// It reads `name:Type` for each field in declaration order, joined by
    /// `,`, with the type written out token by token, as `Vec < Todo >`
    /// for a `Vec<Todo>`. A page served by a debug
    /// build carries it, and after `cargo hypersig dev` restarts the server,
    /// the reloaded tab restores the signals the visitor changed only where
    /// the new page's struct has the same shape. A field whose type is
    /// another `Signals` struct contributes only that type's name, so a
    /// change inside the nested struct alone is not seen, and the value
    /// restored into that field keeps its old fields until the next reload.
    /// An impl written by hand leaves it empty, and its page restores
    /// nothing.
    const SHAPE: &'static str = "";
}

/// A piece of page written once against its own signals, which any page can embed.
///
/// A sign-out button in an app's shell, a newsletter form in a site's
/// footer, or a cart badge in a shared header appear on several pages,
/// whose signal structs differ. Write such a piece as a `Part`: its own
/// signal struct, an [`El`](crate::html::El) over it, and actions over it.
/// A page embeds it with [`html::part`](crate::html::part) at a field of its
/// own struct whose type is the part's struct. The part's actions are lines
/// of the application's [`routes!`](crate::routes) block like any other, so
/// each is registered once, and its one handler serves every page that
/// embeds the part.
///
/// The part's paths are written under that field. A call made inside the
/// part tells the server the field, so the patches its handler answers with
/// land there, and a [`Fragment`](crate::Fragment) of the part re-renders
/// with its paths under it.
///
/// `render` takes no arguments, so nothing built outside it can reach the
/// part, and what the part shows comes from its own signals, which the page
/// fills in as the value of its field. Two fields of one page may hold the
/// same part as long as the part names no element id, node ref, or file
/// input, because [`Page::render`](crate::Page::render) panics on two
/// elements with one of those.
///
/// ```
/// use hypersig::html::{El, button, div, part, span};
/// use hypersig::{Action, Page, Part, Patches, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Account {
///     name: String,
///     signing_out: bool,
/// }
///
/// hypersig::routes! {
///     fn pages(Router<()>);
///     // One handler serves every page that embeds `Account`.
///     action SIGN_OUT: Action<Account> = post "/sign-out" => sign_out;
/// }
///
/// impl Part for Account {
///     fn render() -> El<Self> {
///         div()
///             .child(span().bind_text(Account::name()))
///             .child(button().on_click(SIGN_OUT.call().indicator(Account::signing_out())))
///     }
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Inbox {
///     account: Account,
///     unread: i32,
/// }
///
/// let account = Account { name: "ada".into(), signing_out: false };
/// let html = Page::new("Inbox", Inbox { account, unread: 3 })
///     .child(part(Inbox::account()))
///     .render();
/// assert!(html.contains(r#"[&quot;sig&quot;,&quot;account.name&quot;]"#));
///
/// async fn sign_out((): (), (): ()) -> Patches<Account> {
///     Account::signing_out().patch(false).into()
/// }
/// let _app: axum::Router = pages(axum::Router::new());
/// ```
pub trait Part: Signals {
    /// Renders the part, with its paths written under the field it is embedded at.
    fn render() -> crate::html::El<Self>;
}

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

/// Marks a [`Sig`] whose value [`Page::computed`](crate::Page::computed) keeps up to date.
///
/// `#[derive(Signals)]` gives a field marked `#[signals(computed)]` an
/// accessor that returns a `Sig<S, T, Computed>`. Such a signal can be read
/// like any other, but it has no `set`, `toggle`, list statement, or `patch`,
/// because the browser would overwrite the written value the next time a
/// signal the expression reads changes.
///
/// ```compile_fail
/// use hypersig::Signals;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Cart {
///     price: f64,
///     #[signals(computed)]
///     total: f64,
/// }
///
/// let _ = Cart::total().set(0.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Computed {}

/// The markers of a [`Sig`] the browser may write: [`Root`] and [`Local`].
///
/// [`Computed`] lacks it, so a statement that writes a computed signal does
/// not compile. The trait is sealed.
#[diagnostic::on_unimplemented(
    message = "a computed signal cannot be written",
    label = "this signal is kept equal to the expression `Page::computed` gives it",
    note = "write one of the signals the expression reads instead"
)]
pub trait Writable: sealed::Sealed {}

impl Writable for Root {}
impl Writable for Local {}

/// The marker of a [`Sig`] that [`Page::computed`](crate::Page::computed) accepts: [`Computed`] alone.
///
/// It exists so that passing a field without `#[signals(computed)]` names
/// the missing attribute. The trait is sealed.
#[diagnostic::on_unimplemented(
    message = "`Page::computed` takes a field marked `#[signals(computed)]`",
    label = "this signal can be written, so a computed value would not stay in it",
    note = "add `#[signals(computed)]` to the field in the struct's definition"
)]
pub trait IsComputed: sealed::Sealed {}

impl IsComputed for Computed {}

/// The markers of a [`Sig`] whose path starts at the page's signal struct: [`Root`] and [`Computed`].
///
/// [`Local`] lacks it, because the path of a list item is resolved in the
/// browser. The trait is sealed.
pub trait FromPage: sealed::Sealed {}

impl FromPage for Root {}
impl FromPage for Computed {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Root {}
    impl Sealed for super::Local {}
    impl Sealed for super::Computed {}
}

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

    /// The fields joined by dots, relative to the struct the path starts at.
    pub(crate) fn dotted(&self) -> String {
        self.segments().collect::<Vec<_>>().join(".")
    }

    /// The fields joined by dots and written under the field of the
    /// [`Part`](crate::Part) being built, for a path from the root.
    pub(crate) fn rooted(&self) -> String {
        let dotted = self.dotted();
        match current_part() {
            Some(part) if self.item.is_none() => format!("{part}.{dotted}"),
            _ => dotted,
        }
    }

    /// The JSON the runtime reads this path's value from.
    pub(crate) fn expr_json(&self) -> Value {
        match self.item {
            None => json!(["sig", self.rooted()]),
            Some(depth) => json!(["item", depth, self.dotted()]),
        }
    }

    /// The JSON the runtime resolves to a place it can write to.
    pub(crate) fn place_json(&self) -> Value {
        match self.item {
            None => Value::from(self.rooted()),
            Some(depth) => json!(["item", depth, self.dotted()]),
        }
    }
}

thread_local! {
    static PART: RefCell<Option<Arc<str>>> = const { RefCell::new(None) };
}

/// The dotted path of the page field the [`Part`](crate::Part) being built
/// or answered sits under, or `None` outside a part.
pub(crate) fn current_part() -> Option<Arc<str>> {
    PART.with_borrow(Clone::clone)
}

/// Runs `body` with paths from the root written under `part`, and with
/// `None` writing them from the page root.
///
/// The previous part is put back when `body` returns or panics, so a part
/// inside a part nests.
pub(crate) fn in_part<R>(part: Option<Arc<str>>, body: impl FnOnce() -> R) -> R {
    struct Restore(Option<Arc<str>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            PART.set(self.0.take());
        }
    }
    let _restore = Restore(PART.replace(part));
    body()
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
/// `R` is [`Root`] for a signal reached from the page root, [`Local`] for
/// one reached from a list item, and [`Computed`] for a field marked
/// `#[signals(computed)]`. Values come only from the accessor functions
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

    /// A statement that runs `body` with this signal's value when the statement runs.
    ///
    /// It is [`Expr::let_in_stmt`] on the signal's value, so statements that
    /// run later inside `body`, such as a call's
    /// [`on_error`](crate::Call::on_error), read the value as it was then.
    #[must_use]
    pub fn let_in_stmt<B: IntoStmt<S>>(self, body: impl FnOnce(Expr<S, T>) -> B) -> Stmt<S> {
        self.expr().let_in_stmt(body)
    }

    pub(crate) fn expr(self) -> Expr<S, T> {
        Expr::from_json(self.path.expr_json())
    }

    /// The expression that reads this signal, which the methods
    /// `operations!` writes on `&self` put inside the tree they build.
    pub(crate) fn operand(&self) -> Expr<S, T> {
        self.expr()
    }
}

impl<S, T, R: Writable> Sig<S, T, R> {
    /// A statement that assigns `value` to this signal in the browser.
    #[must_use]
    pub fn set(self, value: impl IntoExpr<S, T>) -> Stmt<S> {
        Stmt::set(self.path.place_json(), value.into_expr())
    }
}

impl<S, T> Sig<S, T, Root> {
    /// The dotted field path this signal is stored under.
    ///
    /// Inside [`Part::render`](crate::Part::render), the path starts with the
    /// page field the part is embedded at.
    #[must_use]
    pub fn path(self) -> String {
        self.path.rooted()
    }

    /// The dotted field path relative to `S`, which a patch writes under the
    /// field of the part answering when it is sent.
    pub(crate) fn dotted(self) -> String {
        self.path.dotted()
    }

    /// A patch that sets this signal to `value` when the server sends it.
    ///
    /// # Panics
    ///
    /// Panics if `value` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error, or if it holds an integer outside -2^53 to 2^53,
    /// which the browser would round.
    #[must_use]
    pub fn patch(self, value: T) -> SignalPatch<S>
    where
        T: Serialize,
    {
        SignalPatch::single(self.dotted(), &value)
    }

    /// The computed field `field` of the struct this signal holds.
    ///
    /// [`Sig::at`] takes only the fields the browser may write, so a field
    /// marked `#[signals(computed)]` in a nested struct is reached with this
    /// method, and only from the page root, where
    /// [`Page::computed`](crate::Page::computed) can name it.
    #[must_use]
    pub fn at_computed<U>(self, field: Sig<T, U, Computed>) -> Sig<S, U, Computed> {
        Sig::from_path(self.path.join(field.path))
    }
}

impl<S, T> Sig<S, T, Computed> {
    /// The dotted field path this signal is stored under, as [`Sig::path`] gives it.
    #[must_use]
    pub fn path(self) -> String {
        self.path.rooted()
    }
}

impl<S, R: Writable> Sig<S, bool, R> {
    /// A statement that flips this flag.
    #[must_use]
    pub fn toggle(self) -> Stmt<S> {
        self.set(!self)
    }
}

impl<S, T, R> Sig<S, Vec<T>, R> {
    /// The number of items for which `matches` is true.
    #[must_use]
    pub fn count<P: IntoExpr<S, bool>>(
        self,
        matches: impl FnOnce(Item<S, T>) -> P,
    ) -> Expr<S, i32> {
        let (depth, predicate) = self.predicate(matches);
        Expr::from_json(json!(["count", self.path.place_json(), depth, predicate]))
    }

    /// The list of what `value` gives for each item, in order.
    ///
    /// ```
    /// use hypersig::{Expr, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Task {
    ///     id: i32,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Board {
    ///     tasks: Vec<Task>,
    ///     chosen: Vec<i32>,
    /// }
    ///
    /// let _choose_all = Board::chosen().set(Board::tasks().map(|task| task.at(Task::id())));
    /// ```
    #[must_use]
    pub fn map<U, E: IntoExpr<S, U>>(self, value: impl FnOnce(Item<S, T>) -> E) -> Expr<S, Vec<U>> {
        self.aggregate("map", value)
    }

    /// The sum of `value` over the items, 0 for an empty list.
    #[must_use]
    pub fn sum_of<U: Number, E: IntoExpr<S, U>>(
        self,
        value: impl FnOnce(Item<S, T>) -> E,
    ) -> Expr<S, U> {
        self.aggregate("sum_of", value)
    }

    /// The largest `value` over the items, or `None` for an empty list.
    #[must_use]
    pub fn max_of<U: Number, E: IntoExpr<S, U>>(
        self,
        value: impl FnOnce(Item<S, T>) -> E,
    ) -> Expr<S, Option<U>> {
        self.aggregate("max_of", value)
    }

    /// The smallest `value` over the items, or `None` for an empty list.
    #[must_use]
    pub fn min_of<U: Number, E: IntoExpr<S, U>>(
        self,
        value: impl FnOnce(Item<S, T>) -> E,
    ) -> Expr<S, Option<U>> {
        self.aggregate("min_of", value)
    }

    fn aggregate<U, E: IntoExpr<S, U>, V>(
        self,
        op: &str,
        value: impl FnOnce(Item<S, T>) -> E,
    ) -> Expr<S, V> {
        let (depth, value) = self.scoped(value);
        Expr::from_json(json!([op, self.path.place_json(), depth, value]))
    }

    fn predicate<P: IntoExpr<S, bool>>(self, build: impl FnOnce(Item<S, T>) -> P) -> (u8, Value) {
        self.scoped(build)
    }

    /// The depth of a new item scope over this list, and the expression
    /// `build` makes from its item.
    fn scoped<U, E: IntoExpr<S, U>>(self, build: impl FnOnce(Item<S, T>) -> E) -> (u8, Value) {
        in_item_scope(|depth| {
            let item = Item::new(self.path, depth);
            (depth, build(item).into_expr().into_json())
        })
    }
}

impl<S, T, R: Writable> Sig<S, Vec<T>, R> {
    /// A statement that appends `value` to this list.
    #[must_use]
    pub fn push(self, value: impl IntoExpr<S, T>) -> Stmt<S> {
        Stmt::list(
            "push",
            self.path.place_json(),
            value.into_expr().into_json(),
        )
    }

    /// A statement that appends every item of `items` to this list.
    #[must_use]
    pub fn append(self, items: impl IntoExpr<S, Vec<T>>) -> Stmt<S> {
        Stmt::list(
            "append",
            self.path.place_json(),
            items.into_expr().into_json(),
        )
    }

    /// A statement that adds every item of `items` before the items of this list.
    ///
    /// Where the list is drawn with [`each_keyed`](crate::html::each_keyed),
    /// the runtime keeps the items that were already on screen where they
    /// were, scrolling the nearest scrollable ancestor by the height of the
    /// items added above them. A list drawn with [`each`](crate::html::each)
    /// matches its elements by position, so its first element shows the first
    /// new item and nothing is scrolled.
    #[must_use]
    pub fn prepend(self, items: impl IntoExpr<S, Vec<T>>) -> Stmt<S> {
        Stmt::list(
            "prepend",
            self.path.place_json(),
            items.into_expr().into_json(),
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
        self.sort(key, false)
    }

    /// A statement that orders the items by `key`, descending, keeping the
    /// order of items with equal keys.
    #[must_use]
    pub fn sort_by_desc<K: Comparable, E: IntoExpr<S, K>>(
        self,
        key: impl FnOnce(Item<S, T>) -> E,
    ) -> Stmt<S> {
        self.sort(key, true)
    }

    fn sort<K: Comparable, E: IntoExpr<S, K>>(
        self,
        key: impl FnOnce(Item<S, T>) -> E,
        descending: bool,
    ) -> Stmt<S> {
        let (depth, key) = in_item_scope(|depth| {
            let item = Item::new(self.path, depth);
            (depth, key(item).into_expr().into_json())
        });
        Stmt::new(json!([
            "sort",
            self.path.place_json(),
            depth,
            key,
            descending
        ]))
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
