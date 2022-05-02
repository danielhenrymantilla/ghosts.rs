use crate::vestibule::*;

/// Ghost operation: extract an imaginary `T` value out of a `Ghost<T>` token.
///
/// ## THIS MUST ONLY BE USED FROM WITHIN A `ghost!` BLOCK
///
/// Or somewhere transitively called by a `ghost!` block exclusively.
///
/// Otherwise this call would try to make it to codegen, which it cannot do.
///
///   - the current implementation uses a polymorphization limitation to prevent
///     this, which works, but is not detected by `cargo check`.
///
/// # Non-error-prone replacements
///
///   - When directly inside a [`ghost!`] block, use [`raise!`].
///
///   - Otherwise, require that the caller pass a [`GhostContext`] token,
///     which they can have by [`raise()`]-ing their own
///     <code>[Ghost]&lt;[GhostContext]&gt;</code>.
///
///     With it, you'll have access to its impossible to misuse [`.raise()`][
///     GhostContext::raise] method.
///
///   - Or perhaps way more simply, consider using the [`.map()`][Ghost::map] or
///     [`.and_then()`][Ghost::and_then] adapters.
pub
fn raise<T> (
    _: Ghost<T>,
) -> T
{
    #![allow(unconditional_recursion)]
    struct RaiseOutsideOfAGhostBlockError(u8);
    raise::<(T, RaiseOutsideOfAGhostBlockError)>(Ghost).0
}

pub use __::GhostContext;
#[allow(unreachable_code)]
mod __ {
    #[allow(unused)]
    use super::*;

    /// Unforgeable token which can thus only be constructed by [`raise!`]-ing
    /// a <code>[Ghost]&lt;[GhostContext]&gt;</code>.
    ///
    /// This provides an impossible to misuse [`.raise()`][Self::raise] method.
    ///
    /** ```rust
    use ::ghosts::vestibule::*;

    let casper = ghost!({
        let ghost_ctx: GhostContext = raise!(Ghost);
        // …
    });
    ``` */
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub
    struct GhostContext(pub(super) ::never_say_never::Never);
}

impl GhostContext {
    /// Equivalent to [`raise()`], but for it being impossible to
    /// misuse.
    ///
    /** ```rust
    use ::ghosts::vestibule::*;

    fn ghost_ok_or<T, Err> (
        inside_a_ghost: GhostContext,
        g: Ghost<Option<T>>,
        err: Err,
    ) -> Result<T, Err>
    {
        inside_a_ghost.raise(g).ok_or(err)
    }

    let casper = ghost!({
        let ctx = raise!(Ghost);
        let none = ghost!(None::<()>);
        let _: i32 = match ghost_ok_or(ctx, none, 42) {
            | Ok(_unreachable) => unreachable!(),
            | Err(err) => err,
        };
    });
    ``` */
    pub
    fn raise<T> (
        self,
        _: Ghost<T>,
    ) -> T
    {
        self.0
    }
}

#[cfg(feature = "better-docs")]
#[doc(cfg(ghostǃ))]
/// Extract an imaginary `T` value out of a `Ghost<T>` phantom token.
///
/// ### Fake namespace
///
/// This macro does not really exist; it's **only available when inside a
/// [`ghost!`] block**.
/** ```rust
use ::ghosts::vestibule::*;

fn wrap_in_some<T> (g: Ghost<T>)
  -> Ghost<Option<T>
{
    ghost!({
        let it: T = raise!(g);
        Some(it)
    })
}
``` */
///
///   - (note that for this example, `.map()` could have been used instead)
#[macro_export]
macro_rules! raise {( $ghost:expr $(,)? ) => (
    $crate::ඞ::core::compile_error! {"\
        Did you really enabled the `better-docs` internal feature just to try \
        and call this fake macro??\
    "}
)}

/// # Examples
///
/// ### `?` and `.await` work inside it.
///
/** ```rust
use ::ghosts::vestibule::*;

fn main ()
  -> ::std::io::Result<()>
{
    let casper = ghost!({
        let foo = ::std::fs::File::open("/the/door")?;
    });
    async {
        let in_the_shell = ghost!({
            let bar = example().await;
        });
    };
    Ok(())
}

async
fn example ()
{
    // …
}
``` */
///
/// ### `break` and `continue` work inside it, provided they be labelled.
///
/** ```rust
use ::ghosts::vestibule::*;

let _: i32 = 'labelled: loop {
    let casper = ghost!({
        break 'labelled 42;
    });
    break 0;
};
``` */
///
/// ### `ghost!` expressions consume ownership of their captures…
///
/** ```rust ,compile_fail
use ::ghosts::vestibule::*;

let owned = String::from("…");
let casper = ghost!(owned);
drop(owned); // Error, use of moved value
``` */
///
///   - (note: once in the Ghost Realm™, things stay there. There is no way for
///     ownership relinquished over the Ghost Realm™ to ever be claimed back
///     outside of it)
///
/// ### … unless the `#![no_init]` opt-out is used
///
/** ```rust
use ::ghosts::vestibule::*;

let owned = String::from("…");
let casper = ghost!(#[no_init] {
    owned
});
drop(owned); // Ok
``` */
#[macro_export]
macro_rules! ghost {
    ( $($_:tt)* ) => (
        $crate::ඞ__ghost! {
            [normal_attrs ]
            [no_init false]
            $($_)*
        }
    );

    (
        $( #[tag($meta:meta)] )?
        $( #[no_init] )?
        $expr_or_block:expr
    ) => (
        $crate::::core::compile_error! { "unreachable" }
    );
}

#[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! ඞ__ghost {
    (
        $normal_attrs:tt
        $no_init:tt
        #[no_init]
        $($rest:tt)*
    ) => (
        $crate::ඞ__ghost! {
            $normal_attrs
            [no_init true]
            $($rest)*
        }
    );

    (
        $normal_attrs:tt
        $no_init:tt
        #[tag($attr:meta)]
        $($rest:tt)*
    ) => (
        $crate::ඞ__ghost! {
            $normal_attrs:tt
            $no_init
            $($rest)*
        }
    );

    (
        [normal_attrs
            $($attrs:tt)*
        ]
        $no_init:tt
        #[$attr:meta]
        $($rest:tt)*
    ) => (
        $crate::ඞ__ghost! {
            [normal_attrs
                $($attrs:tt)*
                #[$attr]
            ]
            $no_init
            $($rest)*
        }
    );

    (
        [normal_attrs
            $($attrs:tt)*
        ]
        [no_init
            $(false)?
            $(true $(if $no_init:tt)?)?
        ]
        $expr_or_block:expr
    ) => ({
        let it = $crate::Ghost;
        if false {
            macro_rules! raise {
                ( $e:expr , ) => ( raise!($e) );
                ( $e:expr ) => (
                    $crate::raise(
                        $($attrs)*
                        $e
                    )
                );
            }
            it.__set($expr_or_block);
            $($(if $no_init)?
                loop {}
            )?
        }
        $crate::ඞ::Flatten::__flatten(it)
    });
}

/// Ghosts oozy nature makes them susceptible to squashing, with a little bit
/// of effort.
pub(in crate)
mod flatten {
    use crate::vestibule::*;

    #[doc(hidden)]
    pub trait Flatten<T : ?Sized, CoherenceDisambiguator = ()> : Sized {
        fn __flatten (_: Self)
          -> Ghost<T>
        ;
    }

    impl<T : ?Sized> Flatten<T> for Ghost<T> {
        #[inline]
        fn __flatten (_: Ghost<T>)
          -> Ghost<T>
        {
            Ghost
        }
    }

    impl<T : ?Sized, G> Flatten<T, G> for Ghost<G>
    where
        G : Flatten<T>,
    {
        #[inline]
        fn __flatten (_: Ghost<G>)
          -> Ghost<T>
        {
            Ghost
        }
    }
}
