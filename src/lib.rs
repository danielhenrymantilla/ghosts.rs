#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "better-docs", feature(
    doc_cfg, rustdoc_internals,
))]
#![doc(html_logo_url = "https://user-images.githubusercontent.com/9920355/166294360-9ee4ec3d-685c-49e5-b006-1e9077f8cc01.png")]
#![deny(rustdoc::private_intra_doc_links)]

#![no_std]
#![forbid(unsafe_code)]
#![allow(uncommon_codepoints)]

/// The crate's _haunted_ prelude.
pub
mod vestibule {
    #[doc(no_inline)]
    pub use {
        crate::{
            Ectoplasm,
            ectoplasm,
            Ghost,
            ghost,
            materialize_return,
            // materialize,
        },
    };
}

#[doc(cfg(ghostǃ))]
#[cfg(feature = "better-docs")]
#[doc(keyword = "no_init")]
/// Use `#[no_init]` inside a [`ghost!`] block to opt out of consuming
/// ownership of outside captures.
///
/** ```rust
use ::ghosts::vestibule::*;

let owned = String::from("…");
let casper = ghost!(#[no_init] {
    owned
});
drop(owned); // Ok
``` */
mod ghost_tag {}

#[doc(cfg(ghostǃ))]
#[cfg(feature = "better-docs")]
#[doc(keyword = "tag")]
/// Use `#[tag]` to accept **and discard** extra attribute metadata.
///
/// May be useful for [tool attributes](
/// https://doc.rust-lang.org/1.60.0/reference/attributes.html#tool-attributes).
///
/** ```rust
use ::ghosts::vestibule::*;

let casper = ghost!(#[tag(my_tool::my_annotation)] {
    drop("this is fine");
});
``` */
mod ghost_no_init {}

pub use expr::*;
mod expr;

// We use this rather than a module so that the methods do show up on docs.rs
// include!("ty/fns.rs");

// macro internals
#[doc(hidden)] /** Not part of the public API */ pub
mod ඞ {
    pub use {
        ::core, // or `std`
        ::ghosts_proc_macros::{
            *,
        },
        crate::{
            expr::flatten::Flatten,
        },
    };
    pub use crate::ty::ghost::Ghost;
}

#[cfg_attr(feature = "ui-tests",
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod _compile_fail_tests {}

#[doc(inline)]
pub use ty::Ghost;
mod ty;
