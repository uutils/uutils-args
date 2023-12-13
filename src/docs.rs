//! This module contains only documentation to be rendered by rustdoc.
//!
//! - [Guide](guide): the guide for using this library
//! - [Design](design): documents about the design of this library

#[doc = include_str!("../docs/guide/guide.md")]
pub mod guide {
    #[doc = include_str!("../docs/guide/quick.md")]
    pub mod quick {}
    #[doc = include_str!("../docs/guide/port.md")]
    pub mod port {}
    #[doc = include_str!("../docs/guide/completions.md")]
    pub mod completions {}
    #[doc = include_str!("../docs/guide/value.md")]
    pub mod value {}
}

#[doc = include_str!("../docs/design/design.md")]
pub mod design {
    #[doc = include_str!("../docs/design/arguments_in_coreutils.md")]
    pub mod coreutils {}
    #[doc = include_str!("../docs/design/problems_with_clap.md")]
    pub mod problems {}
}
