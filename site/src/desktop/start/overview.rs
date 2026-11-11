//! The Desktop overview: adding the crate, the `main` that opens the window, and a map of the section.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, lead, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Overview).leads()),
    ])
}
