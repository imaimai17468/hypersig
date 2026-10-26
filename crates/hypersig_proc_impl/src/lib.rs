//! Token transformations behind `#[derive(Signals)]`, `#[derive(Segment)]`, and `#[derive(Search)]`.
//!
//! This crate is a regular library so the expansion can be unit tested. The
//! `hypersig_proc` crate forwards to it, and users reach both through `hypersig`.

// A library's caller owns standard output and standard error, so the library writes to neither.
#![warn(clippy::print_stdout, clippy::print_stderr)]

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::ext::IdentExt as _;
use syn::spanned::Spanned as _;
use syn::{Data, DeriveInput, Fields, FieldsNamed};

/// Expands `#[derive(Signals)]` for the struct in `input`.
///
/// The expansion implements `hypersig::Signals` for the struct, with
/// `SHAPE` set to each field's name and type, and adds one associated
/// function per field, named after the field, returning a typed
/// `hypersig::Sig` for it. A field marked `#[signals(computed)]` gets a
/// `Sig<Self, T, hypersig::Computed>`, and every other field a
/// `Sig<Self, T>`. Where a field is marked, the impl also sets `COMPUTED`
/// to the names of the marked fields. It defines no other item.
///
/// Rejected inputs, each with a compile error at the offending span: enums,
/// unions, tuple and unit structs, generic structs, and any `#[serde(...)]`
/// attribute on the struct or a field, because a rename or a skip would make
/// the signal path disagree with the JSON the runtime sends, a `#[signals]`
/// attribute on the struct, and a `#[signals(...)]` on a field that holds
/// anything other than `computed`.
///
/// The error comes with the items a valid input would get, as far as the
/// input allows: the impl and the field functions for a struct with a serde
/// attribute, and the empty impl for an enum or a struct without named
/// fields. Code that uses them then compiles past them, so the rejection is
/// the only error the struct causes. A generic struct and a union get no
/// items, because the impl would need bounds or a `Serialize` impl the input
/// does not have.
#[must_use]
pub fn derive_signals(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse2(input) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error(),
    };
    let mut errors = Errors::default();
    errors.push(reject_serde_attributes(&input.attrs));
    errors.push(reject_struct_signals_attribute(&input.attrs));
    let name = &input.ident;
    let named = match named_fields(
        &input,
        "Signals",
        "because each field name becomes a signal path",
    ) {
        Ok(named) => named,
        Err(error) => {
            let impl_signals = (input.generics.params.is_empty()
                && !matches!(input.data, Data::Union(_)))
            .then(|| {
                quote! {
                    #[automatically_derived]
                    impl ::hypersig::Signals for #name {}
                }
            });
            errors.push(Err(error));
            let errors = errors.into_compile_error();
            return quote! { #errors #impl_signals };
        }
    };

    let mut accessors = Vec::with_capacity(named.named.len());
    let mut shape = Vec::with_capacity(named.named.len());
    let mut computed_fields = Vec::new();
    for field in &named.named {
        errors.push(reject_serde_attributes(&field.attrs));
        let computed = match is_computed(&field.attrs) {
            Ok(computed) => computed,
            Err(error) => {
                errors.push(Err(error));
                false
            }
        };
        let Some(ident) = &field.ident else {
            unreachable!("fields of a named struct always have an identifier");
        };
        let path = ident.unraw().to_string();
        let vis = &field.vis;
        let ty = &field.ty;
        shape.push(format!("{path}:{}", quote!(#ty)));
        let doc = format!("Typed reference to the `{path}` signal.");
        if computed {
            computed_fields.push(path.clone());
        }
        let accessor = if computed {
            quote! {
                #vis fn #ident() -> ::hypersig::Sig<Self, #ty, ::hypersig::Computed> {
                    ::hypersig::_private::computed_sig(#path)
                }
            }
        } else {
            quote! {
                #vis fn #ident() -> ::hypersig::Sig<Self, #ty> {
                    ::hypersig::_private::sig(#path)
                }
            }
        };
        accessors.push(quote! {
            #[doc = #doc]
            #[must_use]
            #[allow(dead_code, reason = "an accessor is generated for every field, used or not")]
            #accessor
        });
    }

    let errors = errors.into_compile_error();
    let shape = shape.join(",");
    let computed_const = (!computed_fields.is_empty()).then(|| {
        quote! { const COMPUTED: &'static [&'static str] = &[#(#computed_fields),*]; }
    });
    quote! {
        #errors

        #[automatically_derived]
        impl ::hypersig::Signals for #name {
            const SHAPE: &'static str = #shape;
            #computed_const
        }

        #[automatically_derived]
        impl #name {
            #(#accessors)*
        }
    }
}

/// The compile errors of one derive, gathered so that each shows.
#[derive(Default)]
struct Errors(Option<syn::Error>);

impl Errors {
    fn push(&mut self, result: syn::Result<()>) {
        if let Err(error) = result {
            match &mut self.0 {
                Some(errors) => errors.combine(error),
                None => self.0 = Some(error),
            }
        }
    }

    fn into_compile_error(self) -> Option<TokenStream> {
        self.0.map(|error| error.to_compile_error())
    }
}

/// Expands `#[derive(Search)]` for the struct in `input`.
///
/// The expansion implements `hypersig::Search` and its seal for the struct,
/// with one bound per field that the field's type implements
/// `hypersig::SearchValue`. A field of any other type fails that bound at the
/// field's type. It defines no other item.
///
/// Rejected inputs, each with a compile error at the offending span: enums,
/// unions, tuple and unit structs, and generic structs.
#[must_use]
pub fn derive_search(input: TokenStream) -> TokenStream {
    match expand_search(input) {
        Ok(tokens) => tokens,
        Err(error) => error.to_compile_error(),
    }
}

fn expand_search(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let named = named_fields(
        &input,
        "Search",
        "because each field name becomes a query parameter",
    )?;
    let bounds = named.named.iter().map(|field| {
        let ty = &field.ty;
        quote_spanned! {ty.span()=> #ty: ::hypersig::SearchValue}
    });
    let name = &input.ident;
    Ok(quote! {
        #[automatically_derived]
        impl ::hypersig::_private::SealedSearch for #name {}

        #[automatically_derived]
        impl ::hypersig::Search for #name
        where
            #name: ::hypersig::Signals,
            #(#bounds,)*
        {
            const TYPED: bool = true;
            type Args<P: ::hypersig::Params> = <P as ::hypersig::Params>::With<Self>;

            fn join<P: ::hypersig::Params>(params: P, search: Self) -> Self::Args<P> {
                <P as ::hypersig::Params>::join(params, search)
            }

            fn split<P: ::hypersig::Params>(args: Self::Args<P>) -> (P, Self) {
                <P as ::hypersig::Params>::split(args)
            }

            fn unit() -> ::core::option::Option<Self> {
                ::core::option::Option::None
            }

            fn query(&self) -> ::std::string::String {
                ::hypersig::_private::search_query(self)
            }
        }
    })
}

/// Expands `#[derive(Segment)]` for the enum in `input`.
///
/// The expansion implements `hypersig::Segment` and its seal for the enum,
/// and `hypersig::Variants` with every variant in declaration order. A
/// segment is the string serde writes for the variant: its name, renamed by
/// `#[serde(rename_all = "...")]` on the enum or `#[serde(rename = "...")]`
/// on the variant as serde renames it, so the server and the browser write
/// the same URL. It defines no other item.
///
/// Rejected inputs, each with a compile error at the offending span: structs,
/// unions, generic enums, enums without variants, variants with fields, an
/// unknown `rename_all` rule, and any other `#[serde(...)]` attribute on the
/// enum or a variant, because it could make serde write a variant as
/// something other than its segment.
///
/// The error comes with impls of `Segment`, its seal, and `Variants` that
/// parse no segment and list no variant, for any input but a generic one,
/// so a route or a `bind_select` over the type does not add errors of its
/// own.
#[must_use]
pub fn derive_segment(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse2(input) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error(),
    };
    match expand_segment(&input) {
        Ok(tokens) => tokens,
        Err(error) => {
            let error = error.to_compile_error();
            if !input.generics.params.is_empty() {
                return error;
            }
            let name = &input.ident;
            quote! {
                #error

                #[automatically_derived]
                impl ::hypersig::_private::Sealed for #name {}

                #[automatically_derived]
                impl ::hypersig::Segment for #name {
                    fn parse(_segment: &str) -> ::core::option::Option<Self> {
                        ::core::option::Option::None
                    }

                    fn write(&self) -> ::std::string::String {
                        ::core::unreachable!("the derive of `Segment` failed to compile")
                    }
                }

                #[automatically_derived]
                impl ::hypersig::Variants for #name {
                    const VARIANTS: &'static [Self] = &[];
                }
            }
        }
    }
}

fn expand_segment(input: &DeriveInput) -> syn::Result<TokenStream> {
    let rename_all = serde_rename(&input.attrs, "rename_all")?;

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
        let rename = serde_rename(&variant.attrs, "rename")?;
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                &variant.fields,
                "a `Segment` variant has no fields, because the segment is the variant's name",
            ));
        }
        idents.push(&variant.ident);
        let name = variant.ident.unraw().to_string();
        names.push(match (rename, &rename_all) {
            (Some(rename), _) => rename.value(),
            (None, Some(rule)) => apply_rename_all(rule, &name)?,
            (None, None) => name,
        });
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

        #[automatically_derived]
        impl ::hypersig::Variants for #name {
            const VARIANTS: &'static [Self] = &[#(Self::#idents),*];
        }
    })
}

/// The named fields of the struct in `input`, for the derive named `derive`.
///
/// A generic struct, a tuple or unit struct, an enum, and a union are each a
/// compile error at the offending span. `because` ends the message for a
/// struct whose fields have no names.
fn named_fields<'input>(
    input: &'input DeriveInput,
    derive: &str,
    because: &str,
) -> syn::Result<&'input FieldsNamed> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            format!("`{derive}` cannot be derived for a generic struct"),
        ));
    }
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => Ok(named),
            other => Err(syn::Error::new_spanned(
                other,
                format!("`{derive}` needs a struct with named fields, {because}"),
            )),
        },
        Data::Enum(data) => Err(syn::Error::new_spanned(
            data.enum_token,
            format!("`{derive}` can only be derived for a struct"),
        )),
        Data::Union(data) => Err(syn::Error::new_spanned(
            data.union_token,
            format!("`{derive}` can only be derived for a struct"),
        )),
    }
}

/// Whether `attrs` holds `#[signals(computed)]`, the one `signals` attribute a field takes.
fn is_computed(attrs: &[syn::Attribute]) -> syn::Result<bool> {
    let mut computed = false;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("signals")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("computed") {
                computed = true;
                Ok(())
            } else {
                Err(meta.error("`#[signals(...)]` takes only `computed`"))
            }
        })?;
    }
    Ok(computed)
}

/// Rejects `#[signals]` on the struct, since it marks a single field.
fn reject_struct_signals_attribute(attrs: &[syn::Attribute]) -> syn::Result<()> {
    match attrs.iter().find(|attr| attr.path().is_ident("signals")) {
        Some(attr) => Err(syn::Error::new_spanned(
            attr,
            "`#[signals(computed)]` goes on a field, not on the struct",
        )),
        None => Ok(()),
    }
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

/// The string of `#[serde(KEY = "...")]` in `attrs`, the one serde attribute a
/// `Segment` enum takes on that item.
///
/// Any other serde attribute is an error, because it could make serde write
/// a variant as something other than the string the segment holds.
fn serde_rename(attrs: &[syn::Attribute], key: &str) -> syn::Result<Option<syn::LitStr>> {
    let mut found = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("serde")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(key) {
                found = Some(meta.value()?.parse::<syn::LitStr>()?);
                Ok(())
            } else {
                Err(meta.error(format!(
                    "a `Segment` enum takes only `#[serde({key} = \"...\")]` here, because serde must write each variant as the string its segment holds"
                )))
            }
        })?;
    }
    Ok(found)
}

/// `name`, a variant's name, renamed by the `rename_all` rule `rule` as serde renames a variant.
fn apply_rename_all(rule: &syn::LitStr, name: &str) -> syn::Result<String> {
    let snake = || {
        let mut out = String::new();
        for (i, c) in name.char_indices() {
            if c.is_uppercase() && i > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        }
        out
    };
    Ok(match rule.value().as_str() {
        "lowercase" => name.to_lowercase(),
        "UPPERCASE" => name.to_uppercase(),
        "PascalCase" => name.to_owned(),
        "camelCase" => {
            let mut chars = name.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_lowercase().chain(chars).collect()
            })
        }
        "snake_case" => snake(),
        "SCREAMING_SNAKE_CASE" => snake().to_uppercase(),
        "kebab-case" => snake().replace('_', "-"),
        "SCREAMING-KEBAB-CASE" => snake().replace('_', "-").to_uppercase(),
        _ => {
            return Err(syn::Error::new_spanned(
                rule,
                "an unknown `rename_all` rule; serde takes `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, or `SCREAMING-KEBAB-CASE`",
            ));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{derive_search, derive_segment, derive_signals};
    use quote::quote;

    fn expand(input: proc_macro2::TokenStream) -> String {
        unparse(derive_signals(input))
    }

    fn expand_segment(input: proc_macro2::TokenStream) -> String {
        unparse(derive_segment(input))
    }

    fn expand_search(input: proc_macro2::TokenStream) -> String {
        unparse(derive_search(input))
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
    fn computed_field() {
        insta::assert_snapshot!(expand(quote! {
            struct Cart { price: f64, #[signals(computed)] total: f64 }
        }));
    }

    #[test]
    fn rejects_misplaced_signals_attribute() {
        insta::assert_snapshot!(expand(quote! {
            #[signals(computed)]
            struct R { #[signals(derived)] name: String, count: i32 }
        }));
    }

    #[test]
    fn rejects_each_serde_attribute() {
        insta::assert_snapshot!(expand(quote! {
            #[serde(deny_unknown_fields)]
            struct R { #[serde(skip)] name: String, count: i32 }
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
    fn segment_follows_serde_renames() {
        insta::assert_snapshot!(expand_segment(quote! {
            #[serde(rename_all = "kebab-case")]
            enum Locale { En, JaJp, #[serde(rename = "zh-Hant")] ZhHant }
        }));
    }

    #[test]
    fn rejects_segment_serde_untagged() {
        insta::assert_snapshot!(expand_segment(quote! {
            #[serde(untagged)]
            enum E { A }
        }));
    }

    #[test]
    fn named_struct_search() {
        insta::assert_snapshot!(expand_search(quote! {
            pub struct Filter { q: String, page: Option<i32> }
        }));
    }

    #[test]
    fn rejects_search_enum() {
        insta::assert_snapshot!(expand_search(quote! { enum E { A } }));
    }
}
