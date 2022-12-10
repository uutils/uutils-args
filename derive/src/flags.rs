use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
pub(crate) struct Flags {
    pub short: Vec<char>,
    pub long: Vec<String>,
}

impl Flags {
    pub(crate) fn new() -> Self {
        Self {
            short: Vec::new(),
            long: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, flag: &str) {
        assert!(flag.starts_with('-'), "Flags must start with a '-'");
        if let Some(s) = flag.strip_prefix("--") {
            self.long.push(s.to_string());
        } else if let Some(s) = flag.strip_prefix("-") {
            assert_eq!(s.len(), 1);
            self.short.push(s.chars().next().unwrap())
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.short.is_empty() && self.long.is_empty()
    }

    pub(crate) fn short_pat(&self) -> TokenStream {
        let short = &self.short;
        quote!(#(#short)|*)
    }

    pub(crate) fn long_pat(&self) -> TokenStream {
        let long = &self.long;
        quote!(#(#long)|*)
    }

    pub(crate) fn pat(&self) -> TokenStream {
        match (&self.short[..], &self.long[..]) {
            ([], []) => panic!("Creating pattern from empty flags, probably not what you want!"),
            (short, []) => quote!(lexopt::Arg::Short(#(#short)|*)),
            ([], long) => quote!(lexopt::Arg::Long(#(#long)|*)),
            (short, long) => {
                quote!(lexopt::Arg::Short(#(#short)|*) | lexopt::Arg::Long(#(#long)|*))
            }
        }
    }

    pub(crate) fn default_help() -> Self {
        Self {
            short: vec!['h'],
            long: vec!["help".into()],
        }
    }

    pub(crate) fn default_version() -> Self {
        Self {
            short: vec!['V'],
            long: vec!["version".into()],
        }
    }

    pub(crate) fn format(&self) -> String {
        self.short
            .iter()
            .map(|s| format!("-{s}"))
            .chain(self.long.iter().map(|l| format!("--{l}")))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn or_from_name(self, name: &str) -> Self {
        if self.is_empty() {
            let name = name.to_lowercase();
            let first_char = name.chars().next().unwrap();
            if name.len() > 1 {
                Self {
                    short: vec![first_char],
                    long: vec![name.to_string()],
                }
            } else {
                Self {
                    short: vec![first_char],
                    long: vec![],
                }
            }
        } else {
            self
        }
    }
}
