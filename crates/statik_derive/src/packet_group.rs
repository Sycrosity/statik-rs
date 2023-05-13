use proc_macro2::{Span, TokenStream};
use syn::{Attribute, Data, DeriveInput, Error, Expr, Fields, Lit, LitInt, Meta, Result, Variant};

pub fn expand_derive_packet_group(input: &mut DeriveInput) -> Result<TokenStream> {
    let DeriveInput {
        // attrs: _attrs,
        // vis: _vis,
        ident: input_name,
        // generics: _generics,
        data,
        ..
    } = input;

    // let attrs =

    match data {
        Data::Struct(s) => Err(Error::new(
            s.struct_token.span,
            "cannot derive `Packet` on structs YET",
        )),
        Data::Enum(e) => {
            let encode_arms = e
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = variant.ident;

                    let ctx = format!(
                        "failed to encode enum variant `{variant_name}` \
                         in `{ident}`",
                    );

                    match &variant.fields {
                        Fields::Named(fields) => {
                            let field_names = fields
                                .named
                                .iter()
                                .map(|f| f.ident.as_ref().unwrap())
                                .collect::<Vec<_>>();

                            let encode_fields = field_names
                                .iter()
                                .map(|name| {
                                    let ctx = format!(
                                        "failed to encode field `{name}` in variant \
                                         `{variant_name}` in `{input_name}`",
                                    );

                                    quote! {
                                        #name.encode(&mut _w).context(#ctx)?;
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                Self::#variant_name { #(#field_names,)* } => {
                                    VarInt(#disc).encode(&mut _buffer).context(#ctx)?;

                                    #encode_fields
                                    Ok(())
                                }
                            }
                        }
                        Fields::Unnamed(fields) => {
                            let field_names = (0..fields.unnamed.len())
                                .map(|i| Ident::new(&format!("_{i}"), Span::call_site()))
                                .collect::<Vec<_>>();

                            let encode_fields = field_names
                                .iter()
                                .map(|name| {
                                    let ctx = format!(
                                        "failed to encode field `{name}` in variant \
                                         `{variant_name}` in `{input_name}`"
                                    );

                                    quote! {
                                        #name.encode(&mut _w).context(#ctx)?;
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                Self::#variant_name(#(#field_names,)*) => {
                                    VarInt(#disc).encode(&mut _w).context(#disc_ctx)?;

                                    #encode_fields
                                    Ok(())
                                }
                            }
                        }
                        Fields::Unit => quote! {
                            Self::#variant_name => Ok(
                                VarInt(#disc)
                                    .encode(&mut _w)
                                    .context(#disc_ctx)?
                            ),
                        },
                    }
                })
                .collect::<TokenStream>();

            Ok(quote! {
                #[allow(unused_imports, unreachable_code)]
                impl #impl_generics ::valence_core::__private::Encode for #input_name #ty_generics
                #where_clause
                {
                    fn encode(&self, mut _w: impl ::std::io::Write) -> ::valence_core::__private::Result<()> {
                        use ::valence_core::__private::{Encode, VarInt, Context};

                        match self {
                            #encode_arms
                            _ => unreachable!(),
                        }
                    }
                }
            })
        }
        Data::Union(u) => Err(Error::new(
            u.union_token.span,
            "cannot derive `Packet` on unions",
        )),
    }
}

fn extract_id_attr(attrs: &[Attribute]) -> Result<Option<LitInt>> {
    for attr in attrs {
        if let Meta::NameValue(n) = &attr.meta {
            if n.path.is_ident("id") {
                let span = n.path.segments.first().unwrap().ident.span();

                if let Expr::Lit(l) = &n.value {
                    match &l.lit {
                        Lit::Int(i) => return Ok(Some(i.clone())),
                        _ => (),
                    }
                }

                return Err(Error::new(span, "packet ID must be an integer literal"));
            }
        }
    }
    Ok(None)
}
