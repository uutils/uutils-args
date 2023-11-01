use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
pub(crate) struct Flags {
    pub short: Vec<Flag<char>>,
    pub long: Vec<Flag<String>>,
    pub dd_style: Vec<(String, String)>,
}

#[derive(Clone)]
pub(crate) enum Value {
    No,
    Optional(String),
    Required(String),
}

#[derive(Clone)]
pub(crate) struct Flag<T> {
    pub(crate) flag: T,
    pub(crate) value: Value,
}

impl Flags {
    pub(crate) fn new<T: AsRef<str>>(flags: impl IntoIterator<Item = T>) -> Self {
        let mut self_ = Self::default();
        for flag in flags {
            self_.add(flag.as_ref());
        }
        self_
    }

    pub(crate) fn add(&mut self, flag: &str) {
        if let Some(s) = flag.strip_prefix("--") {
            // There are three possible patterns:
            //   --flag
            //   --flag=value
            //   --flag[=value]

            // First we trim up to the = or [
            let mut chars = s.chars();
            let mut sep = '-';
            let f: String = (&mut chars)
                .take_while(|&c: &char| {
                    sep = c;
                    c.is_alphanumeric() || c == '-'
                })
                .collect();
            let val: String = chars.collect();

            // Now check the cases:
            let value = if val.is_empty() {
                Value::No
            } else if sep == '=' {
                assert!(val.chars().all(|c: char| c.is_alphanumeric() || c == '-'));
                Value::Required(val)
            } else if sep == '[' {
                let optional = val
                    .strip_prefix('=')
                    .and_then(|s| s.strip_suffix(']'))
                    .unwrap();
                assert!(optional
                    .chars()
                    .all(|c: char| c.is_alphanumeric() || c == '-'));
                Value::Optional(optional.into())
            } else {
                panic!("Invalid long flag '{flag}'");
            };

            self.long.push(Flag { flag: f, value });
        } else if let Some(s) = flag.strip_prefix('-') {
            assert!(!s.is_empty());

            // There are three possible patterns:
            //   -f
            //   -f value
            //   -f[value]

            // First we trim up to the = or [
            let mut chars = s.chars();
            let f = chars.next().unwrap();
            let val: String = chars.collect();

            // Now check the cases:
            let value = if val.is_empty() {
                Value::No
            } else if let Some(optional) = val.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                assert!(optional
                    .chars()
                    .all(|c: char| c.is_alphanumeric() || c == '-'));
                Value::Optional(optional.into())
            } else if let Some(required) = val.strip_prefix(' ') {
                assert!(required
                    .chars()
                    .all(|c: char| c.is_alphanumeric() || c == '-'));
                Value::Required(required.into())
            } else {
                panic!("Invalid short flag '{flag}'")
            };
            self.short.push(Flag { flag: f, value });
        } else if let Some((s, v)) = flag.split_once('=') {
            // It's a dd-style argument: arg=value
            assert!(!s.is_empty());
            assert!(!v.is_empty());

            self.dd_style.push((s.into(), v.into()));
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.short.is_empty() && self.long.is_empty() && self.dd_style.is_empty()
    }

    pub(crate) fn pat(&self) -> TokenStream {
        let short: Vec<_> = self.short.iter().map(|f| f.flag).collect();
        let long: Vec<_> = self.long.iter().map(|f| &f.flag).collect();
        match (&short[..], &long[..]) {
            ([], []) => panic!("Creating pattern from empty flags, probably not what you want!"),
            (short, []) => quote!(lexopt::Arg::Short(#(#short)|*)),
            ([], long) => quote!(lexopt::Arg::Long(#(#long)|*)),
            (short, long) => {
                quote!(lexopt::Arg::Short(#(#short)|*) | lexopt::Arg::Long(#(#long)|*))
            }
        }
    }

    pub(crate) fn format(&self) -> String {
        let short = self
            .short
            .iter()
            .map(|f| {
                let s = &f.flag;
                match &f.value {
                    Value::No => format!("-{s}"),
                    Value::Optional(v) => format!("-{s}[{v}]"),
                    Value::Required(v) => format!("-{s} {v}"),
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        let long = self
            .long
            .iter()
            .map(|f| {
                let l = &f.flag;
                match &f.value {
                    Value::No => format!("--{l}"),
                    Value::Optional(v) => format!("--{l}[={v}]"),
                    Value::Required(v) => format!("--{l}={v}"),
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        if short.is_empty() {
            format!("    {long}")
        } else if long.is_empty() {
            short
        } else {
            format!("{short}, {long}")
        }
    }
}
