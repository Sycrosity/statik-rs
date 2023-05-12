use proc_macro2::TokenStream;
use syn::{DeriveInput, Result, LitInt, Attribute, Meta, Lit, Error, Expr};

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
