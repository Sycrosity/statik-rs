use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, spanned::Spanned, Data, DeriveInput, Error, Fields, Ident, Path, Result};

pub fn derive_packet_group(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;

    let ident = input.ident;

    match &input.data {
        Data::Struct(s) => Err(Error::new(
            s.struct_token.span,
            "cannot derive `Packet` on structs YET",
        )),
        Data::Enum(e) => {
            let fields = e
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;

                    let enum_ctx = format!(
                        "enum must have unnamed fields: `{variant_name}` in `{}` is not an \
                         unnamed field.",
                        &ident
                    );

                    match &variant.fields {
                        Fields::Unnamed(fields) => {
                            if fields.unnamed.len() != 1 {
                                return Err(Error::new(
                                    fields.span(),
                                    format!("variants of {} must only have one field!", &ident),
                                ));
                            }

                            //SAFETY: can unwrap because of previous if statement checking length.
                            let field = fields.unnamed.first().unwrap();

                            let packet_name = match &field.ty {
                                syn::Type::Path(p) => &p.path,
                                _ => {
                                    return Err(Error::new(
                                        field.span(),
                                        format!(
                                            "Field of variant {variant_name} of {} must be a path!",
                                            &ident
                                        ),
                                    ));
                                }
                            };

                            Ok((packet_name, variant_name))
                        }
                        _ => Err(Error::new(variant.ident.span(), enum_ctx)),
                    }
                })
                .collect::<Result<Vec<(&Path, &Ident)>>>()?;

            let from_fields = fields
                .iter()
                .map(|(packet_name, variant_name)| {
                    quote! {

                        impl From<#packet_name> for #ident {
                            fn from(p: #packet_name) -> Self {
                                Self::#variant_name(p)
                            }
                        }
                    }
                })
                .collect::<TokenStream>();

            let decode_fields = fields
                .iter()
                .map(|(packet_name, variant_name)| {
                    quote! {
                        #packet_name::ID => {

                            Ok(Self::#variant_name(#packet_name::decode(&mut _buffer)?))

                        },
                    }
                })
                .collect::<TokenStream>();

            // let encode_fields = fields
            //     .iter()
            //     .map(|(packet_name, variant_name)| {
            //         // let ctx = format!("failed to encode packet `{packet_name}` in
            // `{ident}`");         quote! {
            //             #packet_name.encode(&mut _buffer)?;
            //             Ok(())
            //         }
            //     })
            //     .collect::<TokenStream>();

            Ok(quote! {

                #from_fields

                impl ::statik_core::packet::Decode for #ident {

                    fn decode(mut _buffer: impl ::std::io::Read) -> ::anyhow::Result<Self> {

                        use ::statik_core::{packet::{Decode, Packet}, varint::VarInt};
                        use ::anyhow::{Context, ensure, bail, Error};

                        match VarInt::decode(&mut _buffer)?.0 {

                            #decode_fields
                            _n => bail!("Invalid packet id! Tried to parse packet with id: {}", _n)
                        }
                    }
                }

                // impl ::statik_core::packet::Encode for #ident {

                //     fn encode(&self, mut _buffer: impl ::std::io::Write) -> ::anyhow::Result<Self> {

                //         use ::statik_core::{packet::Encode, varint::VarInt};
                //         use ::anyhow::{Context, ensure, bail, Error};

                //         VarInt(#id).encode(&mut _buffer)?;
                //         #encode_fields

                //         Ok(())
                //     }
                // }
            })
        }
        Data::Union(u) => Err(Error::new(
            u.union_token.span,
            "cannot derive `Packet` on unions",
        )),
    }
}
