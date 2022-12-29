use syn::{
    parenthesized,
    parse::{Nothing, Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Token,
};

pub(crate) struct ActionAttr {
    pub(crate) action_type: ActionType,
    pub(crate) collect: bool,
}

pub(crate) enum ActionType {
    Set(Vec<syn::Path>),
    Map(Vec<syn::Arm>),
}

fn parse_paths(attr: &Attribute) -> Vec<syn::Path> {
    attr.parse_args_with(Punctuated::<syn::Path, Token![|]>::parse_terminated)
        .into_iter()
        .flatten()
        .collect()
}

pub(crate) fn parse_action_attr(attr: &Attribute) -> Option<ActionAttr> {
    if attr.path.is_ident("collect") {
        let inner: ActionType = attr.parse_args().unwrap();
        Some(ActionAttr {
            action_type: inner,
            collect: true,
        })
    } else if attr.path.is_ident("map") {
        Some(ActionAttr {
            action_type: ActionType::Map(
                attr.parse_args_with(Punctuated::<syn::Arm, Nothing>::parse_terminated)
                    .into_iter()
                    .flatten()
                    .collect(),
            ),
            collect: false,
        })
    } else if attr.path.is_ident("set") {
        Some(ActionAttr {
            action_type: ActionType::Set(parse_paths(attr)),
            collect: false,
        })
    } else {
        None
    }
}

impl Parse for ActionType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let action: syn::Ident = input.parse()?;
        let action = action.to_string();
        let content;
        parenthesized!(content in input);
        if action == "map" {
            let arms = content.call(Punctuated::<syn::Arm, Nothing>::parse_terminated)?;
            Ok(ActionType::Map(arms.into_iter().collect()))
        } else {
            let pat = content.call(Punctuated::<syn::Path, Token![|]>::parse_terminated)?;
            let pat = pat.into_iter().collect();
            match &action[..] {
                "set" => Ok(ActionType::Set(pat)),
                _ => panic!("Unexpected action type in collect {}", action),
            }
        }
    }
}
