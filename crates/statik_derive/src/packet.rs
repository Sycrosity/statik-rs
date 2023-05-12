use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

pub fn expand_derive_packet(input: &mut DeriveInput) -> Result<TokenStream> {
    // println!("{input:#?}");

    Ok(quote!())
}
