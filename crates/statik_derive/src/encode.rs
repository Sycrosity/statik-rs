use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, Data, DeriveInput, Error, Fields, LitInt, Result};

pub fn derive_encode(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;

    let ident = input.ident;

    match input.data {
        Data::Struct(s) => {
            let encode_fields = match &s.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident.as_ref().unwrap();
                        let ctx = format!("failed to encode field `{name}` in `{}`", &ident);
                        quote! {
                            self.#name.encode(&mut _buffer).context(#ctx)?;
                        }
                    })
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|i| {
                        let lit = LitInt::new(&i.to_string(), Span::call_site());
                        let ctx = format!("failed to encode field `{lit}` in `{}`", &ident);
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

                        use ::statik_core::packet::Encode;
                        use ::anyhow::{Context, ensure};

                        #encode_fields

                        Ok(())
                    }
                }
            })
        }
        Data::Enum(e) => Err(Error::new(
            e.enum_token.span,
            "cannot derive `Encode` on enums",
        )),
        Data::Union(u) => Err(Error::new(
            u.union_token.span,
            "cannot derive `Encode` on unions",
        )),
    }
}
