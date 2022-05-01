#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]
#![no_std]
#![forbid(unsafe_code)]

#[cfg(COMMENTED_OUT)] // <- Remove this when used!
/// The crate's prelude.
pub
mod prelude {
    // …
}

// macro internals
#[doc(hidden)] /** Not part of the public API */ pub
mod __ {
    pub use ::core; // or `std`
}

#[cfg_attr(feature = "ui-tests",
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod _compile_fail_tests {}
