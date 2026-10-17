//! Token transformations behind `#[derive(Signals)]` and `#[derive(Segment)]`.
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

/// Expands `#[derive(Segment)]` for the enum in `input`.
///
/// The expansion implements `hypersig::Segment` and its seal for the enum. A
/// segment is the variant's name, which is also the JSON string serde writes
/// for the variant, so the server and the browser write the same URL. It
/// defines no other item.
///
/// Rejected inputs, each with a compile error at the offending span: structs,
/// unions, generic enums, enums without variants, variants with fields, and
/// any `#[serde(...)]` attribute on the enum or a variant, because a rename
/// would make the segment disagree with the variant's name.
#[must_use]
pub fn derive_segment(input: TokenStream) -> TokenStream {
    match expand_segment(input) {
        Ok(tokens) => tokens,
        Err(error) => error.to_compile_error(),
    }
}

fn expand_segment(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    reject_serde(&input.attrs, SEGMENT_SERDE)?;

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "`Segment` cannot be derived for a generic enum",
        ));
    }

    let data = match &input.data {
        Data::Enum(data) => data,
        Data::Struct(data) => {
            return Err(syn::Error::new_spanned(
                data.struct_token,
                "`Segment` can only be derived for an enum whose variants have no fields",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new_spanned(
                data.union_token,
                "`Segment` can only be derived for an enum whose variants have no fields",
            ));
        }
    };
    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`Segment` needs an enum with at least one variant",
        ));
    }

    let mut idents = Vec::with_capacity(data.variants.len());
    let mut names = Vec::with_capacity(data.variants.len());
    for variant in &data.variants {
        reject_serde(&variant.attrs, SEGMENT_SERDE)?;
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                &variant.fields,
                "a `Segment` variant has no fields, because the segment is the variant's name",
            ));
        }
        idents.push(&variant.ident);
        names.push(variant.ident.unraw().to_string());
    }

    let name = &input.ident;
    Ok(quote! {
        #[automatically_derived]
        impl ::hypersig::_private::Sealed for #name {}

        #[automatically_derived]
        impl ::hypersig::Segment for #name {
            fn parse(segment: &str) -> ::core::option::Option<Self> {
                match segment {
                    #(#names => ::core::option::Option::Some(Self::#idents),)*
                    _ => ::core::option::Option::None,
                }
            }

            fn write(&self) -> ::std::string::String {
                ::std::string::String::from(match self {
                    #(Self::#idents => #names,)*
                })
            }
        }
    })
}

fn reject_serde_attributes(attrs: &[syn::Attribute]) -> syn::Result<()> {
    reject_serde(
        attrs,
        "`#[serde(...)]` is not supported on a `Signals` struct, because the signal path must equal the field name",
    )
}

fn reject_serde(attrs: &[syn::Attribute], message: &str) -> syn::Result<()> {
    match attrs.iter().find(|attr| attr.path().is_ident("serde")) {
        Some(attr) => Err(syn::Error::new_spanned(attr, message)),
        None => Ok(()),
    }
}

const SEGMENT_SERDE: &str = "`#[serde(...)]` is not supported on a `Segment` enum, because the segment must equal the variant's name";

#[cfg(test)]
mod tests {
    use super::{derive_segment, derive_signals};
    use quote::quote;

    fn expand(input: proc_macro2::TokenStream) -> String {
        unparse(derive_signals(input))
    }

    fn expand_segment(input: proc_macro2::TokenStream) -> String {
        unparse(derive_segment(input))
    }

    fn unparse(output: proc_macro2::TokenStream) -> String {
        let text = output.to_string();
        match syn::parse2::<syn::File>(output) {
            Ok(file) => prettyplease::unparse(&file),
            Err(_) => text,
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

    #[test]
    fn unit_enum_segment() {
        insta::assert_snapshot!(expand_segment(quote! {
            pub enum Color { Red, Green, r#Type }
        }));
    }

    #[test]
    fn rejects_segment_variant_with_fields() {
        insta::assert_snapshot!(expand_segment(quote! { enum E { A, B(i32) } }));
    }

    #[test]
    fn rejects_segment_struct() {
        insta::assert_snapshot!(expand_segment(quote! { struct S; }));
    }

    #[test]
    fn rejects_segment_serde_rename() {
        insta::assert_snapshot!(expand_segment(quote! {
            #[serde(rename_all = "kebab-case")]
            enum E { A }
        }));
    }
}
