//! The pages on talking to the server: calling actions, what a call does
//! while it waits and when it fails, and the patches the server answers with.

pub(super) mod actions;
pub(super) mod call_status;
pub(super) mod fragments;
pub(super) mod list_patches;
pub(super) mod optimistic;
pub(super) mod overlapping_calls;
pub(super) mod patches;
pub(super) mod streams;

use crate::Site;
use crate::lang::L;
use crate::prose::{Part, note};

/// The note under a program that needs a server to run.
fn needs_a_server(site: Site) -> Part {
    note(
        site,
        L {
            en: "This program needs a server, so this site shows it as code. It is built by `cargo test` from `site/examples/` in the repository.",
            ja: "このプログラムにはサーバーが必要なので、このサイトではコードとして示します。リポジトリの `site/examples/` にあり、`cargo test` でビルドしています。",
        },
    )
}
