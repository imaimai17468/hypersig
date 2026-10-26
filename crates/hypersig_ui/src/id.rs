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

/// The CSS anchor name `--<id>` for the element `id` names, with each `::`
/// as `--`.
///
/// A dashed ident holds no unescaped `:`. The id comes from a node ref's
/// name, a Rust path whose identifiers hold no `-`, so two names stay apart
/// after the replacement.
pub(crate) fn anchor(id: &Id) -> String {
    format!("--{}", id.as_str().replace("::", "--"))
}
