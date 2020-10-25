use crate::args::{parse_define_args, parse_export_args};
use crate::error::Error;
use crate::iter::{self, Iter};
use crate::Visibility;
use proc_macro::Delimiter::Bracket;
use proc_macro::{Ident, Span, TokenStream, TokenTree};

enum Input {
    Export,
    Define,
}

pub(crate) fn expand_proc_macro_hack(args: Iter, input: TokenStream) -> Result<TokenStream, Error> {
    let ref mut iter = iter::new(input.clone());
    match parse_input(iter)? {
        Input::Export => {
            parse_export_args(args)?;
            Ok(input)
        }
        Input::Define => {
            parse_define_args(args)?;
            Ok(quote! {
                #[proc_macro]
                #input
            })
        }
    }
}

fn parse_input(tokens: Iter) -> Result<Input, Error> {
    let _attrs = parse_attributes(tokens)?;
    let vis = parse_visibility(tokens)?;
    let kw = parse_ident(tokens)?;
    if kw.to_string() == "use" {
        Ok(Input::Export)
    } else if kw.to_string() == "fn" {
        if vis.is_none() {
            return Err(Error::new(
                kw.span(),
                "functions tagged with `#[proc_macro_hack]` must be `pub`",
            ));
        }
        Ok(Input::Define)
    } else {
        Err(Error::new(
            kw.span(),
            "unexpected input to #[proc_macro_hack]",
        ))
    }
}

fn parse_ident(tokens: Iter) -> Result<Ident, Error> {
    match tokens.next() {
        Some(TokenTree::Ident(ident)) => Ok(ident),
        tt => Err(Error::new(
            tt.as_ref().map_or_else(Span::call_site, TokenTree::span),
            "expected identifier",
        )),
    }
}

fn parse_visibility(tokens: Iter) -> Result<Visibility, Error> {
    if let Some(TokenTree::Ident(ident)) = tokens.peek() {
        if ident.to_string() == "pub" {
            return Ok(Some(tokens.next().unwrap().span()));
        }
    }
    Ok(None)
}

fn parse_attributes(tokens: Iter) -> Result<TokenStream, Error> {
    let mut attrs = TokenStream::new();
    while let Some(TokenTree::Punct(punct)) = tokens.peek() {
        if punct.as_char() != '#' {
            break;
        }
        let span = punct.span();
        attrs.extend(tokens.next());
        match tokens.peek() {
            Some(TokenTree::Group(group)) if group.delimiter() == Bracket => {
                attrs.extend(tokens.next());
            }
            _ => return Err(Error::new(span, "unexpected input")),
        }
    }
    Ok(attrs)
}
