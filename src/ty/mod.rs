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

/** ```rust
use ::ghosts::Ghost;

let _: Ghost<()> = Ghost;
``` */
// type-namespace `Ghost`.
pub type Ghost<T /* : ?Sized */> = ghost::Ghost<::core::marker::PhantomData<T>>;

pub use ghost::* /* value-namespace `Ghost` */;
pub mod ghost {
    #![allow(unreachable_code)]

    pub use self::Ghost::*;

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
}

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
        f: Ghost<impl FnOnce(T, GhostContext) -> U>,
    ) -> Ghost<U>
    where
        T : Sized,
    {
        ghost!({
            raise!(f)(raise!(self), raise!(Ghost))
        })
    }

    #[inline]
    pub
    fn and_then<U> (
        self: Ghost<T>,
        f: Ghost<impl FnOnce(T, GhostContext) -> Ghost<U>>,
    ) -> Ghost<U>
    where
        T : Sized,
    {
        ghost!({
            raise!(f)(raise!(self), raise!(Ghost))
        }) // auto-squashed ectoplasm.
    }
}
