//! Crate not intended for direct use.
//! Use https:://docs.rs/ghosts instead.
// Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template

use ::proc_macro::*;

#[proc_macro] pub
fn __loop (code: TokenStream)
  -> TokenStream
{
    // Imbues `'__ghost` with a `call_site()` span.
    let mut ret = stringify!('__ghost: loop).parse::<TokenStream>().unwrap();
    ret.extend([
        TokenTree::from(Group::new(
            Delimiter::Brace,
            code,
        ))
    ]);
    ret
}

#[proc_macro] pub
fn __break (code: TokenStream)
  -> TokenStream
{
    // Imbues `'__ghost` with a `call_site()` span.
    let mut ret = stringify!(break '__ghost).parse::<TokenStream>().unwrap();
    ret.extend(code);
    ret
}
