use std::{
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    argument::{ArgType, Argument},
    attributes::{HelpAttr, VersionAttr},
    flags::Flags,
    markdown::{get_after_event, get_h2, str_to_renderer},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Attribute;

pub(crate) fn parse_help_attr(attrs: &[Attribute]) -> HelpAttr {
    for attr in attrs {
        if attr.path.is_ident("help") {
            return HelpAttr::parse(attr);
        }
    }
    HelpAttr::default()
}

pub(crate) fn parse_version_attr(attrs: &[Attribute]) -> VersionAttr {
    for attr in attrs {
        if attr.path.is_ident("version") {
            return VersionAttr::parse(attr);
        }
    }
    VersionAttr::default()
}

pub(crate) fn help_handling(help_flags: &Flags) -> TokenStream {
    if help_flags.is_empty() {
        return quote!();
    }

    let pat = help_flags.pat();

    quote!(
        if let #pat = arg {
            return Ok(Some(Argument::Help));
        }
    )
}

pub(crate) fn help_string(
    args: &[Argument],
    help_attr: &HelpAttr,
    version_flags: &Flags,
) -> TokenStream {
    let mut options = Vec::new();

    let width: usize = 16;
    let indent: usize = 2;

    for Argument { arg_type, help, .. } in args {
        match arg_type {
            ArgType::Option { flags, .. } => {
                let flags = flags.format();
                let renderer = str_to_renderer(help);
                options.push(quote!((#flags, #renderer)));
            }
            ArgType::Positional { .. } => {}
        }
    }

    let (summary, after_options) = if let Some(file) = &help_attr.file {
        let (summary, after_options) = read_help_file(file);
        (
            quote!(s.push_str(&#summary.render());),
            quote!(
                s.push('\n');
                s.push_str(&#after_options.render());
            ),
        )
    } else {
        (quote!(), quote!())
    };

    if !help_attr.flags.is_empty() {
        let flags = help_attr.flags.format();
        let renderer = str_to_renderer("Display this help message");
        options.push(quote!((#flags, #renderer)));
    }

    if !version_flags.is_empty() {
        let flags = version_flags.format();
        let renderer = str_to_renderer("Display version information");
        options.push(quote!((#flags, #renderer)));
    }

    let options = if !options.is_empty() {
        let options = quote!([#(#options),*]);
        quote!(
            s.push_str("\nOptions:\n");
            for (flags, renderer) in #options {
                let indent = " ".repeat(#indent);

                let help_string = renderer.render();
                let mut help_lines = help_string.lines();
                s.push_str(&indent);
                s.push_str(&flags);

                if flags.len() <= #width {
                    let line = match help_lines.next() {
                        Some(line) => line,
                        None => return s,
                    };
                    let help_indent = " ".repeat(#width-flags.len()+2);
                    s.push_str(&help_indent);
                    s.push_str(line);
                    s.push('\n');
                } else {
                    s.push('\n');
                }

                let help_indent = " ".repeat(#width+#indent+2);
                for line in help_lines {
                    s.push_str(&help_indent);
                    s.push_str(line);
                    s.push('\n');
                }
            }
        )
    } else {
        quote!()
    };

    quote!(
        let mut s = String::new();

        s.push_str(&format!("{} {}\n",
            option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
            env!("CARGO_PKG_VERSION"),
        ));

        #summary

        s.push_str(&format!("\nUsage:\n  {} [OPTIONS] [ARGS]\n", bin_name));

        #options

        #after_options

        s
    )
}

fn read_help_file(file: &str) -> (TokenStream, TokenStream) {
    let path = Path::new(file);
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut location = PathBuf::from(manifest_dir);
    location.push(path);
    let mut contents = String::new();
    let mut f = std::fs::File::open(location).unwrap();
    f.read_to_string(&mut contents).unwrap();

    (
        get_h2("summary", &contents),
        get_after_event(pulldown_cmark::Event::Rule, &contents),
    )
}
