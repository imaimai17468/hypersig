//! The i18n section: typed translations from `hypersig_i18n`.

mod choosing;
mod pages;
mod start;
mod texts;

use crate::Site;
use crate::doc::{Doc, I18nPage};
use crate::prose::{Part, title};

/// The article of `page`.
pub fn content(site: Site, page: I18nPage) -> Part {
    let head = title(Doc::I18n(page).title(site));
    match page {
        // Get started
        I18nPage::Overview => start::overview::page(site, head),
        // Texts
        I18nPage::Locale => texts::locale::page(site, head),
        I18nPage::Texts => texts::writing::page(site, head),
        I18nPage::Messages => texts::messages::page(site, head),
        I18nPage::Formats => texts::formats::page(site, head),
        // Pages in each language
        I18nPage::Pages => pages::routes::page(site, head),
        I18nPage::LanguageSwitcher => pages::language_switcher::page(site, head),
        I18nPage::PageLanguage => pages::page_language::page(site, head),
        I18nPage::Alternates => pages::alternates::page(site, head),
        // Choosing the language
        I18nPage::Negotiation => choosing::negotiation::page(site, head),
        I18nPage::Redirect => choosing::redirect::page(site, head),
    }
}
