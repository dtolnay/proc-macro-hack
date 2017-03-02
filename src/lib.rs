//! ## Defining procedural macros
//!
//! Two crates are required to define a macro.
//!
//! ### The declaration crate
//!
//! This crate is allowed to contain other public things if you need, for example
//! traits or functions or ordinary macros.
//!
//! https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate proc_macro_hack;
//!
//! #[allow(unused_imports)]
//! #[macro_use]
//! extern crate demo_hack_impl;
//! pub use demo_hack_impl::*;
//!
//! /// Add one to an expression.
//! proc_macro_expr_decl!(add_one! => add_one_impl);
//!
//! /// A function that always returns 2.
//! proc_macro_item_decl!(two_fn! => two_fn_impl);
//! ```
//!
//! ### The implementation crate
//!
//! This crate must contain nothing but procedural macros. Private helper functions
//! and private modules are fine but nothing can be public.
//!
//! https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack-impl
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate proc_macro_hack;
//!
//! proc_macro_expr_impl! {
//!     /// Add one to an expression.
//!     pub fn add_one_impl(input: &str) -> String {
//!         format!("1 + {}", input)
//!     }
//! }
//!
//! proc_macro_item_impl! {
//!     /// A function that always returns 2.
//!     pub fn two_fn_impl(input: &str) -> String {
//!         format!("fn {}() -> u8 {{ 2 }}", input)
//!     }
//! }
//! ```
//!
//! Both crates depend on `proc-macro-hack`:
//!
//! ```toml
//! [dependencies]
//! proc-macro-hack = "0.3"
//! ```
//!
//! Additionally, your implementation crate (but not your declaration crate) is a
//! proc macro:
//!
//! ```toml
//! [lib]
//! proc-macro = true
//! ```
//!
//! ## Using procedural macros
//!
//! Users of your crate depend on your declaration crate (not your implementation
//! crate), then use your procedural macros as though it were magic. They even get
//! reasonable error messages if your procedural macro panics.
//!
//! https://github.com/dtolnay/proc-macro-hack/tree/master/example
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate demo_hack;
//!
//! two_fn!(two);
//!
//! fn main() {
//!     let x = two();
//!     let nine = add_one!(x) + add_one!(2 + 3);
//!     println!("nine = {}", nine);
//! }
//! ```
//!
//! ---
//!
//! # Expansion of expression macros
//!
//! ```rust,ignore
//! m!(ARGS)
//! ```
//!
//! ... expands to ...
//!
//! ```rust,ignore
//! {
//!     #[derive(m_impl)]
//!     #[allow(unused)]
//!     enum ProcMacroHack {
//!         Input = (stringify!(ARGS), 0).1,
//!     }
//!     proc_macro_call!()
//! }
//! ```
//!
//! ... expands to ...
//!
//! ```rust,ignore
//! {
//!     macro_rules! proc_macro_call {
//!         () => { RESULT }
//!     }
//!     proc_macro_call!()
//! }
//! ```
//!
//! ... expands to ...
//!
//! ```rust,ignore
//! {
//!     RESULT
//! }
//! ```
//!
//! # Expansion of item macros
//!
//! ```rust,ignore
//! m!(ARGS);
//! ```
//!
//! ... expands to ...
//!
//! ```rust,ignore
//! #[derive(m_impl)]
//! #[allow(unused)]
//! enum ProcMacroHack {
//!     Input = (stringify!(ARGS), 0).1,
//! }
//! ```
//!
//! ... expands to ...
//!
//! ```rust,ignore
//! RESULT
//! ```

#[cfg(feature = "proc_macro")]
extern crate proc_macro;

// Allow the "unused" #[macro_use] because there is a different un-ignorable
// warning otherwise:
//
//    proc macro crates and `#[no_link]` crates have no effect without `#[macro_use]`
#[allow(unused_imports)]
#[macro_use]
extern crate proc_macro_hack_impl;
pub use proc_macro_hack_impl::*;

#[cfg(feature = "proc_macro")]
#[doc(hidden)]
pub use proc_macro::TokenStream;

#[cfg(feature = "proc_macro")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_tokenstream {
    () => {
        $crate::TokenStream
    }
}

#[cfg(not(feature = "proc_macro"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_tokenstream {
    () => {
        ::proc_macro::TokenStream
    }
}

#[macro_export]
macro_rules! proc_macro_expr_decl {
    ($name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackExpr)]
        #[allow(unused, non_camel_case_types)]
        enum $name {
            $name_impl
        }
    }
}

#[macro_export]
macro_rules! proc_macro_item_decl {
    ($name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackItem)]
        #[allow(unused, non_camel_case_types)]
        enum $name {
            $name_impl
        }
    }
}

#[macro_export]
macro_rules! proc_macro_expr_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str) -> String $body:block
    )+) => {
        $(
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: proc_macro_tokenstream!()) -> proc_macro_tokenstream!() {
                let source = input.to_string();
                let source = source.trim();

                let prefix = "#[allow(unused)]\nenum ProcMacroHack {";
                let suffix = "}";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "Input =";
                let suffix = "0).1,";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "(stringify!(";
                let suffix = "),";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let tokens = &source[prefix.len() .. source.len() - suffix.len()].trim();

                fn func($input: &str) -> String $body

                format!("
                    macro_rules! proc_macro_call {{
                        () => {{
                            {}
                        }}
                    }}
                ", func(tokens)).parse().unwrap()
            }
        )+
    }
}

#[macro_export]
macro_rules! proc_macro_item_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str) -> String $body:block
    )+) => {
        $(
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: proc_macro_tokenstream!()) -> proc_macro_tokenstream!() {
                let source = input.to_string();
                let source = source.trim();

                let prefix = "#[allow(unused)]\nenum ProcMacroHack {";
                let suffix = "}";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "Input =";
                let suffix = "0).1,";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "(stringify!(";
                let suffix = "),";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let tokens = &source[prefix.len() .. source.len() - suffix.len()].trim();

                fn func($input: &str) -> String $body

                func(tokens).parse().unwrap()
            }
        )+
    }
}
