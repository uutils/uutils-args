use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data::Struct, DeriveInput, Fields};

// FIXME: Think of a better name
#[proc_macro_derive(Options, attributes(flag))]
pub fn options(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Struct(data) = input.data else {
        panic!("Input should be a struct!");
    };

    let Fields::Named(fields) = data.fields else {
        panic!("Fields must be named");
    };

    // The key of this map is a literal pattern and the value
    // is whatever code needs to be run when that pattern is encountered.
    let mut map: HashMap<String, Vec<TokenStream2>> = HashMap::new();

    for field in fields.named {
        let field_ident = field.ident.as_ref().expect("Each field must be named.");
        let field_name = field_ident.to_string();
        for attr in field.attrs {
            if attr.path.is_ident("flag") {
                map.entry(field_name.clone())
                    .or_default()
                    .push(quote!(_self.#field_ident = true;));
            }
        }
    }

    let mut match_arms = vec![];
    for (pattern, arms) in map {
        match_arms.push(quote!(#pattern => {#(#arms)*}))
    }

    let expanded = quote!(
        impl #impl_generics Options for #name #ty_generics #where_clause {
            fn parse(args: &[&str]) -> #name {
                let mut _self = #name::default();
                for arg in args {
                    match *arg {
                        #(#match_arms)*
                        _ => panic!("Unrecognized argument {}", arg)
                    }
                }
                _self
            }
        }
    );

    TokenStream::from(expanded)
}
