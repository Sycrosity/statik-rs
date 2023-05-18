use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields, Ident, Result};

pub fn expand_derive_packet_group(input: &mut DeriveInput) -> Result<TokenStream> {
    let DeriveInput {
        // attrs,
        // vis,
        ident: input_name,
        // generics,
        data,
        ..
    } = input;

    match data {
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
                        "enum must have unnamed fields: `{variant_name}` in `{input_name}` is not \
                         an unnamed field.",
                    );

                    match &variant.fields {
                        Fields::Unnamed(fields) => {
                            if fields.unnamed.len() != 1 {
                                return Err(Error::new(
                                    fields.span(),
                                    format!("variants of {input_name} must only have one field!",),
                                ));
                            }

                            //SAFETY: can unwrap because of previous if statement checking length.
                            let field = fields.unnamed.first().unwrap();

                            let packet_name = match &field.ty {
                                syn::Type::Path(p) => {
                                    if let Some(ident) = p.path.get_ident() {
                                        ident
                                    } else {
                                        return Err(Error::new(
                                            field.span(),
                                            format!(
                                                "(shouldn't be possible) Field of variant \
                                                 {variant_name} of {input_name} must have an \
                                                 ident!",
                                            ),
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(Error::new(
                                        field.span(),
                                        format!(
                                            "Field of variant {variant_name} of {input_name} must \
                                             be a path!",
                                        ),
                                    ));
                                }
                            };

                            Ok((packet_name, variant_name))
                        }
                        _ => Err(Error::new(variant.ident.span(), enum_ctx)),
                    }
                })
                .collect::<Result<Vec<(&Ident, &Ident)>>>()?;

            let from_fields = fields
                .iter()
                .map(|(packet_name, variant_name)| {
                    quote! {
                        impl From<#packet_name> for #input_name {
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
                        #packet_name::PACKET_ID => {

                            Ok(Self::#variant_name(#packet_name::decode(&mut _buffer)?))

                        },
                    }
                })
                .collect::<TokenStream>();

            Ok(quote! {

                #from_fields

                impl ::statik_common::packet::Decode for #input_name {

                    fn decode(mut _buffer: impl ::std::io::Read) -> ::anyhow::Result<Self> {

                        use ::statik_common::{packet::{Decode, Packet}, varint::VarInt};
                        use ::anyhow::{Context, ensure, bail, Error};

                        match VarInt::decode(&mut _buffer)?.0 {

                            #decode_fields
                            _n => bail!("Invalid packet id! Tried to parse packet with id: {}", _n)
                        }
                    }
                }
            })

            // encode_arms
            // Ok(quote! {
            //     #[allow(unused_imports, unreachable_code)]
            //     impl #impl_generics ::valence_core::__private::Encode for
            // #input_name #ty_generics     #where_clause
            //     {
            //         fn encode(&self, mut _w: impl ::std::io::Write) ->
            // ::valence_core::__private::Result<()> {
            // use ::valence_core::__private::{Encode, VarInt, Context};

            //             match self {
            //                 #encode_arms
            //                 _ => unreachable!(),
            //             }
            //         }
            //     }
            // })
        }
        Data::Union(u) => Err(Error::new(
            u.union_token.span,
            "cannot derive `Packet` on unions",
        )),
    }
}
