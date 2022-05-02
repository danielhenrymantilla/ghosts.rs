# The following snippets fail to compile

```rust ,compile_fail
use ::ghosts::vestibule::*;

let _: ::never_say_never::Never = ::ghosts::materialize(Ghost);
```

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->
