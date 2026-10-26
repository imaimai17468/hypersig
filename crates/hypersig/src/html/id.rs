//! Element ids, which one element carries and others point at.

use std::borrow::Cow;
use std::fmt::{self, Write as _};

/// The id of an element, which [`El::id`](super::El::id) writes and the
/// methods that point at an element read, such as
/// [`El::label_for`](super::El::label_for) and
/// [`El::aria_describedby`](super::El::aria_describedby).
///
/// Declare an id once, as a `const` or a value both ends share, so that the
/// element and each reference to it hold one value rather than two strings
/// that can be spelled apart:
///
/// ```
/// use hypersig::html::{Id, div, input, label};
///
/// const EMAIL: Id = Id::new("email");
///
/// let html = div::<()>()
///     .child(label().label_for(&EMAIL).text("Email"))
///     .child(input().id(EMAIL))
///     .render();
/// assert_eq!(html, r#"<div><label for="email">Email</label><input id="email"></div>"#);
/// ```
///
/// Nothing checks that the page holds the element a reference names. A
/// reference to an id no element carries is ignored by the browser, so the
/// control reads with no label or description.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id(pub(crate) Cow<'static, str>);

impl Id {
    /// An id written in the code.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty or holds ASCII whitespace, which separates
    /// the ids of an `aria-*` list. In a `const`, the panic is a compile
    /// error.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        let bytes = name.as_bytes();
        assert!(!bytes.is_empty(), "an id is never empty");
        let mut i = 0;
        while i < bytes.len() {
            assert!(!bytes[i].is_ascii_whitespace(), "an id holds no whitespace");
            i += 1;
        }
        Self(Cow::Borrowed(name))
    }

    /// This id followed by `-` and `key`, for one of several elements, such
    /// as a row of a list.
    ///
    /// An id holds no whitespace, so each ASCII whitespace character of
    /// `key` is written as `%` and its two hex digits, as in a URL, and `%`
    /// itself as `%25`. Two different keys therefore give two different ids.
    ///
    /// ```
    /// use hypersig::html::Id;
    ///
    /// const ROW: Id = Id::new("row");
    /// assert_eq!(ROW.with_key(7).as_str(), "row-7");
    /// assert_eq!(ROW.with_key("a b").as_str(), "row-a%20b");
    /// assert_eq!(ROW.with_key("a_b").as_str(), "row-a_b");
    /// ```
    #[must_use]
    pub fn with_key(&self, key: impl fmt::Display) -> Self {
        let mut id = format!("{}-", self.0);
        let start = id.len();
        let _ = write!(id, "{key}");
        let key = id.split_off(start);
        for c in key.chars() {
            if c == '%' || c.is_ascii_whitespace() {
                let _ = write!(id, "%{:02X}", u32::from(c));
            } else {
                id.push(c);
            }
        }
        Self(Cow::Owned(id))
    }

    /// An id built at run time from `text`, such as the anchor of a heading
    /// made from its words.
    ///
    /// ASCII whitespace in `text` becomes `_`, and an empty `text` gives
    /// `_`, because an id holds neither. Prefer [`Id::new`] for an id written
    /// in the code, since only that id is checked for a second element
    /// carrying it.
    ///
    /// ```
    /// use hypersig::html::Id;
    ///
    /// assert_eq!(Id::from_text("list-patches").as_str(), "list-patches");
    /// assert_eq!(Id::from_text("").as_str(), "_");
    /// ```
    #[must_use]
    pub fn from_text(text: impl fmt::Display) -> Self {
        let mut id = String::new();
        push_text(&mut id, text);
        if id.is_empty() {
            id.push('_');
        }
        Self(Cow::Owned(id))
    }

    /// The id as the HTML holds it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Appends `text` to `id` with each ASCII whitespace character as `_`.
fn push_text(id: &mut String, text: impl fmt::Display) {
    let start = id.len();
    let _ = write!(id, "{text}");
    let text = id.split_off(start);
    id.extend(
        text.chars()
            .map(|c| if c.is_ascii_whitespace() { '_' } else { c }),
    );
}

/// The ids of `ids` separated by spaces, as an `aria-*` list holds them.
pub(crate) fn list(ids: &[&Id]) -> String {
    let mut out = String::new();
    for id in ids {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(id.as_str());
    }
    out
}
