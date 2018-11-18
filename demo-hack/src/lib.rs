extern crate demo_hack_impl;
extern crate proc_macro_hack;
extern crate proc_macro_nested;

use proc_macro_hack::proc_macro_hack;

/// Add one to an expression.
#[proc_macro_hack]
pub use demo_hack_impl::add_one;
