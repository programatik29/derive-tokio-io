use crate::parse::{parse_derive_input, ParsedData};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn expand_async_write(input: DeriveInput) -> syn::Result<TokenStream> {
    let ParsedData {
        ident,
        generics,
        field_ty,
        field_id,
    } = parse_derive_input(input, "async_write")?;

    let token_stream = quote! {
        #[automatically_derived]
        impl #generics ::tokio::io::AsyncWrite for #ident #generics
        where
            #field_ty: ::tokio::io::AsyncWrite,
        {
            fn poll_write(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &[u8],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                unsafe {
                    ::std::pin::Pin::new_unchecked(&mut self.get_unchecked_mut().#field_id).poll_write(cx, buf)
                }
            }

            fn poll_flush(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                unsafe { ::std::pin::Pin::new_unchecked(&mut self.get_unchecked_mut().#field_id).poll_flush(cx) }
            }

            fn poll_shutdown(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                unsafe { ::std::pin::Pin::new_unchecked(&mut self.get_unchecked_mut().#field_id).poll_shutdown(cx) }
            }
        }
    };

    Ok(token_stream)
}
