//! Token transformation behind `#[derive(Signals)]`.
//!
//! This crate is a regular library so the expansion can be unit tested. The
//! `hypersig_proc` crate forwards to it, and users reach both through `hypersig`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::ext::IdentExt as _;
use syn::{Data, DeriveInput, Fields};

/// Expands `#[derive(Signals)]` for the struct in `input`.
///
/// The expansion implements `hypersig::Signals` for the struct and adds one
/// associated function per field, named after the field, returning a typed
/// `hypersig::Sig` for it. It defines no other item.
///
/// Rejected inputs, each with a compile error at the offending span: enums,
/// unions, tuple and unit structs, generic structs, and any `#[serde(...)]`
/// attribute on the struct or a field, because a rename or a skip would make
/// the signal path disagree with the JSON the runtime sends.
#[must_use]
pub fn derive_signals(input: TokenStream) -> TokenStream {
    match expand(input) {
        Ok(tokens) => tokens,
        Err(error) => error.to_compile_error(),
    }
}

fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    reject_serde_attributes(&input.attrs)?;

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "`Signals` cannot be derived for a generic struct",
        ));
    }

    let named = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => named,
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "`Signals` needs a struct with named fields, because each field name becomes a signal path",
                ));
            }
        },
        Data::Enum(data) => {
            return Err(syn::Error::new_spanned(
                data.enum_token,
                "`Signals` can only be derived for a struct",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new_spanned(
                data.union_token,
                "`Signals` can only be derived for a struct",
            ));
        }
    };

    let mut accessors = Vec::with_capacity(named.named.len());
    for field in &named.named {
        reject_serde_attributes(&field.attrs)?;
        let Some(ident) = &field.ident else {
            unreachable!("fields of a named struct always have an identifier");
        };
        let path = ident.unraw().to_string();
        let vis = &field.vis;
        let ty = &field.ty;
        let doc = format!("Typed reference to the `{path}` signal.");
        accessors.push(quote! {
            #[doc = #doc]
            #[must_use]
            #[allow(dead_code, reason = "an accessor is generated for every field, used or not")]
            #vis fn #ident() -> ::hypersig::Sig<Self, #ty> {
                ::hypersig::_private::sig(#path)
            }
        });
    }

    let name = &input.ident;
    Ok(quote! {
        #[automatically_derived]
        impl ::hypersig::Signals for #name {}

        #[automatically_derived]
        impl #name {
            #(#accessors)*
        }
    })
}

fn reject_serde_attributes(attrs: &[syn::Attribute]) -> syn::Result<()> {
    match attrs.iter().find(|attr| attr.path().is_ident("serde")) {
        Some(attr) => Err(syn::Error::new_spanned(
            attr,
            "`#[serde(...)]` is not supported on a `Signals` struct, because the signal path must equal the field name",
        )),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::derive_signals;
    use quote::quote;

    fn expand(input: proc_macro2::TokenStream) -> String {
        let output = derive_signals(input);
        match syn::parse2::<syn::File>(output.clone()) {
            Ok(file) => prettyplease::unparse(&file),
            Err(_) => output.to_string(),
        }
    }

    #[test]
    fn named_struct() {
        insta::assert_snapshot!(expand(quote! {
            pub struct Counter {
                pub count: i32,
                label: String,
                pub r#type: bool,
            }
        }));
    }

    #[test]
    fn rejects_enum() {
        insta::assert_snapshot!(expand(quote! { enum E { A } }));
    }

    #[test]
    fn rejects_tuple_struct() {
        insta::assert_snapshot!(expand(quote! { struct T(i32); }));
    }

    #[test]
    fn rejects_generic_struct() {
        insta::assert_snapshot!(expand(quote! { struct G<T> { value: T } }));
    }

    #[test]
    fn rejects_serde_rename() {
        insta::assert_snapshot!(expand(quote! {
            struct R { #[serde(rename = "n")] name: String }
        }));
    }
}
