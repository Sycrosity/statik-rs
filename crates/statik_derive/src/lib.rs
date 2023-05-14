mod decode;
mod encode;
mod packet;
mod packet_group;
// mod packet_group;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;

#[proc_macro_derive(Packet, attributes(packet_id))]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    packet::expand_derive_packet(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(PacketGroup)]
pub fn derive_packet_group(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    packet_group::expand_derive_packet_group(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    encode::expand_derive_encode(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    decode::expand_derive_decode(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(nightly)]
#[proc_macro_derive(PrintTokenStream)]
pub fn derive_print_token_stream(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    println!("{input:#?}");

    TokenStream::new()
}
