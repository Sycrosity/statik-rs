#![allow(unused)]

mod packet;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{DeriveInput, GenericParam, Generics};

#[proc_macro_derive(Packet, attributes(packet_id))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    packet::expand_derive_packet(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
