//! Element ids and names derived from a signal or a handle.

use std::fmt::Display;

use hypersig::Sig;
use hypersig::html::Id;

/// `ui-<path>`, where `<path>` is the signal's path with each `.` replaced
/// by `-`. A page signal's path is unique on its page, so ids built from
/// different signals differ.
pub(crate) fn base<S, T>(signal: Sig<S, T>) -> Id {
    const UI: Id = Id::new("ui");
    UI.with_key(signal.path().replace('.', "-"))
}

/// `ui-<path>-<part>`, as [`base`] builds `ui-<path>`.
pub(crate) fn of<S, T>(signal: Sig<S, T>, part: impl Display) -> Id {
    base(signal).with_key(part)
}

/// Panics unless `name` is non-empty ASCII letters, digits, `-`, and `_`,
/// which an id, an anchor name, and a node ref can hold unescaped. In a
/// `const`, the panic is a compile error.
pub(crate) const fn check_name(name: &str) {
    let bytes = name.as_bytes();
    assert!(!bytes.is_empty(), "a component name is never empty");
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        assert!(
            b.is_ascii_alphanumeric() || b == b'-' || b == b'_',
            "a component name is ASCII letters, digits, `-`, and `_`"
        );
        i += 1;
    }
}

/// Whether `a` and `b` hold the same bytes, for a `const` check.
pub(crate) const fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}
