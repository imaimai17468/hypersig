//! The Learn section, a guide read from top to bottom.

mod api;
mod basics;
mod database;
mod deploying;
mod forms;
mod javascript;
mod requests;
mod security;
mod server;
mod sessions;
pub mod snippet;
mod state;
mod testing;
mod why;

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::Learn;
use crate::prose::Part;

/// The article of `page`.
pub fn content(site: Site, page: Learn) -> Part {
    match page {
        Learn::Why => why::why(site),
        Learn::QuickStart => basics::quick_start(site),
        Learn::DescribingThePage => basics::describing_the_page(site),
        Learn::Signals => state::signals(site),
        Learn::Expressions => state::expressions(site),
        Learn::Lists => state::lists(site),
        Learn::Server => server::server(site),
        Learn::CallsInDepth => server::calls_in_depth(site),
        Learn::Forms => forms::forms(site),
        Learn::Requests => requests::requests(site),
        Learn::Sessions => sessions::sessions(site),
        Learn::Database => database::database(site),
        Learn::Api => api::api(site),
        Learn::Pages => server::pages(site),
        Learn::RoutesInDepth => server::routes_in_depth(site),
        Learn::Styling => server::styling(site),
        Learn::EscapeHatches => server::escape_hatches(site),
        Learn::JavaScript => javascript::javascript(site),
        Learn::Deploying => deploying::deploying(site),
        Learn::DeployContainer => deploying::container::page(site),
        Learn::DeployCloudflare => deploying::cloudflare::page(site),
        Learn::DeployVercel => deploying::vercel::page(site),
        Learn::Security => security::security(site),
        Learn::Testing => testing::testing(site),
    }
}

/// A complete program under `site/examples/`, which `cargo test` builds.
#[derive(Debug, Clone, Copy)]
enum Program {
    QuickStart,
    Actions,
    Fragments,
    Streams,
    Pages,
    Testing,
}

/// `program` as a code block titled `src/main.rs`.
fn program(program: Program) -> Part {
    let source = match program {
        Program::QuickStart => include_str!("../../examples/quick_start.rs"),
        Program::Actions => include_str!("../../examples/actions.rs"),
        Program::Fragments => include_str!("../../examples/fragments.rs"),
        Program::Streams => include_str!("../../examples/streams.rs"),
        Program::Pages => include_str!("../../examples/pages.rs"),
        Program::Testing => include_str!("../../examples/testing.rs"),
    };
    code::block(
        Lang::Rust,
        Some("src/main.rs"),
        code::without_header(source),
    )
}
