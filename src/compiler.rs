use crate::error::Error;
use crate::iter::{self, Iter};
use crate::{ExportArgs, Visibility};
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

fn parse_punct(tokens: Iter, ch: char) -> Result<(), Error> {
    match tokens.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ch => {
            tokens.next().unwrap();
            Ok(())
        }
        tt => Err(Error::new(
            tt.map_or_else(Span::call_site, TokenTree::span),
            format!("expected `{}`", ch),
        )),
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

fn parse_int(tokens: Iter) -> Result<u16, Span> {
    match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string().parse().map_err(|_| lit.span()),
        Some(tt) => Err(tt.span()),
        None => Err(Span::call_site()),
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

fn parse_export_args(tokens: Iter) -> Result<ExportArgs, Error> {
    let mut args = ExportArgs {
        support_nested: false,
        internal_macro_calls: 0,
        fake_call_site: false,
    };

    while let Some(tt) = tokens.next() {
        match &tt {
            TokenTree::Ident(ident) if ident.to_string() == "support_nested" => {
                args.support_nested = true;
            }
            TokenTree::Ident(ident) if ident.to_string() == "internal_macro_calls" => {
                parse_punct(tokens, '=')?;
                let calls = parse_int(tokens).map_err(|span| {
                    Error::new(span, "expected integer value for internal_macro_calls")
                })?;
                args.internal_macro_calls = calls;
            }
            TokenTree::Ident(ident) if ident.to_string() == "fake_call_site" => {
                args.fake_call_site = true;
            }
            _ => {
                return Err(Error::new(
                    tt.span(),
                    "expected one of: `support_nested`, `internal_macro_calls`, `fake_call_site`",
                ))
            }
        }
        if tokens.peek().is_none() {
            break;
        }
        parse_punct(tokens, ',')?;
    }

    Ok(args)
}

fn parse_define_args(tokens: Iter) -> Result<(), Error> {
    if tokens.peek().is_none() {
        Ok(())
    } else {
        Err(Error::new(Span::call_site(), "unexpected input"))
    }
}
