# `::ghosts`

Type-check non-existing `Phantom` code for Fun And Profit™.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/ghosts.rs)
[![Latest version](https://img.shields.io/crates/v/ghosts.svg)](
https://crates.io/crates/ghosts)
[![Documentation](https://docs.rs/ghosts/badge.svg)](
https://docs.rs/ghosts)
[![MSRV](https://img.shields.io/badge/MSRV-1.56.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/ghosts.svg)](
https://github.com/danielhenrymantilla/ghosts.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/ghosts.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/ghosts.rs/actions)

![image](https://user-images.githubusercontent.com/9920355/166281534-7a38e90c-aa9a-47be-83e5-fa64d1fa1249.png)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

## Rationale

Sometimes you may want to write Rust code that ought to be type-checked
(_e.g._, borrow-checked) in the same fashion as real Rust code even though
that code is never intended to be run / to affect or even reach code
generation.

Why? Well, ok, this need is _incredibly niche_. But code verification tools can
benefit from this, so [there is a proposal out there] to add a bunch of magic
language features just to support this.

[there is a proposal out there]: https://github.com/rust-lang/lang-team/issues/161

This crates is a demonstration of the vast majority of such features being
achievable within already existing Stable Rust code, thanks to macros, type
inference, and `if false` (+ unreachable code paths) tricks.

```rust
use ::ghosts::vestibule::*;

type Nat = u64;

fn fibo (
    ghost_ctx: Ectoplasm,
    n: Ghost<Nat>,
) -> Nat
{
    let n = ghost_ctx.materialize(n);
    ghost!(#[tag(decreases)] #[no_init] { n });
    match n {
        | 0 => 0,
        | 1 => 1,
        | _ => {
            fibo(ghost_ctx, ghost!(n - 1))
            +
            fibo(ghost_ctx, ghost!(n - 2))
        },
    }
}

fn lemma_fibo_is_monotonic (
    ghost_ctx: Ectoplasm,
    i: Ghost<Nat>,
    j: Ghost<Nat>,
)
{
    let i = ghost_ctx.materialize(i);
    let j = ghost_ctx.materialize(j);
    ghost!(#[tag(requires)] #[no_init] { i <= j });
    ghost!(#[tag(ensures)] #[no_init] { i <= j });
    ghost!(#[tag(ensures)] #[no_init] {
        fibo(ghost_ctx, ghost!(i)) <= fibo(ghost_ctx, ghost!(j))
    });
    ghost!(#[tag(decreases)] #[no_init] { j - 1 });

    match () {
        | _case if i < 2 && j < 2 => {},
        | _case if i == j => {},
        | _case if i == j - 1 => {
            // reveal_with_fuel(fibo, 2);
            lemma_fibo_is_monotonic(ghost_ctx, ghost!(i), ghost!(j - 1));
        },
        | _default => {
            lemma_fibo_is_monotonic(ghost_ctx, ghost!(i), ghost!(j - 1));
            lemma_fibo_is_monotonic(ghost_ctx, ghost!(i), ghost!(j - 2));
        },
    }
}

fn fibo_fits_u64 (
    ghost_ctx: Ectoplasm,
    n: Ghost<Nat>,
) -> bool
{
    fibo(ghost_ctx, n) <= 0xffff_ffff_ffff_ffff
}

fn assume (
    _: bool
)
{}

fn fibo_impl (n: u64)
  -> u64
{
    ghost!(#[tag(requires)] #[no_init] |ectoplasm| {
        fibo_fits_u64(ectoplasm, ghost!(n))
    });
    ghost!(#[tag(ensures)] #[no_init] |ectoplasm| {
        materialize_return!() == fibo(ectoplasm, ghost!(n))
    });

    if n == 0 {
        return 0;
    }
    let mut prev: u64 = 0;
    let mut cur: u64 = 1;
    let mut i: u64 = 1;
    while i < n {
        ghost!(#[tag(invariant)] #[no_init] |ectoplasm| [
            i > 0,
            i <= n,
            fibo_fits_u64(ectoplasm,
                ghost!(#[tag(spec_expr)] #[no_init] n as Nat),
            ),
            fibo_fits_u64(ectoplasm,
                ghost!(#[tag(spec_expr)] #[no_init] i as Nat),
            ),
            cur == fibo(ectoplasm,
                ghost!(#[tag(spec_expr)] #[no_init] i),
            ),
            prev == fibo(ectoplasm,
                ghost!(#[tag(spec_expr)] #[no_init] { i as Nat - 1 }),
            ),
        ]);
        ghost!(#[tag(proof)] {
            assume(cur as Nat + prev <= 0xffff_ffff_ffff_ffff);
        });
        let new_cur = cur + prev;
        prev = cur;
        cur = new_cur;
        i += 1;
        ghost!(#[tag(proof)] |ectoplasm| {
            lemma_fibo_is_monotonic(ectoplasm,
                ghost!(#[tag(spec_expr)] #[no_init] { i }),
                ghost!(#[tag(spec_expr)] #[no_init] { n }),
            );
        });
    }
    cur
}
```
