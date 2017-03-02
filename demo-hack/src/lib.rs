#[macro_use]
extern crate proc_macro_hack;

// Allow the "unused" #[macro_use] because there is a different un-ignorable
// warning otherwise:
//
//    proc macro crates and `#[no_link]` crates have no effect without `#[macro_use]`
#[allow(unused_imports)]
#[macro_use]
extern crate demo_hack_impl;
pub use demo_hack_impl::*;

/// Add one to an expression.
proc_macro_expr_decl!(add_one! => add_one_impl);

/// A function that always returns 2.
proc_macro_item_decl!(two_fn! => two_fn_impl);
