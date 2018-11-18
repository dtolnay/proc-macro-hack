//! Support for nested invocations of proc-macro-hack expression macros.
//!
//! By default, macros defined through proc-macro-hack do not support nested
//! invocations, i.e. the code emitted by a proc-macro-hack macro invocation
//! cannot contain recursive calls to the same proc-macro-hack macro nor calls
//! to any other proc-macro-hack macros.
//!
//! This crate provides opt-in support for such nested invocations.
//!
//! To make a macro callable recursively, add a dependency on this crate from
//! your declaration crate and update the `#[proc_macro_hack]` re-export as
//! follows.
//!
//! ```rust
//! // Before
//! # const IGNORE: &str = stringify! {
//! #[proc_macro_hack]
//! pub use demo_hack_impl::add_one;
//! # }
//! ```
//!
//! ```rust
//! // After
//! # const IGNORE: &str = stringify! {
//! extern crate proc_macro_nested;
//!
//! #[proc_macro_hack(support_nested)]
//! pub use demo_hack_impl::add_one;
//! # }
//! ```
//!
//! No change is required within your definition crate, only in the declaration
//! crate.

include!(concat!(env!("OUT_DIR"), "/count.rs"));

#[doc(hidden)]
#[macro_export]
macro_rules! dispatch {
    (() $($bang:tt)*) => {
        $crate::count!($($bang)*)
    };
    ((($($first:tt)*) $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($first)* $($rest)*) $($bang)*)
    };
    (([$($first:tt)*] $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($first)* $($rest)*) $($bang)*)
    };
    (({$($first:tt)*} $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($first)* $($rest)*) $($bang)*)
    };
    ((! $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($rest)*) $($bang)* !)
    };
    (($first:tt $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($rest)*) $($bang)*)
    };
}
