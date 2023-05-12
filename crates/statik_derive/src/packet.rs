use proc_macro2::TokenStream;
use syn::{Attribute, DeriveInput, Error, Expr, Lit, LitInt, Meta, Result};

pub fn expand_derive_packet(input: &mut DeriveInput) -> Result<TokenStream> {
    let DeriveInput {
        // attrs: _attrs,
        // vis: _vis,
        ident,
        // generics: _generics,
        data,
        ..
    } = input;

    Ok(quote!())
}
