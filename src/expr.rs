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
///   - When directly inside a [`ghost!`] block, use [`materialize!`].
///
///   - Otherwise, require that the caller pass some [`Ectoplasm`],
///     which they can have by [`materialize()`]-ing their own
///     <code>[Ghost]&lt;[Ectoplasm]&gt;</code>.
///
///     With it, you'll have access to its impossible to misuse
///     [`.materialize()`][Ectoplasm::materialize] method.
///
///   - Or perhaps way more simply, consider using the [`.map()`][Ghost] or
///     [`.and_then()`][Ghost] adapters.
pub
fn materialize<T> (
    _: Ghost<T>,
) -> T
{
    #![allow(unconditional_recursion)]
    struct MaterializeOutsideOfAGhostBlockError(u8);
    materialize::<(T, MaterializeOutsideOfAGhostBlockError)>(Ghost).0
}

pub use __::Ectoplasm;
#[allow(unreachable_code)]
mod __ {
    #[allow(unused)]
    use super::*;

    /// You know you are in the Ghost Realm™ when ectoplasm oozes all around you…
    ///
    /// Unforgeable token which can thus only be constructed by
    /// [`materialize!`]-ing a <code>[Ghost]&lt;[Ectoplasm]&gt;</code>.
    ///
    /// This provides an impossible to misuse
    /// [`.materialize()`][Self::materialize] method.
    ///
    /** ```rust
    use ::ghosts::vestibule::*;

    let casper = ghost!({
        let inside_a_ghost: Ectoplasm = materialize!(Ghost);
        // …
    });
    ``` */
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub
    struct Ectoplasm(pub(super) ::never_say_never::Never);
}

impl Ectoplasm {
    /// Equivalent to [`materialize()`], but for it being impossible to
    /// misuse.
    ///
    /** ```rust
    use ::ghosts::vestibule::*;

    fn ghost_ok_or<T, Err> (
        inside_a_ghost: Ectoplasm,
        g: Ghost<Option<T>>,
        err: Err,
    ) -> Result<T, Err>
    {
        inside_a_ghost.materialize(g).ok_or(err)
    }

    let casper = ghost!({
        let ectoplasm = materialize!(Ghost);
        let none = ghost!(None::<()>);
        let _: i32 = match ghost_ok_or(ectoplasm, none, 42) {
            | Ok(_unreachable) => unreachable!(),
            | Err(err) => err,
        };
    });
    ``` */
    pub
    fn materialize<T> (
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
  -> Ghost<Option<T>>
{
    ghost!({
        let it: T = materialize!(g);
        Some(it)
    })
}
``` */
///
///   - (note that for this example, `.map()` could have been used instead)
#[macro_export]
macro_rules! materialize {( $ghost:expr $(,)? ) => (
    $crate::ඞ::core::compile_error! {"\
        Did you really enabled the `better-docs` internal feature just to try \
        and call this fake macro??\
    "}
)}

// Work around a docs.rs bug…
#[doc(hidden)]
#[cfg(feature = "better-docs")]
pub use docsrs_y_u_do_dis::*;
#[doc(hidden)]
#[cfg(feature = "better-docs")]
pub mod docsrs_y_u_do_dis {
    pub use materialize;
}

/// `Ghost` expressions. `PhantomCode` of sorts, if you want.
///
/// See [the main docs][crate] for more info.
///
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
/// ### … unless the `#[no_init]` opt-out is used
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
        $( |$ghost_ctx:pat| )?
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
            $normal_attrs
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
        |$ghost_ctx:pat_param| $expr_or_block:expr
    ) => ({
        let it = $crate::Ghost;
        if false {
            macro_rules! materialize {
                ( $e:expr , ) => ( materialize!($e) );
                ( $e:expr ) => (
                    $crate::materialize(
                        $($attrs)*
                        $e
                    )
                );
            }
            let $ghost_ctx = $crate::ectoplasm!();
            it.__set($expr_or_block);
            $($(if $no_init)?
                loop {}
            )?
        }
        $crate::ඞ::Flatten::__flatten(it)
    });

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
            macro_rules! materialize {
                ( $e:expr , ) => ( materialize!($e) );
                ( $e:expr ) => (
                    $crate::materialize(
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

/// Produces a [`materialize!`]d expression with the same type as the return
/// type of the function where the invocation occurs.
#[macro_export]
macro_rules! materialize_return {() => ({
    let ret = materialize!($crate::Ghost);
    if false {
        return ret;
    } else {
        ret
    }
})}

/// Shorthand for `materialize!(Ghost) : Ectoplasm`.
#[macro_export]
macro_rules! ectoplasm {() => (
    materialize!(
        $crate::Ghost::<$crate::ඞ::core::marker::PhantomData<$crate::Ectoplasm>>
    )
)}

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
