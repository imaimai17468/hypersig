//! The page on smoke runs of the real window in CI.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, lead, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::SmokeRuns).leads()),
    ])
}
