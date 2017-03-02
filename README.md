Procedural functionlike!() macros using only Macros 1.1
=======================================================

[![Build Status](https://api.travis-ci.org/dtolnay/proc-macro-hack.svg?branch=master)](https://travis-ci.org/dtolnay/proc-macro-hack)
[![Latest Version](https://img.shields.io/crates/v/proc-macro-hack.svg)](https://crates.io/crates/proc-macro-hack)

Did you think Macros 1.1 was only for custom derives? Think again.

*Note:* Requires stable Rust 1.15 or beta Rust 1.16. This library is currently
affected by a regression in rustc, [rust-lang/rust#39889][regression], and does
not work on the most recent nightlies. It works on stable and beta but not
nightly. How times have changed, huh? A fix has been merged in
[rust-lang/rust#39419][fix] and should be in the next nightly after March 1.

[regression]: https://github.com/rust-lang/rust/issues/39889
[fix]: https://github.com/rust-lang/rust/issues/39419

## Defining procedural macros

Two crates are required to define a macro.

### The declaration crate

This crate is allowed to contain other public things if you need, for example
traits or functions or ordinary macros.

https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack

```rust
#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate demo_hack_impl;
pub use demo_hack_impl::*;

/// Add one to an expression.
proc_macro_expr_decl!(add_one! => add_one_impl);

/// A function that always returns 2.
proc_macro_item_decl!(two_fn! => two_fn_impl);
```

### The implementation crate

This crate must contain nothing but procedural macros. Private helper functions
and private modules are fine but nothing can be public.

https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack-impl

```rust
#[macro_use]
extern crate proc_macro_hack;

proc_macro_expr_impl! {
    /// Add one to an expression.
    pub fn add_one_impl(input: &str) -> String {
        format!("1 + {}", input)
    }
}

proc_macro_item_impl! {
    /// A function that always returns 2.
    pub fn two_fn_impl(input: &str) -> String {
        format!("fn {}() -> u8 {{ 2 }}", input)
    }
}
```

Both crates depend on `proc-macro-hack`:

```toml
[dependencies]
proc-macro-hack = "0.3"
```

Additionally, your implementation crate (but not your declaration crate) is a
proc macro:

```toml
[lib]
proc-macro = true
```

## Using procedural macros

Users of your crate depend on your declaration crate (not your implementation
crate), then use your procedural macros as though it were magic. They even get
reasonable error messages if your procedural macro panics.

https://github.com/dtolnay/proc-macro-hack/tree/master/example

```rust
#[macro_use]
extern crate demo_hack;

two_fn!(two);

fn main() {
    let x = two();
    let nine = add_one!(x) + add_one!(2 + 3);
    println!("nine = {}", nine);
}
```

## Limitations

- The input to your macro cannot contain dollar signs.
- Your macro must expand to either an expression or zero-or-more items, cannot
  sometimes be one or the other depending on input.
- An item macro cannot be invoked multiple times within the same scope
  ([#2][multiple]).
- An expression macro cannot expand into recursive calls to itself
  ([#4][recursive]).

[multiple]: https://github.com/dtolnay/proc-macro-hack/issues/2
[recursive]: https://github.com/dtolnay/proc-macro-hack/issues/4

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this hack by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
