use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;

pub struct Error {
    span: Span,
    msg: String,
}

impl Error {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        Error {
            span,
            msg: msg.into(),
        }
    }
}

pub fn compile_error(err: Error) -> TokenStream {
    let span = err.span;
    let msg = err.msg;
    quote_spanned!(span=> compile_error! { #msg })
}
