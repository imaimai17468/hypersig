//! Derive macros for hypersig. Depend on `hypersig` and use the re-export there.
// A library's caller owns standard output and standard error, so the library writes to neither.
#![warn(clippy::print_stdout, clippy::print_stderr)]
#![doc(
    html_logo_url = "https://imaimai17468.github.io/hypersig/icon.svg",
    html_favicon_url = "https://imaimai17468.github.io/hypersig/icon.svg"
)]

use proc_macro::TokenStream;

/// Implements `hypersig::Signals` and adds a typed accessor per field.
///
/// See `hypersig::Signals` for what the expansion contains.
#[proc_macro_derive(Signals, attributes(signals))]
pub fn derive_signals(input: TokenStream) -> TokenStream {
    hypersig_proc_impl::derive_signals(input.into()).into()
}

/// Implements `hypersig::Segment` for an enum whose variants have no fields.
///
/// It also implements `hypersig::Variants`, which lists the variants. See
/// `hypersig::Segment` for what the expansion contains.
#[proc_macro_derive(Segment)]
pub fn derive_segment(input: TokenStream) -> TokenStream {
    hypersig_proc_impl::derive_segment(input.into()).into()
}

/// Checks that each field of a struct can be written into a query string.
///
/// See `hypersig::Search` for what the expansion contains.
#[proc_macro_derive(Search)]
pub fn derive_search(input: TokenStream) -> TokenStream {
    hypersig_proc_impl::derive_search(input.into()).into()
}
