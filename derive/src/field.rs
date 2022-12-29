use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Field, Ident};

use crate::{
    action::{parse_action_attr, ActionAttr, ActionType},
    attributes::FieldAttr,
};

pub(crate) struct FieldData {
    pub(crate) ident: Ident,
    pub(crate) default_value: TokenStream,
    pub(crate) match_stmt: TokenStream,
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
            match ::std::env::var_os(#env_var) {
                Some(x) => ::uutils_args::FromValue::from_value("", x)?,
                None => #default_value
            }
        )
    }

    let match_arms = field
        .attrs
        .iter()
        .filter_map(parse_action_attr)
        .flat_map(|attr| action_attr_to_match_arms(&field_ident, attr));

    let match_stmt = quote!(match arg.clone() {
        #(#match_arms)*,
        _ => {}
    });

    FieldData {
        ident: field_ident,
        default_value,
        match_stmt,
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

fn action_attr_to_match_arms(field_ident: &Ident, attr: ActionAttr) -> Vec<TokenStream> {
    let mut match_arms = Vec::new();
    match attr.action_type {
        ActionType::Map(arms) => {
            for arm in arms {
                match_arms.push(field_expression(
                    arm.pat.to_token_stream(),
                    arm.body.to_token_stream(),
                    field_ident,
                    attr.collect,
                ));
            }
        }

        ActionType::Set(pats) => {
            let pats: Vec<_> = pats.iter().map(|p| quote!(#p(x))).collect();
            match_arms.push(field_expression(
                quote!(#(#pats)|*),
                quote!(x),
                field_ident,
                attr.collect,
            ));
        }
    };
    match_arms
}

fn field_expression(
    pat: TokenStream,
    expr: TokenStream,
    field_ident: &Ident,
    collect: bool,
) -> TokenStream {
    if collect {
        quote!(
            #pat => { self.#field_ident.push(#expr) }
        )
    } else {
        quote!(
            #pat => { self.#field_ident = #expr }
        )
    }
}
