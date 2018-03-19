Procedural functionlike!() macros using only Macros 1.1
=======================================================

[![Build Status](https://api.travis-ci.org/dtolnay/proc-macro-hack.svg?branch=master)](https://travis-ci.org/dtolnay/proc-macro-hack)
[![Latest Version](https://img.shields.io/crates/v/proc-macro-hack.svg)](https://crates.io/crates/proc-macro-hack)

Did you think Macros 1.1 was only for custom derives? Think again.

This approach works with any Rust version >= 1.15.0.

## Defining procedural macros

Two crates are required to define a macro.

### The declaration crate

This crate is allowed to contain other public things if you need, for example
traits or functions or ordinary macros.

https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack

```rust
#[macro_use]
extern crate proc_macro_hack;

// This is what allows the users to depend on just your
// declaration crate rather than both crates.
#[allow(unused_imports)]
#[macro_use]
extern crate demo_hack_impl;
#[doc(hidden)]
pub use demo_hack_impl::*;

proc_macro_expr_decl! {
    /// Add one to an expression.
    add_one! => add_one_impl
}

proc_macro_item_decl! {
    /// A function that always returns 2.
    two_fn! => two_fn_impl
}
```

### The implementation crate

This crate must contain nothing but procedural macros. Private helper functions
and private modules are fine but nothing can be public.

A less trivial macro would probably use the [`syn`] crate to parse its input and
the [`quote`] crate to generate the output.

[`syn`]: https://github.com/dtolnay/syn
[`quote`]: https://github.com/dtolnay/quote

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
proc-macro-hack = "0.4"
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
    let nine = add_one!(two()) + add_one!(2 + 3);
    println!("nine = {}", nine);
}
```

## Crates based on this approach

- [`indoc`] – Macro that allows the content of string literals to be indented in
  source code.
- [`structure`] – Macro that uses a format string to create strongly-typed data
  pack/unpack interfaces.
- [`bstring`] – Macro for formatting byte strings.
- [`net-literals`] – Macros for writing IP/socket address literals that are
  checked for validity at compile time.
- [`wstr`] – Macros for compile-time UTF-16 (wide) string literals.
- [`hexf`] – Macros that enable hexadecimal floating point literals.
- [`binary_macros`] – Macros for decoding base64 and hexadecimal-like encodings
  from string literals to [u8] literals at compile time.
- [`autoimpl`] – Macro to generate a default blanket impl for a generic trait.
- [`array-macro`] – Macro for concisely building large arrays.
- [`reql`] – Includes a macro to splice an array of ReQL arguments into another
  term.

[`indoc`]: https://github.com/dtolnay/indoc
[`structure`]: https://docs.rs/structure/0.1.1/structure/
[`bstring`]: https://github.com/murarth/bstring
[`net-literals`]: https://github.com/canndrew/net-literals
[`wstr`]: https://github.com/nitric1/wstr-rs
[`hexf`]: https://github.com/lifthrasiir/hexf
[`binary_macros`]: https://github.com/golddranks/binary_macros
[`autoimpl`]: https://github.com/blakepettersson/autoimpl
[`array-macro`]: https://docs.rs/array-macro/0.1.1/array_macro/
[`reql`]: https://docs.rs/reql/0.0.8/reql/macro.args.html

## Limitations

- An item macro cannot be invoked multiple times within the same scope ([#2]).
- An expression macro cannot expand into recursive calls to itself ([#4]).
- The input to your macro cannot contain dollar signs ([#8]).
- Your macro must expand to either an expression or zero-or-more items, cannot
  sometimes be one or the other depending on input ([#9]).
- Type macros are not supported ([#10]).
- Input to an expression macro may not refer to hygienic identifiers of local
  variables ([#15]).
- An item macro cannot be used as an item in an impl block ([#18]).
- Macro output may not refer to the special metavariable `$crate` ([#19]).

[#2]: https://github.com/dtolnay/proc-macro-hack/issues/2
[#4]: https://github.com/dtolnay/proc-macro-hack/issues/4
[#8]: https://github.com/dtolnay/proc-macro-hack/issues/8
[#9]: https://github.com/dtolnay/proc-macro-hack/issues/9
[#10]: https://github.com/dtolnay/proc-macro-hack/issues/10
[#15]: https://github.com/dtolnay/proc-macro-hack/issues/15
[#18]: https://github.com/dtolnay/proc-macro-hack/issues/18
[#19]: https://github.com/dtolnay/proc-macro-hack/issues/19

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this hack by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
