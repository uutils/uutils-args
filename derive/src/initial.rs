use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Fields, Token,
};

use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, Expr, LitStr};

mod kw {
    syn::custom_keyword!(env);
}

enum InitialArg {
    Expr(Expr),
    Env(String),
}

#[derive(Default)]
struct InitialField {
    expr: Option<syn::Expr>,
    env: Option<String>,
}

impl Parse for InitialArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::env) && input.peek2(Token![=]) {
            input.parse::<kw::env>()?;
            input.parse::<Token![=]>()?;
            Ok(InitialArg::Env(input.parse::<LitStr>()?.value()))
        } else {
            Ok(InitialArg::Expr(input.parse::<Expr>()?))
        }
    }
}

impl InitialField {
    fn from_attribute(attribute: &Attribute) -> syn::Result<Self> {
        let mut _self = Self::default();

        let args =
            attribute.parse_args_with(Punctuated::<InitialArg, Token![,]>::parse_terminated)?;

        for arg in args {
            match arg {
                InitialArg::Expr(e) => {
                    if _self.expr.is_some() {
                        panic!("Can only specify one initial expression")
                    }
                    _self.expr = Some(e);
                }
                InitialArg::Env(s) => {
                    if _self.expr.is_some() {
                        panic!("Can only specify one env variable")
                    }
                    _self.env = Some(s);
                }
            }
        }

        Ok(_self)
    }

    fn into_expr(self) -> proc_macro2::TokenStream {
        let mut default_value = match self.expr {
            Some(val) => quote!(#val.into()),
            None => quote!(::core::default::Default::default()),
        };

        if let Some(env_var) = self.env {
            default_value = quote!(
                ::std::env::var_os(#env_var)
                    .and_then(|v| ::uutils_args::Value::from_value(&v).ok())
                    .unwrap_or(#default_value)
            );
        }

        default_value
    }
}

pub fn initial(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let function_body = match input.data {
        Data::Struct(data) => initial_struct(data),
        _ => panic!("Initial derive macro can only be used on structs"),
    };

    quote!(
        impl #impl_generics Initial for #name #ty_generics #where_clause {
            fn initial() -> Self {
                #function_body
            }
        }
    )
    .into()
}

fn initial_struct(data: syn::DataStruct) -> proc_macro2::TokenStream {
    let Fields::Named(fields) = data.fields else {
        panic!("Fields must be named");
    };

    // The key of this map is a literal pattern and the value
    // is whatever code needs to be run when that pattern is encountered.
    let mut defaults = Vec::new();
    for field in fields.named {
        let ident = field.ident;
        let field = parse_field_attr(&field.attrs);
        let default_value = field.into_expr();

        defaults.push(quote!(#ident: #default_value));
    }

    quote!(
        Self {
            #(#defaults),*
        }
    )
}

fn parse_field_attr(attrs: &[Attribute]) -> InitialField {
    for attr in attrs {
        if attr.path().is_ident("initial") {
            return InitialField::from_attribute(attr).expect("Failed to parse initial attribute");
        }
    }
    InitialField::default()
}
