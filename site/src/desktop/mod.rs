//! The Desktop section: running an application in a window with `hypersig_desktop`.

mod running;
mod shipping;
mod start;
mod system;
mod testing;
mod windows;

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, title};

/// The start-up and size of `hypersig_desktop` beside Tauri and Electron.
pub(crate) fn benchmark(site: Site) -> Part {
    start::why::results(site)
}

/// The article of `page`.
pub(crate) fn content(site: Site, page: DesktopPage) -> Part {
    let head = title(Doc::Desktop(page).title(site));
    match page {
        DesktopPage::Why => start::why::page(site, head),
        DesktopPage::Overview => start::overview::page(site, head),
        DesktopPage::TheApp => start::app::page(site, head),
        DesktopPage::Windows => windows::page(site, head),
        DesktopPage::Dialogs => system::dialogs::page(site, head),
        DesktopPage::Clipboard => system::clipboard::page(site, head),
        DesktopPage::Badge => system::badge::page(site, head),
        DesktopPage::Notifications => system::notifications::page(site, head),
        DesktopPage::Tray => system::tray::page(site, head),
        DesktopPage::Devices => system::devices::page(site, head),
        DesktopPage::Shortcuts => system::shortcuts::page(site, head),
        DesktopPage::Menus => system::menus::page(site, head),
        DesktopPage::Autostart => system::autostart::page(site, head),
        DesktopPage::ExternalLinks => system::external_links::page(site, head),
        DesktopPage::Testing => testing::fake_os::page(site, head),
        DesktopPage::SmokeRuns => testing::smoke_runs::page(site, head),
        DesktopPage::Packaging => shipping::packaging::page(site, head),
        DesktopPage::Updates => shipping::updates::page(site, head),
        DesktopPage::Security => running::security::page(site, head),
        DesktopPage::PersistedSignals => running::persisted_signals::page(site, head),
        DesktopPage::Limits => running::limits::page(site, head),
    }
}
