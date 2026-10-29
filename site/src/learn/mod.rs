//! The Learn section, a guide read from top to bottom.

mod api;
mod database;
mod deploying;
mod forms;
mod javascript;
mod page;
mod requests;
mod security;
mod server;
mod sessions;
pub mod snippet;
mod start;
mod state;
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
        Learn::Signals
        | Learn::NestedSignals
        | Learn::ComputedSignals
        | Learn::PersistedSignals => state::signals(site),
        Learn::Expressions | Learn::Statements | Learn::Events | Learn::BindingInputs => {
            state::expressions(site)
        }
        Learn::Lists | Learn::ListItems => state::lists(site),
        // Talking to the server
        Learn::Actions
        | Learn::CallStatus
        | Learn::Optimistic
        | Learn::Patches
        | Learn::Fragments
        | Learn::Streams
        | Learn::Server => server::server(site),
        Learn::OverlappingCalls | Learn::ListPatches | Learn::CallsInDepth => {
            server::calls_in_depth(site)
        }
        // Pages and routing
        Learn::Routes | Learn::SearchParams | Learn::ErrorPages | Learn::BasePath => {
            server::routes_in_depth(site)
        }
        Learn::Links | Learn::ChildPages | Learn::Pages => server::pages(site),
        // Forms and requests
        Learn::Forms | Learn::FormValidation => forms::forms(site),
        Learn::Requests | Learn::Errors | Learn::Uploads => requests::requests(site),
        // Data and sign-in
        Learn::Database => database::database(site),
        Learn::Api => api::api(site),
        Learn::Sessions => sessions::sessions(site),
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
