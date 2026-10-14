//! Derive macro for hypersig. Depend on `hypersig` and use the re-export there.

use proc_macro::TokenStream;

/// Implements `hypersig::Signals` and adds a typed accessor per field.
///
/// See `hypersig::Signals` for what the expansion contains.
#[proc_macro_derive(Signals)]
pub fn derive_signals(input: TokenStream) -> TokenStream {
    hypersig_proc_impl::derive_signals(input.into()).into()
}
