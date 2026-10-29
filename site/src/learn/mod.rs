//! The Learn section, a guide read from top to bottom.

mod data;
mod deploying;
mod javascript;
mod page;
mod requests;
mod routing;
mod security;
mod server;
pub mod snippet;
mod start;
mod state;
mod talking;
mod testing;
mod troubleshooting;
mod upgrading;

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::Learn;
use crate::prose::Part;

/// The article of `page`.
pub fn content(site: Site, page: Learn) -> Part {
    match page {
        // Get started
        Learn::QuickStart => start::quick_start::page(site),
        Learn::Why => start::why::page(site),
        Learn::DevServer => start::dev_server::page(site),
        // Building the page
        Learn::Elements => page::elements::page(site),
        Learn::ThePage => page::document::page(site),
        Learn::Bindings => page::bindings::page(site),
        Learn::Components => page::components::page(site),
        Learn::Styling => page::styling::page(site),
        // Browser state
        Learn::Signals => state::signals::page(site),
        Learn::NestedSignals => state::nested::page(site),
        Learn::Expressions => state::expressions::page(site),
        Learn::Statements => state::statements::page(site),
        Learn::Events => state::events::page(site),
        Learn::BindingInputs => state::inputs::page(site),
        Learn::ComputedSignals => state::computed::page(site),
        Learn::PersistedSignals => state::persisted::page(site),
        Learn::Lists => state::lists::page(site),
        Learn::ListItems => state::list_items::page(site),
        // Talking to the server
        Learn::Actions => talking::actions::page(site),
        Learn::CallStatus => talking::call_status::page(site),
        Learn::Optimistic => talking::optimistic::page(site),
        Learn::OverlappingCalls => talking::overlapping_calls::page(site),
        Learn::Patches => talking::patches::page(site),
        Learn::ListPatches => talking::list_patches::page(site),
        Learn::Fragments => talking::fragments::page(site),
        Learn::Streams => talking::streams::page(site),
        // Pages and routing
        Learn::Routes => routing::routes::page(site),
        Learn::Links => routing::links::page(site),
        Learn::ChildPages => routing::child_pages::page(site),
        Learn::SearchParams => routing::search_params::page(site),
        Learn::ErrorPages => routing::error_pages::page(site),
        Learn::BasePath => routing::base_path::page(site),
        // Forms and requests
        Learn::Forms => requests::forms::page(site),
        Learn::FormValidation => requests::validation::page(site),
        Learn::Requests => requests::extractors::page(site),
        Learn::Errors => requests::errors::page(site),
        Learn::Uploads => requests::uploads::page(site),
        // Data and sign-in
        Learn::Database => data::database::page(site),
        Learn::DatabaseWithoutSql => data::without_sql::page(site),
        Learn::CheckedSql => data::checked_sql::page(site),
        Learn::HostedDatabases => data::hosted::page(site),
        Learn::DatabaseHosts => data::hosts::page(site),
        Learn::Api => data::api::page(site),
        Learn::Sessions => data::sessions::page(site),
        Learn::MembersOnly => data::members::page(site),
        Learn::SessionCookie => data::cookie::page(site),
        Learn::SessionStores => data::stores::page(site),
        // Browser features
        Learn::NodeRefs
        | Learn::Timers
        | Learn::Keyboard
        | Learn::UncheckedValues
        | Learn::StaticPages
        | Learn::EscapeHatches => server::escape_hatches(site),
        Learn::JavaScript => javascript::javascript(site),
        // Quality and operations
        Learn::Security => security::security(site),
        Learn::Testing | Learn::TestingPages => testing::testing(site),
        Learn::CompileErrors | Learn::Debugging | Learn::Troubleshooting => {
            troubleshooting::troubleshooting(site)
        }
        Learn::Upgrading => upgrading::upgrading(site),
        // Deploy
        Learn::Deploying => deploying::deploying(site),
        Learn::DeployContainer => deploying::container::page(site),
        Learn::DeployCloudflare => deploying::cloudflare::page(site),
        Learn::DeployVercel => deploying::vercel::page(site),
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
