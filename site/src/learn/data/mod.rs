//! The Learn group on data and signing in: where a handler keeps the data
//! requests share, the services it calls, and the session that tells one
//! visitor from another.

pub(super) mod api;
pub(super) mod checked_sql;
pub(super) mod cookie;
pub(super) mod database;
pub(super) mod hosted;
pub(super) mod hosts;
pub(super) mod members;
pub(super) mod sessions;
pub(super) mod stores;
pub(super) mod without_sql;

use crate::Site;
use crate::lang::L;
use crate::prose::{Part, bullets, heading, parts};

/// The closing section of a page: the pages to read after it.
fn next(site: Site, pages: &[L<&str>]) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        bullets(site, pages),
    ])
}
