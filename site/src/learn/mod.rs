//! The Learn section, a guide read from top to bottom.

mod basics;
mod server;
pub mod snippet;
mod state;

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::Learn;
use crate::prose::Part;

/// The article of `page`.
pub fn content(site: Site, page: Learn) -> Part {
    match page {
        Learn::QuickStart => basics::quick_start(site),
        Learn::DescribingThePage => basics::describing_the_page(site),
        Learn::Signals => state::signals(site),
        Learn::Expressions => state::expressions(site),
        Learn::Lists => state::lists(site),
        Learn::Server => server::server(site),
        Learn::Styling => server::styling(site),
        Learn::EscapeHatches => server::escape_hatches(site),
    }
}

/// A complete program under `site/examples/`, which `cargo test` builds.
#[derive(Debug, Clone, Copy)]
enum Program {
    QuickStart,
    Actions,
    Fragments,
    Streams,
}

/// `program` as a code block titled `src/main.rs`.
fn program(program: Program) -> Part {
    let source = match program {
        Program::QuickStart => include_str!("../../examples/quick_start.rs"),
        Program::Actions => include_str!("../../examples/actions.rs"),
        Program::Fragments => include_str!("../../examples/fragments.rs"),
        Program::Streams => include_str!("../../examples/streams.rs"),
    };
    code::block(
        Lang::Rust,
        Some("src/main.rs"),
        code::without_header(source),
    )
}
