use proc_macro2::{Span, TokenStream};
use syn::{Attribute, Data, DeriveInput, Error, Expr, Fields, Lit, LitInt, Meta, Result};

pub fn expand_derive_packet(input: &mut DeriveInput) -> Result<TokenStream> {
    let DeriveInput {
        attrs,
        // vis,
        ident,
        // generics,
        data,
        ..
    } = input;

    let Some(packet_id) = extract_packet_id_attr(attrs)? else {
        return Err(Error::new(
            input.ident.span(),
            "cannot derive `Packet` without `#[packet_id = ...]` helper attribute",
        ))
    };

    match data {
        Data::Struct(s) => {
            let decode_fields = match &s.fields {
                Fields::Named(fields) => {
                    let init = fields.named.iter().map(|f| {
                        let name = f.ident.as_ref().unwrap();
                        let ctx = format!("failed to decode field `{name}` in `{ident}`");
                        quote! {
                            #name: ::statik_common::prelude::Decode::decode(&mut _buffer).context(#ctx)?,
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

            let encode_fields = match &s.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident.as_ref().unwrap();
                        let ctx = format!("failed to encode field `{name}` in `{ident}`");
                        quote! {
                            self.#name.encode(&mut _buffer).context(#ctx)?;
                        }
                    })
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|i| {
                        let lit = LitInt::new(&i.to_string(), Span::call_site());
                        let ctx = format!("failed to encode field `{lit}` in `{ident}`");
                        quote! {
                            self.#lit.encode(&mut _buffer).context(#ctx)?;
                        }
                    })
                    .collect(),
                Fields::Unit => TokenStream::new(),
            };

            Ok(quote! {
                #[allow(unused_imports)]
                impl ::statik_common::packet::Encode for #ident
                {
                    fn encode(&self, mut _buffer: impl ::std::io::Write) -> ::anyhow::Result<()> {

                        use ::statik_common::{packet::Encode, varint::VarInt};
                        use ::anyhow::{Context, ensure};

                        VarInt(#packet_id).encode(&mut _buffer)?;
                        #encode_fields

                        Ok(())
                    }
                }

                #[allow(unused_imports)]
                impl ::statik_common::packet::Decode for #ident
                {
                    fn decode(mut _buffer: impl ::std::io::Read) -> ::anyhow::Result<Self> {

                        use ::statik_common::{packet::Decode, varint::VarInt};
                        use ::anyhow::{Context, ensure};

                        let id = VarInt::decode(&mut _buffer).context("failed to decode packet ID")?.0;
                        ensure!(id == #packet_id, "unexpected packet ID {} (expected {})", id, #packet_id);

                        Ok(#decode_fields)
                    }
                }

                impl ::statik_common::packet::Packet for #ident {

                    const PACKET_ID: i32 = #packet_id;

                    fn id(&self) -> ::statik_common::varint::VarInt {

                        ::statik_common::varint::VarInt(Self::PACKET_ID)

                    }

                }
            })
        }

        Data::Enum(e) => Err(Error::new(
            e.enum_token.span,
            "cannot derive `Packet` on enum's YET",
        )),
        Data::Union(u) => Err(Error::new(
            u.union_token.span,
            "cannot derive `Packet` on unions",
        )),
    }
}

fn extract_packet_id_attr(attrs: &[Attribute]) -> Result<Option<LitInt>> {
    for attr in attrs {
        if let Meta::NameValue(n) = &attr.meta {
            if n.path.is_ident("packet_id") {
                let span = n.path.segments.first().unwrap().ident.span();

                if let Expr::Lit(l) = &n.value {
                    if let Lit::Int(i) = &l.lit {
                        return Ok(Some(i.clone()));
                    }
                }

                return Err(Error::new(span, "packet ID must be an integer literal"));
            }
        }
    }
    Ok(None)
}
