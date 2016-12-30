#![feature(proc_macro)]

#[macro_use] extern crate proc_macro_hack;
#[macro_use] extern crate proc_macro_hack_impl;

/// Add one to an expression.
proc_macro_expr_decl!(add_one! => add_one_impl);

/// A function that always returns 2.
proc_macro_item_decl!(two_fn! => two_fn_impl);
