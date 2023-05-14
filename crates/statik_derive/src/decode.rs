use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Fields, Result};

pub fn expand_derive_decode(input: &mut DeriveInput) -> Result<TokenStream> {
    let DeriveInput {
        // attrs,
        // vis,
        ident,
        // generics,
        data,
        ..
    } = input;

    match data {
        syn::Data::Struct(s) => {
            let decode_fields = match &s.fields {
                Fields::Named(fields) => {
                    let init = fields.named.iter().map(|f| {
                        let name = f.ident.as_ref().unwrap();
                        let ctx = format!("failed to decode field `{name}` in `{ident}`");
                        quote! {
                            #name: ::statik_common::prelude::Decode::decode(_buffer).context(#ctx)?,
                        }
                    });

                    quote! {
                        Self {
                            #(#init)*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let init = (0..fields.unnamed.len())
                        .map(|i| {
                            let ctx = format!("failed to decode field `{i}` in `{ident}`");
                            quote! {
                                ::statik_common::prelude::Decode::decode(&mut _buffer).context(#ctx)?,
                            }
                        })
                        .collect::<TokenStream>();

                    quote! {
                        Self(#init)
                    }
                }
                Fields::Unit => quote!(Self),
            };

            Ok(quote! {
                #[allow(unused_imports)]
                impl ::statik_common::packet::Decode for #ident
                {
                    fn decode(mut _buffer: impl ::std::io::Read) -> ::anyhow::Result<Self> {

                        use ::statik_common::packet::Decode;
                        use ::anyhow::{Context, ensure};

                        Ok(#decode_fields)
                    }
                }
            })
        }
        syn::Data::Enum(e) => Err(Error::new(
            e.enum_token.span,
            "cannot derive `Decode` on enums YET",
        )),
        syn::Data::Union(u) => Err(Error::new(
            u.union_token.span,
            "cannot derive `Decode` on unions",
        )),
    }
    // Ok(quote!())
}
