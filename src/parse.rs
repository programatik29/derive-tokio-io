use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::fmt::Display;
use syn::{DeriveInput, Index};

pub(crate) struct ParsedData {
    pub(crate) ident: TokenStream,
    pub(crate) generics: TokenStream,
    pub(crate) field_id: TokenStream,
    pub(crate) field_ty: TokenStream,
}

pub(crate) fn parse_derive_input(input: DeriveInput, attr_name: &str) -> syn::Result<ParsedData> {
    let ident = &input.ident;
    let generics = &input.generics;
    let data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => {
            return Err(error("this macro can only be used for structs"));
        }
    };

    if data.fields.is_empty() {
        return Err(error("struct must have fields"));
    }

    let (i, field) = if data.fields.len() == 1 {
        (Index::from(0), data.fields.iter().next().unwrap())
    } else {
        let mut iter = data.fields.iter().enumerate().filter_map(|(i, f)| {
            let attr = f.attrs.iter().find(|a| a.path.is_ident(attr_name))?;

            Some((i, f, attr))
        });

        let (i, field, _attr) = iter.next().ok_or_else(|| {
            error(format!(
                "#[{}] on one field is required for structs with multiple fields",
                attr_name
            ))
        })?;

        if let Some((_i, _field, attr)) = iter.next() {
            return Err(syn::Error::new_spanned(
                attr,
                format!("only one #[{}] can be used", attr_name),
            ));
        }

        (Index::from(i), field)
    };

    let field_id = field
        .ident
        .as_ref()
        .map(ToTokens::to_token_stream)
        .unwrap_or_else(|| i.to_token_stream());

    let field_ty = &field.ty;

    Ok(ParsedData {
        ident: ident.to_token_stream(),
        generics: generics.to_token_stream(),
        field_id: field_id.to_token_stream(),
        field_ty: field_ty.to_token_stream(),
    })
}

fn error(s: impl Display) -> syn::Error {
    syn::Error::new(Span::call_site(), s)
}
