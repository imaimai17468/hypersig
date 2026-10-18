//! Short code samples, each a file under `site/snippets/` that `cargo test`
//! runs as a doctest, so a sample that stops compiling fails the tests.
//!
//! A line that starts with `# ` is compiled and left out of the page, as in
//! rustdoc.

use crate::code::{self, Lang};
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
    Forms => "forms.rs";
    Latest => "latest.rs";
    Extractors => "extractors.rs";
    HandlerErrors => "handler_errors.rs";
    Links => "links.rs";
    WrongRoute => "wrong_route.rs", "compile_fail,E0080";
    ChildRoutes => "child_routes.rs";
    Navigation => "navigation.rs";
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
}

/// `snippet` as a code block, without its hidden lines.
pub fn show(snippet: Snippet) -> Part {
    let shown: String = snippet
        .source()
        .lines()
        .filter(|line| !(line.starts_with("# ") || *line == "#"))
        .flat_map(|line| [line, "\n"])
        .collect();
    code::block(Lang::Rust, None, &shown)
}
