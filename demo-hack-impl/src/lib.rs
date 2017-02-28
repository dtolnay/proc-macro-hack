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
