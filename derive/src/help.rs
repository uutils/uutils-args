use std::{
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    argument::{ArgType, Argument},
    flags::Flags,
    help_parser::{parse_about, parse_section, parse_usage},
};
use proc_macro2::TokenStream;
use quote::quote;

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
    help_flags: &Flags,
    version_flags: &Flags,
    file: &Option<String>,
) -> TokenStream {
    let mut options = Vec::new();

    let width: usize = 16;
    let indent: usize = 2;

    for Argument { arg_type, help, .. } in args {
        match arg_type {
            ArgType::Option {
                flags,
                hidden: false,
                ..
            } => {
                let flags = flags.format();
                options.push(quote!((#flags, #help)));
            }
            // Hidden arguments should not show up in --help
            ArgType::Option { hidden: true, .. } => {}
            ArgType::Positional { .. } => {}
        }
    }

    // FIXME: We need to get an option per item and provide proper defaults
    let (summary, usage, after_options) = if let Some(file) = file {
        read_help_file(file)
    } else {
        ("".into(), "{} [OPTIONS] [ARGUMENTS]".into(), "".into())
    };

    if !help_flags.is_empty() {
        let flags = help_flags.format();
        options.push(quote!((#flags, "Display this help message")));
    }

    if !version_flags.is_empty() {
        let flags = version_flags.format();
        options.push(quote!((#flags, "Display version information")));
    }

    let options = if !options.is_empty() {
        let options = quote!([#(#options),*]);
        quote!(
            writeln!(w, "\nOptions:")?;
            for (flags, help_string) in #options {
                let indent = " ".repeat(#indent);

                let mut help_lines = help_string.lines();
                write!(w, "{}", &indent)?;
                write!(w, "{}", &flags)?;

                if flags.len() <= #width {
                    let line = match help_lines.next() {
                        Some(line) => line,
                        None => {
                            writeln!(w)?;
                            continue;
                        },
                    };
                    let help_indent = " ".repeat(#width-flags.len()+2);
                    writeln!(w, "{}{}", help_indent, line)?;
                } else {
                    writeln!(w, "\n")?;
                }

                let help_indent = " ".repeat(#width+#indent+2);
                for line in help_lines {
                    writeln!(w, "{}{}", help_indent, line)?;
                }
            }
        )
    } else {
        quote!()
    };

    quote!(
        let mut w = ::std::io::stdout();
        use ::std::io::Write;
        writeln!(w, "{} {}",
            option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
            env!("CARGO_PKG_VERSION"),
        )?;

        writeln!(w, "{}", #summary)?;

        writeln!(w, "\nUsage:\n  {}", format!(#usage, bin_name))?;

        #options

        writeln!(w, "{}", #after_options)?;
        Ok(())
    )
}

fn read_help_file(file: &str) -> (String, String, String) {
    let path = Path::new(file);
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut location = PathBuf::from(manifest_dir);
    location.push(path);
    let mut contents = String::new();
    let mut f = std::fs::File::open(location).unwrap();
    f.read_to_string(&mut contents).unwrap();

    (
        parse_about(&contents),
        parse_usage(&contents),
        parse_section("after help", &contents).unwrap_or_default(),
    )
}

pub(crate) fn version_handling(version_flags: &Flags) -> TokenStream {
    if version_flags.is_empty() {
        return quote!();
    }

    let pat = version_flags.pat();

    quote!(
        if let #pat = arg {
            return Ok(Some(Argument::Version));
        }
    )
}
