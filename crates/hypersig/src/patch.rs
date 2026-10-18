//! Patches the server sends back to the page, and the fragments they replace.
//!
//! On the wire each patch is one Server-Sent Event with a single-line JSON
//! payload. The runtime in `runtime/src/runtime.ts` reads five event names:
//!
//! | Event | Data |
//! |---|---|
//! | `hs-signals` | `{"path": value, ...}`, merged into the signal store |
//! | `hs-list` | a list statement, `["append" \| "prepend", "path", ["lit", items]]`, `["upsert", "path", depth, key, ["lit", items]]`, or `["remove_keys", "path", depth, key, ["lit", keys]]`, run against the signal store |
//! | `hs-element` | `{"html": "...", "css": [[sort key, css], ...]}`, morphed into the element with the root's `id` |
//! | `hs-navigate` | `{"url": "..."}`, the page the browser loads next |
//! | `hs-history` | `{"url": "...", "replace": bool}`, the URL the address bar shows, without loading it |
//!
//! A patch answering a call made inside a [`Part`](crate::Part) is written
//! under the page field the part sits at: each key of `hs-signals` starts
//! with that field, and the fragment of `hs-element` was rendered with its
//! paths under it.
//!
//! `css` is present when the fragment uses [`El::css`] or [`El::css_when`].
//! It lists the rule buckets of [`Sheet::buckets`]. The runtime adds each to
//! the `<style data-hs-css>` with the same sort key, or inserts a new one
//! before the first whose key sorts after it, so the rules keep the order
//! the page would have written them in.

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::expr::IntoExpr;
use crate::html::El;
use crate::route::Url;
use crate::signal::{Item, Root, Sig, Signals, current_part, in_item_scope};
use crate::style::Sheet;

/// A part of the page that the server can re-render and replace without the rest of the page.
///
/// The fragment owns its element id, so the id in the initial page and the id
/// in a later patch cannot drift apart.
///
/// ```
/// use hypersig::html::{El, li, ul};
/// use hypersig::{Fragment, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Shop {}
///
/// struct Cart {
///     items: Vec<String>,
/// }
///
/// impl Fragment for Cart {
///     type Signals = Shop;
///     const ID: &'static str = "cart";
///
///     fn render(&self) -> El<Shop> {
///         ul().children(self.items.iter().map(|item| li().text(item.as_str())))
///     }
/// }
///
/// let html = hypersig::fragment(&Cart { items: vec!["tea".into()] }).render();
/// assert_eq!(html, r#"<ul id="cart"><li>tea</li></ul>"#);
/// ```
pub trait Fragment {
    /// The signal struct of the page this fragment lives on.
    type Signals: Signals;

    /// The element id of the fragment's root.
    const ID: &'static str;

    /// Renders the fragment's root element. Its own `id` is replaced by [`Self::ID`].
    fn render(&self) -> El<Self::Signals>;
}

/// Renders `fragment` with its root id set to [`Fragment::ID`].
#[must_use]
pub fn fragment<F: Fragment>(fragment: &F) -> El<F::Signals> {
    fragment.render().with_id(F::ID)
}

/// New values for one or more signals of `S`.
///
/// Build one with [`Sig::patch`](crate::Sig::patch) and combine with [`SignalPatch::and`].
pub struct SignalPatch<S> {
    values: Map<String, Value>,
    marker: PhantomData<fn() -> S>,
}

impl<S> SignalPatch<S> {
    pub(crate) fn single(path: String, value: &impl Serialize) -> Self {
        let value = serde_json::to_value(value).expect("a signal value serializes to JSON");
        let mut values = Map::new();
        values.insert(path, value);
        Self {
            values,
            marker: PhantomData,
        }
    }

    /// Adds the values in `other`. A signal present in both takes `other`'s value.
    #[must_use]
    pub fn and(mut self, other: Self) -> Self {
        self.values.extend(other.values);
        self
    }
}

impl<S> fmt::Debug for SignalPatch<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SignalPatch").field(&self.values).finish()
    }
}

/// Item changes to list signals of `S`, sent without the rest of each list.
///
/// Build one with [`Sig::patch_append`], [`Sig::patch_prepend`],
/// [`Sig::patch_upsert`], or [`Sig::patch_remove`], and combine with
/// [`ListPatch::and`]. The browser applies the changes in order to the list
/// it holds, so a list the server sends one message of at a time grows in
/// the browser without the server keeping a copy of it.
///
/// ```
/// use hypersig::{Patches, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Message {
///     body: String,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Room {
///     messages: Vec<Message>,
/// }
///
/// let hello = Message { body: "hello".into() };
/// let patches: Patches<Room> = Room::messages().patch_append(&[hello]).into();
/// assert_eq!(
///     patches.to_event_stream(),
///     "event: hs-list\ndata: [\"append\",\"messages\",[\"lit\",[{\"body\":\"hello\"}]]]\n\n",
/// );
/// ```
pub struct ListPatch<S> {
    statements: Vec<Value>,
    marker: PhantomData<fn() -> S>,
}

impl<S> ListPatch<S> {
    fn single(statement: Value) -> Self {
        Self {
            statements: vec![statement],
            marker: PhantomData,
        }
    }

    /// Adds the changes in `other`, applied after the ones in `self`.
    #[must_use]
    pub fn and(mut self, other: Self) -> Self {
        self.statements.extend(other.statements);
        self
    }
}

impl<S> fmt::Debug for ListPatch<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ListPatch").field(&self.statements).finish()
    }
}

/// The list patches exist only on a signal reached from the page root, as
/// [`Sig::patch`] does, because the server cannot name the item of an
/// enclosing list that the browser is rendering.
impl<S, T: Serialize> Sig<S, Vec<T>, Root> {
    /// A patch that adds `items` after the items the browser holds.
    ///
    /// # Panics
    ///
    /// Panics if an item fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn patch_append(self, items: &[T]) -> ListPatch<S> {
        ListPatch::single(json!(["append", self.dotted(), literal(items)]))
    }

    /// A patch that adds `items` before the items the browser holds.
    ///
    /// Where the list is drawn with [`each_keyed`](crate::html::each_keyed),
    /// the runtime keeps the items that were already on screen where they
    /// were, scrolling the nearest scrollable ancestor by the height of the
    /// items added above them, as [`Sig::prepend`] does.
    ///
    /// # Panics
    ///
    /// Panics if an item fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn patch_prepend(self, items: &[T]) -> ListPatch<S> {
        ListPatch::single(json!(["prepend", self.dotted(), literal(items)]))
    }

    /// A patch that replaces items by key and appends those with new keys.
    ///
    /// The browser computes `key` for the items it holds and for each of
    /// `items`, as [`each_keyed`](crate::html::each_keyed) computes the key of
    /// a row, and compares the keys as JSON. An item of `items` replaces the
    /// first held item with its key, and an item whose key no held item has
    /// goes at the end. Of two items in `items` with one key, the later wins.
    ///
    /// ```
    /// use hypersig::{Patches, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Server {
    ///     name: String,
    ///     load: f64,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Dashboard {
    ///     servers: Vec<Server>,
    /// }
    ///
    /// let busy = Server { name: "db-1".into(), load: 0.9 };
    /// let _: Patches<Dashboard> = Dashboard::servers()
    ///     .patch_upsert(&[busy], |server| server.at(Server::name()))
    ///     .into();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if an item fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn patch_upsert<K, E: IntoExpr<S, K>>(
        self,
        items: &[T],
        key: impl FnOnce(Item<S, T>) -> E,
    ) -> ListPatch<S> {
        let (depth, key) = self.key(key);
        ListPatch::single(json!(["upsert", self.dotted(), depth, key, literal(items)]))
    }

    /// A patch that removes each item whose key is one of `keys`.
    ///
    /// The browser computes `key` for the items it holds, as
    /// [`patch_upsert`](Sig::patch_upsert) does, and compares it with each of
    /// `keys` as JSON.
    ///
    /// # Panics
    ///
    /// Panics if a key fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn patch_remove<K: Serialize, E: IntoExpr<S, K>>(
        self,
        keys: &[K],
        key: impl FnOnce(Item<S, T>) -> E,
    ) -> ListPatch<S> {
        let (depth, key) = self.key(key);
        ListPatch::single(json!([
            "remove_keys",
            self.dotted(),
            depth,
            key,
            literal(keys)
        ]))
    }

    /// The depth of the item scope `key` is built in, and the key's expression.
    fn key<K, E: IntoExpr<S, K>>(self, key: impl FnOnce(Item<S, T>) -> E) -> (u8, Value) {
        in_item_scope(|depth| {
            let item = Item::new(self.raw_path(), depth);
            (depth, key(item).into_expr().into_json())
        })
    }
}

fn literal<T: Serialize>(values: &[T]) -> Value {
    json!([
        "lit",
        serde_json::to_value(values).expect("a list item serializes to JSON")
    ])
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PatchEvent {
    Signals(Map<String, Value>),
    List(Value),
    Element {
        html: String,
        css: Vec<(String, String)>,
        /// The part the fragment's paths were written under.
        part: Option<Arc<str>>,
    },
    Navigate(Url),
    History {
        url: Url,
        replace: bool,
    },
}

impl PatchEvent {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Signals(_) => "hs-signals",
            Self::List(_) => "hs-list",
            Self::Element { .. } => "hs-element",
            Self::Navigate(_) => "hs-navigate",
            Self::History { .. } => "hs-history",
        }
    }

    /// The event's payload, written for the part the current call came from.
    ///
    /// # Panics
    ///
    /// Panics if the event is a fragment rendered under another part than
    /// the current one, such as one rendered in a task the call's handler
    /// did not run, because its paths would land on the wrong signals.
    pub(crate) fn data(&self) -> String {
        match self {
            Self::Signals(values) => match current_part() {
                None => Value::Object(values.clone()).to_string(),
                Some(part) => Value::Object(
                    values
                        .iter()
                        .map(|(path, value)| (format!("{part}.{path}"), value.clone()))
                        .collect(),
                )
                .to_string(),
            },
            Self::List(statement) => match current_part() {
                None => statement.to_string(),
                Some(part) => {
                    let mut statement = statement.clone();
                    if let Some(place) = statement.get_mut(1)
                        && let Value::String(path) = place
                    {
                        *path = format!("{part}.{path}");
                    }
                    statement.to_string()
                }
            },
            Self::Element { html, css, part } => {
                assert!(
                    *part == current_part(),
                    "a fragment rendered under the part at {part:?} answers a call from the part \
                     at {:?}; render a part's fragment while its handler or stream runs",
                    current_part(),
                );
                if css.is_empty() {
                    json!({ "html": html }).to_string()
                } else {
                    json!({ "html": html, "css": css }).to_string()
                }
            }
            Self::Navigate(url) => json!({ "url": url.as_str() }).to_string(),
            Self::History { url, replace } => {
                json!({ "url": url.as_str(), "replace": replace }).to_string()
            }
        }
    }
}

/// A batch of patches for a page whose signals are `S`, applied in order.
pub struct Patches<S> {
    events: Vec<PatchEvent>,
    marker: PhantomData<fn() -> S>,
}

impl<S> Default for Patches<S> {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            marker: PhantomData,
        }
    }
}

impl<S> fmt::Debug for Patches<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Patches").field(&self.events).finish()
    }
}

impl<S: Signals> Patches<S> {
    /// An empty batch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets signals in the browser.
    ///
    /// For a [`Part`](crate::Part), the values land under the page field the
    /// calling part sits at, wherever the patch was built.
    #[must_use]
    pub fn signals(mut self, patch: SignalPatch<S>) -> Self {
        self.events.push(PatchEvent::Signals(patch.values));
        self
    }

    /// Changes items of list signals in the browser, sending only the items that change.
    #[must_use]
    pub fn list(mut self, patch: ListPatch<S>) -> Self {
        self.events
            .extend(patch.statements.into_iter().map(PatchEvent::List));
        self
    }

    /// Replaces every signal with the values in `signals`.
    ///
    /// # Panics
    ///
    /// Panics if `signals` fails to serialize to a JSON object, which only
    /// happens for a `Serialize` impl that returns an error.
    #[must_use]
    pub fn all_signals(mut self, signals: &S) -> Self {
        let Value::Object(values) =
            serde_json::to_value(signals).expect("a Signals struct serializes to JSON")
        else {
            panic!("a Signals struct serializes to a JSON object");
        };
        self.events.push(PatchEvent::Signals(values));
        self
    }

    /// Re-renders `fragment` and morphs it into the element with its id.
    ///
    /// The fragment is rendered without the signal values, so the text and
    /// attributes its bindings show are written by the runtime once the patch
    /// arrives. A list or condition inside it whose template is unchanged keeps
    /// the items and content the runtime drew, with their focus and any text
    /// being typed in them.
    ///
    /// The server checks the fragment's names and ids on their own, and the
    /// browser checks them against the rest of the page once the patch is
    /// applied, writing a warning to the console for each one another element
    /// already carries.
    ///
    /// # Panics
    ///
    /// For a [`Part`](crate::Part), the fragment is rendered with its paths
    /// under the field of the part being answered, so the batch panics when
    /// it is sent if it was built outside the handler or stream answering
    /// that part, such as in a task that broadcasts one batch to every page.
    #[must_use]
    pub fn element<F: Fragment<Signals = S>>(mut self, fragment: &F) -> Self {
        let el = self::fragment(fragment);
        let mut sheet = Sheet::new();
        el.collect_css(&mut sheet);
        let css = sheet
            .buckets()
            .map(|bucket| (bucket.sort_key().to_owned(), bucket.css().to_owned()))
            .collect();
        self.events.push(PatchEvent::Element {
            html: el.render(),
            css,
            part: current_part(),
        });
        self
    }

    /// Loads the page at `url` once the patches before it are applied.
    ///
    /// The browser leaves the current page, so patches after this one in the
    /// same batch are not applied.
    #[must_use]
    pub fn navigate(mut self, url: Url) -> Self {
        self.events.push(PatchEvent::Navigate(url));
        self
    }

    /// Shows `url` in the address bar as a new history entry, without loading it.
    ///
    /// The patches after it in the batch are applied, so an action that
    /// answers a change of filter can send the rows it found and the URL
    /// that names them together. The visitor's Back and Forward buttons
    /// move between the entries, and the browser then loads the page at the
    /// entry's URL from the server. A URL equal to the current one adds no
    /// entry.
    ///
    /// The type system does not check that `url` is an address of the page
    /// the action patches. The URL of another page shows over this page's
    /// content, and reloading it loads the other page.
    ///
    /// ```
    /// use hypersig::{Patches, Route, Search, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Search, Serialize, Deserialize)]
    /// struct Filter {
    ///     q: String,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Fruit {
    ///     matches: Vec<String>,
    /// }
    ///
    /// # async fn fruit((): (), _: Filter) -> Option<hypersig::Page<Fruit>> { None }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const FRUIT: Route<(), Filter> = "/fruit" => fruit;
    /// }
    ///
    /// let q = "app".to_owned();
    /// let patches = Patches::from(Fruit::matches().patch(vec!["apple".to_owned()]))
    ///     .push_url(FRUIT.url(Filter { q }));
    /// assert!(patches.to_event_stream().ends_with(
    ///     "event: hs-history\ndata: {\"replace\":false,\"url\":\"/fruit?q=app\"}\n\n"
    /// ));
    /// # let _ = pages;
    /// ```
    #[must_use]
    pub fn push_url(mut self, url: Url) -> Self {
        self.events.push(PatchEvent::History {
            url,
            replace: false,
        });
        self
    }

    /// Shows `url` in the address bar in place of the current URL, without loading it.
    ///
    /// It is [`Patches::push_url`] without the new history entry.
    #[must_use]
    pub fn replace_url(mut self, url: Url) -> Self {
        self.events.push(PatchEvent::History { url, replace: true });
        self
    }

    /// The batch as a Server-Sent Events body, for use without the axum feature.
    #[must_use]
    pub fn to_event_stream(&self) -> String {
        let mut out = String::new();
        for event in &self.events {
            out.push_str("event: ");
            out.push_str(event.name());
            out.push_str("\ndata: ");
            out.push_str(&event.data());
            out.push_str("\n\n");
        }
        out
    }

    #[cfg(feature = "axum")]
    pub(crate) fn into_events(self) -> Vec<PatchEvent> {
        self.events
    }
}

impl<S: Signals> From<SignalPatch<S>> for Patches<S> {
    fn from(patch: SignalPatch<S>) -> Self {
        Self::new().signals(patch)
    }
}

impl<S: Signals> From<ListPatch<S>> for Patches<S> {
    fn from(patch: ListPatch<S>) -> Self {
        Self::new().list(patch)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::html::span;
    use crate::signal::{Path, in_part};

    #[derive(Serialize, Deserialize)]
    struct Badge {
        count: i32,
    }

    impl Signals for Badge {}

    fn count() -> Sig<Badge, i32> {
        Sig::from_path(Path::field("count"))
    }

    struct Count;

    impl Fragment for Count {
        type Signals = Badge;
        const ID: &'static str = "count";

        fn render(&self) -> El<Badge> {
            span().bind_text(count().to_text())
        }
    }

    #[test]
    fn signals_land_under_the_part_wherever_they_were_built() {
        let patches = Patches::from(count().patch(3));
        let stream = in_part(Some("header.badge".into()), || patches.to_event_stream());
        assert_eq!(
            stream,
            "event: hs-signals\ndata: {\"header.badge.count\":3}\n\n"
        );
    }

    #[derive(Serialize, Deserialize)]
    struct Feed {
        items: Vec<i32>,
    }

    impl Signals for Feed {}

    #[test]
    fn a_list_patch_lands_under_the_part_wherever_it_was_built() {
        let items: Sig<Feed, Vec<i32>> = Sig::from_path(Path::field("items"));
        let patches = Patches::from(items.patch_append(&[4]));
        let stream = in_part(Some("footer".into()), || patches.to_event_stream());
        assert_eq!(
            stream,
            "event: hs-list\ndata: [\"append\",\"footer.items\",[\"lit\",[4]]]\n\n"
        );
    }

    #[test]
    #[should_panic(expected = "render a part's fragment while its handler or stream runs")]
    fn a_fragment_rendered_outside_its_part_panics_when_written() {
        let patches = Patches::new().element(&Count);
        in_part(Some("header.badge".into()), || patches.to_event_stream());
    }
}
