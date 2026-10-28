//! The development server: `cargo hypersig dev`, the reload it causes, and
//! `cargo hypersig new`.

use crate::Site;
use crate::prose::Part;

pub fn page(site: Site) -> Part {
    super::quick_start::page(site)
}
