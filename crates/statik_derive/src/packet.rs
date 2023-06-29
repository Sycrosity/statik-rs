use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse2, parse_quote, spanned::Spanned, Attribute, Data, DeriveInput, Error, Expr, Fields,
    LitInt, LitStr, Result,
};

pub fn derive_packet(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;

    let ident = input.ident.clone();

    let Some(packet_attr) = parse_packet_helper_attr(&input.attrs)? else {
        return Err(Error::new(input.span(), "missing `packet` attribute"));
    };

    let Some(id) = packet_attr.id else {
        return Err(Error::new(packet_attr.span, "missing `id = ...` value from packet attribute"));
    };

    let state = packet_attr
        .state
        .unwrap_or_else(|| parse_quote!(::statik_core::state::State::Play));

    match input.data {
        Data::Struct(s) => {
            let decode_fields = match &s.fields {
                Fields::Named(fields) => {
                    let init = fields.named.iter().map(|f| {
                        let name = f.ident.as_ref().unwrap();
                        let ctx = format!("failed to decode field `{name}` in `{ident}`");
                        quote! {
                            #name: ::statik_core::prelude::Decode::decode(&mut _buffer).context(#ctx)?,
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
                                ::statik_core::prelude::Decode::decode(&mut _buffer).context(#ctx)?,
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
                impl ::statik_core::packet::Encode for #ident
                {
                    fn encode(&self, mut _buffer: impl ::std::io::Write) -> ::anyhow::Result<()> {

                        use ::statik_core::{packet::Encode, varint::VarInt};
                        use ::anyhow::{Context, ensure};

                        VarInt(#id).encode(&mut _buffer)?;
                        #encode_fields

                        Ok(())
                    }
                }

                #[allow(unused_imports)]
                impl ::statik_core::packet::Decode for #ident
                {
                    fn decode(mut _buffer: impl ::std::io::Read) -> ::anyhow::Result<Self> {

                        use ::statik_core::{packet::Decode, varint::VarInt};
                        use ::anyhow::{Context, ensure};

                        Ok(#decode_fields)
                    }
                }

                impl ::statik_core::packet::Packet for #ident {

                    const ID: i32 = #id;
                    const STATE: ::statik_core::state::State = #state;

                    fn id(&self) -> ::statik_core::varint::VarInt {

                        ::statik_core::varint::VarInt(Self::ID)

                    }

                    fn state(&self) -> ::statik_core::state::State {

                        Self::STATE

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

struct PacketAttr {
    span: Span,
    id: Option<i32>,
    tag: Option<i32>,
    name: Option<LitStr>,
    side: Option<Expr>,
    state: Option<Expr>,
}

fn parse_packet_helper_attr(attrs: &[Attribute]) -> Result<Option<PacketAttr>> {
    for attr in attrs {
        if attr.path().is_ident("packet") {
            let mut res = PacketAttr {
                span: attr.span(),
                id: None,
                tag: None,
                name: None,
                side: None,
                state: None,
            };

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    res.id = Some(meta.value()?.parse::<LitInt>()?.base10_parse::<i32>()?);
                    Ok(())
                } else if meta.path.is_ident("tag") {
                    res.tag = Some(meta.value()?.parse::<LitInt>()?.base10_parse::<i32>()?);
                    Ok(())
                } else if meta.path.is_ident("name") {
                    res.name = Some(meta.value()?.parse::<LitStr>()?);
                    Ok(())
                } else if meta.path.is_ident("side") {
                    res.side = Some(meta.value()?.parse::<Expr>()?);
                    Ok(())
                } else if meta.path.is_ident("state") {
                    res.state = Some(meta.value()?.parse::<Expr>()?);
                    Ok(())
                } else {
                    Err(meta.error("unrecognized packet argument"))
                }
            })?;

            return Ok(Some(res));
        }
    }

    Ok(None)
}
