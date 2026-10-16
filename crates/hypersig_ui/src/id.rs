//! Element ids and names derived from a signal or a handle.

use hypersig::Sig;

/// `ui-<path>-<part>`, where `<path>` is the signal's path with each `.`
/// replaced by `-`. A page signal's path is unique on its page, so ids built
/// from different signals differ.
pub(crate) fn of<S, T>(signal: Sig<S, T>, part: &str) -> String {
    format!("ui-{}-{part}", signal.path().replace('.', "-"))
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
