//! The page on links to other sites and `Os::open_external`.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, lead, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::ExternalLinks).leads()),
    ])
}
