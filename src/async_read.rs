use crate::parse::{parse_derive_input, ParsedData};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn expand_async_read(input: DeriveInput) -> syn::Result<TokenStream> {
    let ParsedData {
        ident,
        generics,
        field_ty,
        field_id,
    } = parse_derive_input(input, "async_read")?;

    let token_stream = quote! {
        #[automatically_derived]
        impl #generics ::tokio::io::AsyncRead for #ident #generics
        where
            #field_ty: ::tokio::io::AsyncRead,
        {
            fn poll_read(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &mut ::tokio::io::ReadBuf<'_>,
            ) -> ::std::task::Poll<std::io::Result<()>> {
                unsafe {
                    ::std::pin::Pin::new_unchecked(&mut self.get_unchecked_mut().#field_id).poll_read(cx, buf)
                }
            }
        }
    };

    Ok(token_stream)
}
