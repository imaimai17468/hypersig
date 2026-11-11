//! The page on opening, closing, and focusing windows, from a handler and from a page.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, lead, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([head, lead(site, Doc::Desktop(DesktopPage::Windows).leads())])
}
