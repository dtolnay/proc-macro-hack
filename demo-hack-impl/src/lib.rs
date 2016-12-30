#![feature(proc_macro, proc_macro_lib)]

#[macro_use] extern crate proc_macro_hack;
#[macro_use] extern crate proc_macro_hack_impl;

proc_macro_expr_impl! {
    /// Add one to an expression.
    pub fn add_one_impl(input: &str) -> Result<String, String> {
        Ok(format!("1 + {}", input))
    }
}

proc_macro_item_impl! {
    /// A function that always returns 2.
    pub fn two_fn_impl(input: &str) -> Result<String, String> {
        Ok(format!("fn {}() -> u8 {{ 2 }}", input))
    }
}
