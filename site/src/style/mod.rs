//! The Style section: typed atomic CSS from `hypersig::style`.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::prose::{Part, title};

mod animation;
mod conditions;
mod global;
mod layout;
mod start;
mod tokens;
mod unchecked;
mod values;

pub use start::overview::benchmarks;

/// The article of `page`.
pub fn content(site: Site, page: StylePage) -> Part {
    let head = title(Doc::Style(page).title(site));
    match page {
        // Get started
        StylePage::Overview => start::overview::page(site, head),
        StylePage::Applying => start::applying::page(site, head),
        // Values
        StylePage::Values => values::properties::page(site, head),
        StylePage::Lengths => values::lengths::page(site, head),
        StylePage::Colors => values::colors::page(site, head),
        StylePage::AnglesAndTimes => values::angles_and_times::page(site, head),
        StylePage::UrlsAndStrings => values::urls_and_strings::page(site, head),
        StylePage::Keywords => values::keywords::page(site, head),
        // Conditions
        StylePage::PseudoClasses => conditions::pseudo_classes::page(site, head),
        StylePage::PseudoElements => conditions::pseudo_elements::page(site, head),
        StylePage::Media => conditions::media::page(site, head),
        StylePage::Containers => conditions::containers::page(site, head),
        StylePage::GroupAndPeer => conditions::group_and_peer::page(site, head),
        StylePage::Toggled => conditions::toggled::page(site, head),
        // Tokens and themes
        StylePage::Tokens => tokens::token::page(site, head),
        StylePage::Themes => tokens::themes::page(site, head),
        StylePage::DarkMode => tokens::dark_mode::page(site, head),
        // Global styles
        StylePage::Global => global::rules::page(site, head),
        StylePage::Fonts => global::fonts::page(site, head),
        // Layout
        StylePage::Stacks => layout::stacks::page(site),
        StylePage::Grids => layout::grids::page(site),
        StylePage::Patterns => layout::patterns::page(site),
        // Animation
        StylePage::Animation => animation::keyframes::page(site),
        StylePage::Transitions => animation::transitions::page(site),
        StylePage::PageTransitions => animation::page_transitions::page(site),
        StylePage::ReducedMotion => animation::reduced_motion::page(site),
        // Opting out of type checks
        StylePage::Unchecked => unchecked::values::page(site),
    }
}
