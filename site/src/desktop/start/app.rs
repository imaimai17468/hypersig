//! The page on `App`: the identifier, the window's size, the data directory, and the run's lifetime.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::prose::{Part, lead, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([head, lead(site, Doc::Desktop(DesktopPage::TheApp).leads())])
}
