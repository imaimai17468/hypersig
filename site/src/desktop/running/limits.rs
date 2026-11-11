//! The page on what the windows share and what each system needs.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, lead, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([head, lead(site, Doc::Desktop(DesktopPage::Limits).leads())])
}
