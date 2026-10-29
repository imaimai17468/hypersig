//! The Learn group on data and signing in: where a handler keeps the data
//! requests share, the services it calls, and the session that tells one
//! visitor from another.

pub mod api;
pub mod checked_sql;
pub mod cookie;
pub mod database;
pub mod hosted;
pub mod hosts;
pub mod members;
pub mod sessions;
pub mod stores;
pub mod without_sql;

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
