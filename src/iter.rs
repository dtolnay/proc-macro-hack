use proc_macro::{token_stream, TokenStream, TokenTree};
use std::iter::Peekable;

pub type Iter<'a> = &'a mut IterImpl;

pub struct IterImpl {
    tokens: Peekable<token_stream::IntoIter>,
}

pub fn new(tokens: TokenStream) -> IterImpl {
    IterImpl {
        tokens: tokens.into_iter().peekable(),
    }
}

impl IterImpl {
    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.tokens.peek()
    }
}

impl Iterator for IterImpl {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
    }
}
