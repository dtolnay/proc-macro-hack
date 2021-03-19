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
//! ```rust
//! #[macro_use]
//! extern crate proc_macro_hack;
//!
//! // This is what allows the users to depend on just your
//! // declaration crate rather than both crates.
//! #[allow(unused_imports)]
//! #[macro_use]
//! extern crate demo_hack_impl;
//! #[doc(hidden)]
//! pub use demo_hack_impl::*;
//!
//! proc_macro_expr_decl! {
//!     /// Add one to an expression.
//!     add_one! => add_one_impl
//! }
//!
//! proc_macro_item_decl! {
//!     /// A function that always returns 2.
//!     two_fn! => two_fn_impl
//! }
//! # fn main() {}
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
//! proc-macro-hack = "0.4"
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
//! ```rust
//! #[macro_use]
//! extern crate demo_hack;
//!
//! two_fn!(two);
//!
//! fn main() {
//!     let nine = add_one!(two()) + add_one!(2 + 3);
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

#![no_std]

// Allow the "unused" #[macro_use] because there is a different un-ignorable
// warning otherwise:
//
//    proc macro crates and `#[no_link]` crates have no effect without `#[macro_use]`
#[allow(unused_imports)]
#[macro_use]
extern crate proc_macro_hack_impl;
#[doc(hidden)]
pub use proc_macro_hack_impl::*;

/// Declare a hacky procedural macro that expands to an expression.
///
/// ```rust
/// # #[macro_use] extern crate proc_macro_hack;
/// proc_macro_expr_decl! {
///     /// Add one to an expression.
///     add_one! => add_one_impl
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! proc_macro_expr_decl {
    (#[$attr:meta] $($rest:tt)+) => {
        proc_macro_expr_decl_helper!((#[$attr]) $($rest)+);
    };
    ($name:ident ! => $name_impl:ident) => {
        proc_macro_expr_decl_helper!(() $name ! => $name_impl);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_expr_decl_helper {
    (($($attrs:tt)*) #[$first:meta] $($rest:tt)+) => {
        proc_macro_expr_decl_helper!(($($attrs)* #[$first]) $($rest)+);
    };
    (($($attrs:tt)*) $name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackExpr)]
        #[allow(unused, non_camel_case_types)]
        $($attrs)*
        enum $name {
            $name_impl
        }
    };
    (($($attrs:tt)*) $name:ident ! => $name_impl:ident #[$first:meta] $($rest:tt)+) => {
        proc_macro_expr_decl_helper!(($($attrs)*) $name ! => $name_impl);
        proc_macro_expr_decl_helper!((#[$first]) $($rest)+);
    };
}

/// Declare a hacky procedural macro that expands to items.
///
/// ```rust
/// # #[macro_use] extern crate proc_macro_hack;
/// proc_macro_item_decl! {
///     /// A function that always returns 2.
///     two_fn! => two_fn_impl
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! proc_macro_item_decl {
    (#[$attr:meta] $($rest:tt)+) => {
        proc_macro_item_decl_helper!((#[$attr]) $($rest)+);
    };
    ($name:ident ! => $name_impl:ident) => {
        proc_macro_item_decl_helper!(() $name ! => $name_impl);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_item_decl_helper {
    (($($attrs:tt)*) #[$first:meta] $($rest:tt)+) => {
        proc_macro_item_decl_helper!(($($attrs)* #[$first]) $($rest)+);
    };
    (($($attrs:tt)*) $name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackItem)]
        #[allow(unused, non_camel_case_types)]
        $($attrs)*
        enum $name {
            $name_impl
        }
    };
    (($($attrs:tt)*) $name:ident ! => $name_impl:ident #[$first:meta] $($rest:tt)+) => {
        proc_macro_item_decl_helper!(($($attrs)*) $name ! => $name_impl);
        proc_macro_item_decl_helper!((#[$first]) $($rest)+);
    };
}

/// Implement a hacky procedural macro that expands to an expression.
///
/// ```rust,ignore
/// proc_macro_expr_impl! {
///     /// Add one to an expression.
///     pub fn add_one_impl(input: &str) -> String {
///         format!("1 + {}", input)
///     }
/// }
/// ```
#[macro_export]
macro_rules! proc_macro_expr_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str) -> String $body:block
    )+) => {
        $(
            mod $func {
                extern crate proc_macro;
                pub use self::proc_macro::TokenStream;
            }

            // Parses an input that looks like:
            //
            // ```
            // #[allow(unused)]
            // enum ProcMacroHack {
            //     Input = (stringify!(ARGS), 0).1,
            // }
            // ```
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: $func::TokenStream) -> $func::TokenStream {
                let source = input.to_string();
                let mut tokens = source.trim();

                for &prefix in &[
                    "#",
                    "[",
                    "allow",
                    "(",
                    "unused",
                    ")",
                    "]",
                    "enum",
                    "ProcMacroHack",
                    "{",
                    "Input",
                    "=",
                    "(",
                    "stringify",
                    "!",
                    "(",
                ] {
                    assert!(tokens.starts_with(prefix));
                    tokens = &tokens[prefix.len()..].trim();
                }

                for &suffix in &[
                    "}",
                    ",",
                    "1",
                    ".",
                    ")",
                    "0",
                    ",",
                    ")",
                ] {
                    if suffix == "," && !tokens.ends_with(suffix) {
                        continue;
                    }
                    assert!(tokens.ends_with(suffix));
                    tokens = &tokens[..tokens.len() - suffix.len()].trim();
                }

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
    };
}

/// Implement a hacky procedural macro that expands to items.
///
/// ```rust,ignore
/// proc_macro_item_impl! {
///     /// A function that always returns 2.
///     pub fn two_fn_impl(input: &str) -> String {
///         format!("fn {}() -> u8 {{ 2 }}", input)
///     }
/// }
/// ```
#[macro_export]
macro_rules! proc_macro_item_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str) -> String $body:block
    )+) => {
        $(
            mod $func {
                extern crate proc_macro;
                pub use self::proc_macro::TokenStream;
            }

            // Parses an input that looks like:
            //
            // ```
            // #[allow(unused)]
            // enum ProcMacroHack {
            //     Input = (stringify!(ARGS), 0).1,
            // }
            // ```
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: $func::TokenStream) -> $func::TokenStream {
                let source = input.to_string();
                let mut tokens = source.trim();

                for &prefix in &[
                    "#",
                    "[",
                    "allow",
                    "(",
                    "unused",
                    ")",
                    "]",
                    "enum",
                    "ProcMacroHack",
                    "{",
                    "Input",
                    "=",
                    "(",
                    "stringify",
                    "!",
                    "(",
                ] {
                    assert!(tokens.starts_with(prefix));
                    tokens = &tokens[prefix.len()..].trim();
                }

                for &suffix in &[
                    "}",
                    ",",
                    "1",
                    ".",
                    ")",
                    "0",
                    ",",
                    ")",
                ] {
                    if suffix == "," && !tokens.ends_with(suffix) {
                        continue;
                    }
                    assert!(tokens.ends_with(suffix));
                    tokens = &tokens[..tokens.len() - suffix.len()].trim();
                }

                fn func($input: &str) -> String $body

                func(tokens).parse().unwrap()
            }
        )+
    };
}
