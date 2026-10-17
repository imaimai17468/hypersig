//! Derive macros for hypersig. Depend on `hypersig` and use the re-export there.

use proc_macro::TokenStream;

/// Implements `hypersig::Signals` and adds a typed accessor per field.
///
/// See `hypersig::Signals` for what the expansion contains.
#[proc_macro_derive(Signals)]
pub fn derive_signals(input: TokenStream) -> TokenStream {
    hypersig_proc_impl::derive_signals(input.into()).into()
}

/// Implements `hypersig::Segment` for an enum whose variants have no fields.
///
/// See `hypersig::Segment` for what the expansion contains.
#[proc_macro_derive(Segment)]
pub fn derive_segment(input: TokenStream) -> TokenStream {
    hypersig_proc_impl::derive_segment(input.into()).into()
}
