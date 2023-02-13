use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Field, Ident};

use crate::attributes::FieldAttr;

pub(crate) struct FieldData {
    pub(crate) ident: Ident,
    pub(crate) default_value: TokenStream,
}

pub(crate) fn parse_field(field: &Field) -> FieldData {
    let field_ident = field.ident.as_ref().unwrap().clone();

    let field_attr = parse_field_attr(&field.attrs);

    let mut default_value = match field_attr.default {
        Some(val) => val.to_token_stream(),
        None => quote!(::core::default::Default::default()),
    };

    if let Some(env_var) = field_attr.env {
        default_value = quote!(
            ::std::env::var_os(#env_var)
                .and_then(|v| ::uutils_args::FromValue::from_value("", v).ok())
                .unwrap_or(#default_value)
        )
    }

    FieldData {
        ident: field_ident,
        default_value,
    }
}

pub(crate) fn parse_field_attr(attrs: &[Attribute]) -> FieldAttr {
    for attr in attrs {
        if attr.path.is_ident("field") {
            return FieldAttr::parse(attr);
        }
    }
    FieldAttr::default()
}
