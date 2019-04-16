extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro_hack]
pub fn add_one(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    TokenStream::from(quote! {
        1 + (#expr)
    })
}
