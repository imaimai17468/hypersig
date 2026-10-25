//! Short code samples, each a file under `site/snippets/` that `cargo test`
//! runs as a doctest, so a sample that stops compiling fails the tests.
//!
//! A line that starts with `# ` is compiled and left out of the page, as in
//! rustdoc. A sample shown as a change is the code after it: a line that ends
//! in `//+` was added, and a comment line that starts with `//-` holds a line
//! the change removed, so the doctest compiles what the change leaves.

use crate::code::{self, Change, Lang};
use crate::prose::Part;

macro_rules! snippets {
    ($($name:ident => $file:literal $(, $attr:literal)?;)*) => {
        /// One code sample.
        #[derive(Debug, Clone, Copy)]
        pub enum Snippet {
            $($name,)*
        }

        impl Snippet {
            fn source(self) -> &'static str {
                match self {
                    $(Self::$name => include_str!(concat!("../../snippets/", $file)),)*
                }
            }
        }

        #[cfg(doctest)]
        mod doctests {
            $(
                #[doc = concat!("```", $($attr,)? "\n", include_str!(concat!("../../snippets/", $file)), "```")]
                struct $name;
            )*
        }
    };
}

snippets! {
    Elements => "elements.rs";
    Foreign => "foreign.rs";
    ElementsModule => "elements_module.rs";
    ElementsPage => "elements_page.rs";
    Component => "component.rs";
    Bindings => "bindings.rs";
    SignalPaths => "signal_paths.rs";
    WrongField => "wrong_field.rs", "compile_fail,E0599";
    Nested => "nested.rs";
    Operators => "operators.rs";
    WrongType => "wrong_type.rs", "compile_fail,E0277";
    WhyCounter => "why_counter.rs";
    Statements => "statements.rs";
    Events => "events.rs";
    Each => "each.rs";
    ListStatements => "list_statements.rs";
    Patches => "patches.rs";
    ListPatches => "list_patches.rs";
    Forms => "forms.rs";
    Latest => "latest.rs";
    Parts => "parts.rs";
    Extractors => "extractors.rs";
    HandlerErrors => "handler_errors.rs";
    Uploads => "uploads.rs";
    Sessions => "sessions.rs";
    Database => "database.rs", "no_run";
    DynamoDb => "dynamodb.rs", "no_run";
    Api => "api.rs", "no_run";
    Links => "links.rs";
    WrongRoute => "wrong_route.rs", "compile_fail,E0080";
    ChildRoutes => "child_routes.rs";
    Navigation => "navigation.rs";
    History => "history.rs";
    Head => "head.rs";
    SearchParams => "search_params.rs";
    Styles => "styles.rs";
    Effects => "effects.rs";
    StaticPage => "static_page.rs";
    UiSetup => "ui_setup.rs";
    UiTheme => "ui_theme.rs";
    StyleBasics => "style_basics.rs";
    StyleValues => "style_values.rs";
    WrongValue => "wrong_value.rs", "compile_fail,E0277";
    Conditions => "conditions.rs";
    GroupPeer => "group_peer.rs";
    Tokens => "tokens.rs";
    GlobalRules => "global_rules.rs";
    Patterns => "patterns.rs";
    Animation => "animation.rs";
    Unchecked => "unchecked.rs";
    I18nLocale => "i18n_locale.rs";
    I18nMissing => "i18n_missing.rs", "compile_fail,E0063";
    I18nPages => "i18n_pages.rs";
    I18nNegotiate => "i18n_negotiate.rs";
    // The changes the use cases page shows, each written as the code after
    // the change, so the doctest compiles what the page adds.
    UseOthersChanges => "use_cases/others_changes.rs";
    UseServerError => "use_cases/server_error.rs";
    UseLatest => "use_cases/latest.rs";
    UseAuction => "use_cases/auction.rs";
    UseDeployLog => "use_cases/deploy_log.rs";
    UseKitchen => "use_cases/kitchen.rs";
    UseSeats => "use_cases/seats.rs";
    UseUrlFilter => "use_cases/url_filter.rs";
    UseTranslations => "use_cases/translations.rs";
    // The code `cargo hypersig add server` prints, so the page and the CLI
    // show one text.
    Listen => "../../crates/hypersig_cli/steps/server.rs";
    SafeUrl => "safe_url.rs";
}

/// The lines of `snippet` that the page shows.
fn shown(snippet: Snippet) -> impl Iterator<Item = &'static str> {
    snippet
        .source()
        .lines()
        .filter(|line| !(line.starts_with("# ") || *line == "#"))
}

/// `snippet` as a code block, without its hidden lines.
pub fn show(snippet: Snippet) -> Part {
    let shown: String = shown(snippet).flat_map(|line| [line, "\n"]).collect();
    code::block(Lang::Rust, None, &shown)
}

/// `snippet` as a code block headed by the file name `file`.
pub fn show_file(snippet: Snippet, file: &str) -> Part {
    let shown: String = shown(snippet).flat_map(|line| [line, "\n"]).collect();
    code::block(Lang::Rust, Some(file), &shown)
}

/// `snippet` as a change, with its added and removed lines marked.
pub fn show_change(snippet: Snippet) -> Part {
    code::diff(
        Lang::Rust,
        shown(snippet).map(|line| {
            if let Some(removed) = line.strip_prefix("//-") {
                (Change::Removed, removed)
            } else if let Some(added) = line.strip_suffix("//+") {
                (Change::Added, added.trim_end())
            } else {
                (Change::Kept, line)
            }
        }),
    )
}
