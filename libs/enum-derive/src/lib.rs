//! Procedural macro to derive `EnumList` for field-less enums.

#![recursion_limit = "128"]
#![deny(warnings)]

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Ident};

use errors::*;

mod errors;

/// Derives `EnumList` for field-less enums.
#[proc_macro_derive(EnumList)]
pub fn enum_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn derive(input: proc_macro::TokenStream) -> Result<TokenStream, syn::Error> {
    let ast = syn::parse::<DeriveInput>(input)?;
    if !ast.generics.params.is_empty() {
        return Err(Error::GenericsUnsupported.with_tokens(&ast.generics));
    }
    let ty = &ast.ident;
    let list_ty = Ident::new(&(ty.to_string() + "EnumList"), Span::call_site());
    let variants = match &ast.data {
        syn::Data::Enum(e) => &e.variants,
        _ => return Err(Error::ExpectedEnum.with_tokens(&ast)),
    };
    let arms = variants
        .iter()
        .enumerate()
        .map(|(_, v)| {
            let id = &v.ident;
            match v.fields {
                syn::Fields::Unit => Ok(quote! { #ty::#id }),
                _ => Err(Error::ExpectedUnitVariant.with_tokens(v)),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let tokens = quote! {
        pub static #list_ty: &[#ty] = &[
            #(#arms),*
        ];

        impl #ty {
            pub fn enum_list() -> &'static [#ty] {
                &#list_ty
            }
        }
    };
    let tokens = quote! {
        const _: () = {
            #tokens
        };
    };
    Ok(tokens)
}
