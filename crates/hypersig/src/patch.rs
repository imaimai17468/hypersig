//! Patches the server sends back to the page, and the fragments they replace.
//!
//! On the wire each patch is one Server-Sent Event with a single-line JSON
//! payload. The runtime in `runtime/src/sse.ts` reads two event names:
//!
//! | Event | Data |
//! |---|---|
//! | `hs-signals` | `{"path": value, ...}`, merged into the signal store |
//! | `hs-element` | `{"html": "..."}`, morphed into the element with the root's `id` |

use std::fmt;
use std::marker::PhantomData;

use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::html::El;
use crate::signal::Signals;

/// A part of the page the server can re-render and replace on its own.
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
    pub(crate) fn single(path: &'static str, value: &impl Serialize) -> Self {
        let value = serde_json::to_value(value).expect("a signal value serializes to JSON");
        let mut values = Map::new();
        values.insert(path.to_owned(), value);
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PatchEvent {
    Signals(Map<String, Value>),
    Element(String),
}

impl PatchEvent {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Signals(_) => "hs-signals",
            Self::Element(_) => "hs-element",
        }
    }

    pub(crate) fn data(&self) -> String {
        match self {
            Self::Signals(values) => Value::Object(values.clone()).to_string(),
            Self::Element(html) => json!({ "html": html }).to_string(),
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
    #[must_use]
    pub fn signals(mut self, patch: SignalPatch<S>) -> Self {
        self.events.push(PatchEvent::Signals(patch.values));
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
    #[must_use]
    pub fn element<F: Fragment<Signals = S>>(mut self, fragment: &F) -> Self {
        self.events
            .push(PatchEvent::Element(self::fragment(fragment).render()));
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
