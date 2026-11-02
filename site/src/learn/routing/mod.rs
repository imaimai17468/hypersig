//! The pages on routes: declaring them, linking to them, nesting them,
//! typing their query strings, answering the requests they do not serve,
//! and serving them under a path prefix.

pub(super) mod base_path;
pub(super) mod child_pages;
pub(super) mod error_pages;
pub(super) mod links;
pub(super) mod routes;
pub(super) mod search_params;
