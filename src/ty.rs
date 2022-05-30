//! Custom `PhantomData`: generic "unit-type".
//!
//! Pattern taken from <https://docs.rs/ghost> & their case-studies post.
//!
//! It would have been incredibly fitting to have `ghost-exprs` depend on the
//! `ghost` crate; but pulling in `syn` with `default` features is way too steep
//! a cost.
//!
//! Actually, I've tweaked the pattern a bit: I'm using a non-`PhantomData`
//! payload by default, which is then said to be of the `PhantomData` form
//! through the type alias. This ensures the std-lib `derives` behave in a
//! smarter way, deducing all the right bounds and impls for the derive off
//! those of `PhantomData`.

use crate::vestibule::*;

pub use ghost::*; // value-namespace `Ghost`
pub mod ghost {
    #![allow(unreachable_code)]

    #[derive(
        Debug,
        Clone, Copy,
        Eq, PartialEq, Ord, PartialOrd,
        Hash,
    )]
    pub
    enum Ghost<T> {
        Ghost,

        #[allow(dead_code)]
        #[doc(hidden)]
        __(::never_say_never::Never, T),
    }

    pub use self::Ghost::*;
}

/// The type of [`ghost!`] expressions.
///
/// Guaranteed to be a zero-cost ZST token you can pass around in functions and
/// embed in data structures with no runtime impact whatsoever.
///
/// Similar to a `PhantomData`, it can be directly instanced as a unit struct.
///
/**  - ```rust
    use ::ghosts::Ghost;

    let _: Ghost<()> = Ghost;
    ``` */
pub
type Ghost<T /* : ?Sized */> = // type-namespace `Ghost`.
    ghost::Ghost<::core::marker::PhantomData<T>>
;

impl<T> Ghost<T> {
    /// Nudge type-inference.
    #[doc(hidden)] pub
    const
    fn __set (self: &'_ Ghost<T>, value: T)
    {
        ::core::mem::forget(value);
    }
}

impl<T : ?Sized> Default
    for Ghost<T>
{
    #[inline]
    fn default ()
      -> Self
    {
        Ghost
    }
}

impl<T : ?Sized> Ghost<T> {
    #[inline]
    pub
    fn map<U> (
        self: Ghost<T>,
        f: Ghost<impl FnOnce(T, Ectoplasm) -> U>,
    ) -> Ghost<U>
    where
        T : Sized,
    {
        ghost!(
            materialize!(f)(materialize!(self), materialize!(Ghost))
        )
    }

    #[inline]
    pub
    fn and_then<U> (
        self: Ghost<T>,
        f: Ghost<impl FnOnce(T, Ectoplasm) -> Ghost<U>>,
    ) -> Ghost<U>
    where
        T : Sized,
    {
        ghost!(
            materialize!(f)(materialize!(self), materialize!(Ghost))
        ) // auto-squashed ectoplasm.
    }
}
