//! [![github]](https://github.com/dtolnay/proc-macro-hack)&ensp;[![crates-io]](https://crates.io/crates/proc-macro-hack)&ensp;[![docs-rs]](https://docs.rs/proc-macro-hack)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! <table><tr><td><hr>
//! <b>Note:</b> <i>As of Rust 1.45 this crate is superseded by native support
//! for #[proc_macro] in expression position. Only consider using this crate if
//! you care about supporting compilers between 1.31 and 1.45.</i>
//! <hr></td></tr></table>
//!
//! Since Rust 1.30, the language supports user-defined function-like procedural
//! macros. However these can only be invoked in item position, not in
//! statements or expressions.
//!
//! This crate implements an alternative type of procedural macro that can be
//! invoked in statement or expression position.
//!
//! # Defining procedural macros
//!
//! Two crates are required to define a procedural macro.
//!
//! ## The implementation crate
//!
//! This crate must contain nothing but procedural macros. Private helper
//! functions and private modules are fine but nothing can be public.
//!
//! [&raquo; example of an implementation crate][demo-hack-impl]
//!
//! Just like you would use a #\[proc_macro\] attribute to define a natively
//! supported procedural macro, use proc-macro-hack's #\[proc_macro_hack\]
//! attribute to define a procedural macro that works in expression position.
//! The function signature is the same as for ordinary function-like procedural
//! macros.
//!
//! ```
//! # extern crate proc_macro;
//! #
//! use proc_macro::TokenStream;
//! use proc_macro_hack::proc_macro_hack;
//! use quote::quote;
//! use syn::{parse_macro_input, Expr};
//!
//! # const IGNORE: &str = stringify! {
//! #[proc_macro_hack]
//! # };
//! pub fn add_one(input: TokenStream) -> TokenStream {
//!     let expr = parse_macro_input!(input as Expr);
//!     TokenStream::from(quote! {
//!         1 + (#expr)
//!     })
//! }
//! #
//! # fn main() {}
//! ```
//!
//! ## The declaration crate
//!
//! This crate is allowed to contain other public things if you need, for
//! example traits or functions or ordinary macros.
//!
//! [&raquo; example of a declaration crate][demo-hack]
//!
//! Within the declaration crate there needs to be a re-export of your
//! procedural macro from the implementation crate. The re-export also carries a
//! \#\[proc_macro_hack\] attribute.
//!
//! ```
//! use proc_macro_hack::proc_macro_hack;
//!
//! /// Add one to an expression.
//! ///
//! /// (Documentation goes here on the re-export, not in the other crate.)
//! #[proc_macro_hack]
//! pub use demo_hack_impl::add_one;
//! #
//! # fn main() {}
//! ```
//!
//! Both crates depend on `proc-macro-hack`:
//!
//! ```toml
//! [dependencies]
//! proc-macro-hack = "0.5"
//! ```
//!
//! Additionally, your implementation crate (but not your declaration crate) is
//! a proc macro crate:
//!
//! ```toml
//! [lib]
//! proc-macro = true
//! ```
//!
//! # Using procedural macros
//!
//! Users of your crate depend on your declaration crate (not your
//! implementation crate), then use your procedural macros as usual.
//!
//! [&raquo; example of a downstream crate][example]
//!
//! ```
//! use demo_hack::add_one;
//!
//! fn main() {
//!     let two = 2;
//!     let nine = add_one!(two) + add_one!(2 + 3);
//!     println!("nine = {}", nine);
//! }
//! ```
//!
//! [demo-hack-impl]: https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack-impl
//! [demo-hack]: https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack
//! [example]: https://github.com/dtolnay/proc-macro-hack/tree/master/example
//!
//! # Limitations
//!
//! - Only proc macros in expression position are supported. Proc macros in
//!   pattern position ([#20]) are not supported.
//!
//! - By default, nested invocations are not supported i.e. the code emitted by
//!   a proc-macro-hack macro invocation cannot contain recursive calls to the
//!   same proc-macro-hack macro nor calls to any other proc-macro-hack macros.
//!   Use [`proc-macro-nested`] if you require support for nested invocations.
//!
//! - By default, hygiene is structured such that the expanded code can't refer
//!   to local variables other than those passed by name somewhere in the macro
//!   input. If your macro must refer to *local* variables that don't get named
//!   in the macro input, use `#[proc_macro_hack(fake_call_site)]` on the
//!   re-export in your declaration crate. *Most macros won't need this.*
//!
//! [#10]: https://github.com/dtolnay/proc-macro-hack/issues/10
//! [#20]: https://github.com/dtolnay/proc-macro-hack/issues/20
//! [`proc-macro-nested`]: https://docs.rs/proc-macro-nested

#![recursion_limit = "512"]
#![allow(clippy::needless_doctest_main, clippy::toplevel_ref_arg)]

extern crate proc_macro;

#[macro_use]
mod quote;

mod args;
mod error;
mod iter;

#[cfg_attr(not(fn_like_proc_macro), path = "fallback/mod.rs")]
#[cfg_attr(fn_like_proc_macro, path = "compiler.rs")]
mod imp;
use crate::imp::*;

use crate::error::compile_error;
use proc_macro::{Span, TokenStream};

type Visibility = Option<Span>;

#[proc_macro_attribute]
pub fn proc_macro_hack(args: TokenStream, input: TokenStream) -> TokenStream {
    let ref mut args = iter::new(args);
    expand_proc_macro_hack(args, input).unwrap_or_else(compile_error)
}

#[cfg(not(fn_like_proc_macro))]
#[doc(hidden)]
#[proc_macro_derive(ProcMacroHack)]
pub fn enum_hack(input: TokenStream) -> TokenStream {
    let ref mut input = iter::new(input);
    parse_enum_hack(input).unwrap_or_else(compile_error)
}

#[cfg(not(fn_like_proc_macro))]
#[doc(hidden)]
#[proc_macro_attribute]
pub fn fake_call_site(args: TokenStream, input: TokenStream) -> TokenStream {
    let ref mut args = iter::new(args);
    let ref mut input = iter::new(input);
    expand_fake_call_site(args, input).unwrap_or_else(compile_error)
}

struct ExportArgs {
    support_nested: bool,
    internal_macro_calls: u16,
    fake_call_site: bool,
}
