//! The page on testing the handlers and pages with `FakeOs`.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, lead, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([head, lead(site, Doc::Desktop(DesktopPage::Testing).leads())])
}
