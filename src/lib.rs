//! As of Rust 1.30, the language supports user-defined function-like procedural
//! macros. However these can only be invoked in item position, not in
//! statements or expressions.
//!
//! This crate implements an alternative type of procedural macro that can be
//! invoked in statement or expression position.
//!
//! # Defining procedural macros
//!
//! Two crates are required to define a procedural macro.
//!
//! ## The implementation crate
//!
//! This crate must contain nothing but procedural macros. Private helper
//! functions and private modules are fine but nothing can be public.
//!
//! [> example of an implementation crate][demo-hack-impl]
//!
//! Just like you would use a #\[proc_macro\] attribute to define a natively
//! supported procedural macro, use proc-macro-hack's #\[proc_macro_hack\]
//! attribute to define a procedural macro that works in expression position.
//! The function signature is the same as for ordinary function-like procedural
//! macros.
//!
//! ```rust
//! extern crate proc_macro;
//! extern crate proc_macro_hack;
//! extern crate quote;
//! extern crate syn;
//!
//! use proc_macro::TokenStream;
//! use proc_macro_hack::proc_macro_hack;
//! use quote::quote;
//! use syn::{parse_macro_input, Expr};
//!
//! /// Add one to an expression.
//! # const IGNORE: &str = stringify! {
//! #[proc_macro_hack]
//! # };
//! pub fn add_one(input: TokenStream) -> TokenStream {
//!     let expr = parse_macro_input!(input as Expr);
//!     TokenStream::from(quote! {
//!         1 + (#expr)
//!     })
//! }
//! #
//! # fn main() {}
//! ```
//!
//! ## The declaration crate
//!
//! This crate is allowed to contain other public things if you need, for
//! example traits or functions or ordinary macros.
//!
//! [> example of a declaration crate][demo-hack]
//!
//! Within the declaration crate there needs to be a re-export of your
//! procedural macro from the implementation crate. The re-export also carries a
//! \#\[proc_macro_hack\] attribute.
//!
//! ```rust
//! extern crate demo_hack_impl;
//! extern crate proc_macro_hack;
//!
//! use proc_macro_hack::proc_macro_hack;
//!
//! /// Add one to an expression.
//! #[proc_macro_hack]
//! pub use demo_hack_impl::add_one;
//! #
//! # fn main() {}
//! ```
//!
//! Both crates depend on `proc-macro-hack`:
//!
//! ```toml
//! [dependencies]
//! proc-macro-hack = "0.5"
//! ```
//!
//! Additionally, your implementation crate (but not your declaration crate) is
//! a proc macro crate:
//!
//! ```toml
//! [lib]
//! proc-macro = true
//! ```
//!
//! # Using procedural macros
//!
//! Users of your crate depend on your declaration crate (not your
//! implementation crate), then use your procedural macros as usual.
//!
//! [> example of a downstream crate][example]
//!
//! ```rust
//! extern crate demo_hack;
//! use demo_hack::add_one;
//!
//! fn main() {
//!     let two = 2;
//!     let nine = add_one!(two) + add_one!(2 + 3);
//!     println!("nine = {}", nine);
//! }
//! ```
//!
//! [demo-hack-impl]: https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack-impl
//! [demo-hack]: https://github.com/dtolnay/proc-macro-hack/tree/master/demo-hack
//! [example]: https://github.com/dtolnay/proc-macro-hack/tree/master/example

#![recursion_limit = "256"]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use std::fmt::Write;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, bracketed, parenthesized, parse_macro_input, token, Ident, Token};

type Visibility = Option<Token![pub]>;

enum Input {
    Export(Export),
    Define(Define),
}

// pub use demo_hack_impl::{m1, m2 as qrst};
struct Export {
    attrs: TokenStream,
    vis: Visibility,
    from: Ident,
    macros: Vec<Macro>,
}

// pub fn m1(input: TokenStream) -> TokenStream { ... }
struct Define {
    attrs: TokenStream,
    name: Ident,
    body: TokenStream,
}

struct Macro {
    name: Ident,
    export_as: Ident,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let ahead = input.fork();
        parse_attributes(&ahead)?;
        ahead.parse::<Visibility>()?;

        if ahead.peek(Token![use]) {
            input.parse().map(Input::Export)
        } else if ahead.peek(Token![fn]) {
            input.parse().map(Input::Define)
        } else {
            Err(input.error("unexpected input to #[proc_macro_hack]"))
        }
    }
}

impl Parse for Export {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(parse_attributes)?;
        let vis: Visibility = input.parse()?;
        input.parse::<Token![use]>()?;
        let from: Ident = input.parse()?;
        input.parse::<Token![::]>()?;

        let mut macros = Vec::new();
        if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            loop {
                macros.push(content.parse()?);
                if content.is_empty() {
                    break;
                }
                content.parse::<Token![,]>()?;
                if content.is_empty() {
                    break;
                }
            }
        } else {
            macros.push(input.parse()?);
        }

        input.parse::<Token![;]>()?;
        Ok(Export { attrs, vis, from, macros })
    }
}

impl Parse for Define {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(parse_attributes)?;
        let vis: Visibility = input.parse()?;
        if vis.is_none() {
            return Err(input.error("functions tagged with `#[proc_macro_hack]` must be `pub`"));
        }

        input.parse::<Token![fn]>()?;
        let name: Ident = input.parse()?;
        let body: TokenStream = input.parse()?;
        Ok(Define { attrs, name, body })
    }
}

impl Parse for Macro {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let renamed: Option<Token![as]> = input.parse()?;
        let export_as = if renamed.is_some() {
            input.parse()?
        } else {
            name.clone()
        };
        Ok(Macro { name, export_as })
    }
}

fn parse_attributes(input: ParseStream) -> Result<TokenStream> {
    let mut attrs = TokenStream::new();
    while input.peek(Token![#]) {
        let pound: Token![#] = input.parse()?;
        pound.to_tokens(&mut attrs);
        let content;
        let bracket_token = bracketed!(content in input);
        let content: TokenStream = content.parse()?;
        bracket_token.surround(&mut attrs, |tokens| content.to_tokens(tokens));
    }
    Ok(attrs)
}

#[proc_macro_attribute]
pub fn proc_macro_hack(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(match parse_macro_input!(input) {
        Input::Export(export) => expand_export(export),
        Input::Define(define) => expand_define(define),
    })
}

struct EnumHack {
    token_stream: TokenStream,
}

impl Parse for EnumHack {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![enum]>()?;
        input.parse::<Ident>()?;

        let braces;
        braced!(braces in input);
        braces.parse::<Ident>()?;
        braces.parse::<Token![=]>()?;

        let parens;
        parenthesized!(parens in braces);
        parens.parse::<Ident>()?;
        parens.parse::<Token![!]>()?;

        let inner;
        braced!(inner in parens);
        let token_stream: TokenStream = inner.parse()?;

        parens.parse::<Token![,]>()?;
        parens.parse::<TokenTree>()?;
        braces.parse::<Token![.]>()?;
        braces.parse::<TokenTree>()?;
        braces.parse::<Token![,]>()?;

        Ok(EnumHack { token_stream })
    }
}

#[doc(hidden)]
#[proc_macro_derive(ProcMacroHack)]
pub fn enum_hack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inner = parse_macro_input!(input as EnumHack);
    proc_macro::TokenStream::from(inner.token_stream)
}

fn expand_export(export: Export) -> TokenStream {
    let dummy = dummy_name_for_export(&export);

    let attrs = export.attrs;
    let vis = export.vis;
    let macro_export = match vis {
        Some(_) => quote!(#[macro_export]),
        None => quote!(),
    };
    let crate_prefix = match vis {
        Some(_) => quote!($crate::),
        None => quote!(),
    };

    let from = export.from;
    let rules = export
        .macros
        .into_iter()
        .map(|Macro { name, export_as }| {
            let actual_name = actual_proc_macro_name(&name);

            quote! {
                #[doc(hidden)]
                #vis use #from::#actual_name;

                #attrs
                #macro_export
                macro_rules! #export_as {
                    ($($proc_macro:tt)*) => {{
                        #[derive(#crate_prefix #actual_name)]
                        enum ProcMacroHack {
                            Value = (stringify! { $($proc_macro)* }, 0).1,
                        }
                        proc_macro_call!()
                    }};
                }
            }
        })
        .collect();

    wrap_in_enum_hack(dummy, rules)
}

fn expand_define(define: Define) -> TokenStream {
    let attrs = define.attrs;
    let name = define.name;
    let dummy = actual_proc_macro_name(&name);
    let body = define.body;

    quote! {
        mod #dummy {
            extern crate proc_macro;
            pub use self::proc_macro::*;
        }

        #attrs
        #[proc_macro_derive(#dummy)]
        pub fn #dummy(input: #dummy::TokenStream) -> #dummy::TokenStream {
            use std::iter::FromIterator;

            let mut iter = input.into_iter();
            iter.next().unwrap(); // `enum`
            iter.next().unwrap(); // `ProcMacroHack`

            let mut braces = match iter.next().unwrap() {
                #dummy::TokenTree::Group(group) => group.stream().into_iter(),
                _ => unimplemented!(),
            };
            braces.next().unwrap(); // `Value`
            braces.next().unwrap(); // `=`

            let mut parens = match braces.next().unwrap() {
                #dummy::TokenTree::Group(group) => group.stream().into_iter(),
                _ => unimplemented!(),
            };
            parens.next().unwrap(); // `stringify`
            parens.next().unwrap(); // `!`

            let inner = match parens.next().unwrap() {
                #dummy::TokenTree::Group(group) => group.stream(),
                _ => unimplemented!(),
            };

            let output: #dummy::TokenStream = #name(inner);

            // macro_rules! proc_macro_call {
            //     () => { #output }
            // }
            #dummy::TokenStream::from_iter(vec![
                #dummy::TokenTree::Ident(
                    #dummy::Ident::new("macro_rules", #dummy::Span::call_site()),
                ),
                #dummy::TokenTree::Punct(
                    #dummy::Punct::new('!', #dummy::Spacing::Alone),
                ),
                #dummy::TokenTree::Ident(
                    #dummy::Ident::new("proc_macro_call", #dummy::Span::call_site()),
                ),
                #dummy::TokenTree::Group(
                    #dummy::Group::new(#dummy::Delimiter::Brace, #dummy::TokenStream::from_iter(vec![
                        #dummy::TokenTree::Group(
                            #dummy::Group::new(#dummy::Delimiter::Parenthesis, #dummy::TokenStream::new()),
                        ),
                        #dummy::TokenTree::Punct(
                            #dummy::Punct::new('=', #dummy::Spacing::Joint),
                        ),
                        #dummy::TokenTree::Punct(
                            #dummy::Punct::new('>', #dummy::Spacing::Alone),
                        ),
                        #dummy::TokenTree::Group(
                            #dummy::Group::new(#dummy::Delimiter::Brace, output),
                        ),
                    ])),
                ),
            ])
        }

        fn #name #body
    }
}

fn actual_proc_macro_name(conceptual: &Ident) -> Ident {
    let actual_name = format!("proc_macro_hack_{}", conceptual);
    Ident::new(&actual_name, Span::call_site())
}

fn dummy_name_for_export(export: &Export) -> String {
    let mut dummy = String::new();
    write!(dummy, "_{}{}", export.from.to_string().len(), export.from);
    for m in &export.macros {
        write!(dummy, "_{}{}", m.name.to_string().len(), m.name);
    }
    dummy
}

fn wrap_in_enum_hack(dummy: String, inner: TokenStream) -> TokenStream {
    let dummy = Ident::new(&dummy, Span::call_site());
    quote! {
        #[derive(proc_macro_hack::ProcMacroHack)]
        enum #dummy {
            Value = (stringify! { #inner }, 0).1,
        }
    }
}
