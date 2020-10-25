use crate::error::Error;
use crate::iter::Iter;
use crate::ExportArgs;
use proc_macro::{Span, TokenTree};

pub(crate) fn parse_export_args(tokens: Iter) -> Result<ExportArgs, Error> {
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

pub(crate) fn parse_define_args(tokens: Iter) -> Result<(), Error> {
    match tokens.peek() {
        None => Ok(()),
        Some(token) => Err(Error::new(
            token.span(),
            "unexpected argument to proc_macro_hack macro implementation; args are only accepted on the macro declaration (the `pub use`)",
        )),
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

fn parse_int(tokens: Iter) -> Result<u16, Span> {
    match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string().parse().map_err(|_| lit.span()),
        Some(tt) => Err(tt.span()),
        None => Err(Span::call_site()),
    }
}
