mod parse;

use self::parse::{parse_define_args, parse_export_args, parse_fake_call_site, parse_input};
use crate::error::Error;
use crate::iter::{self, Iter};
use crate::{ExportArgs, Visibility};
use proc_macro::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::fmt::Write;

pub(crate) use self::parse::parse_enum_hack;

pub(crate) enum Input {
    Export(Export),
    Define(Define),
}

// pub use demo_hack_impl::{m1, m2 as qrst};
pub(crate) struct Export {
    attrs: TokenStream,
    vis: Visibility,
    from: Ident,
    macros: Vec<Macro>,
}

// pub fn m1(input: TokenStream) -> TokenStream { ... }
pub(crate) struct Define {
    attrs: TokenStream,
    name: Ident,
    body: TokenStream,
}

struct Macro {
    name: Ident,
    export_as: Ident,
}

pub(crate) fn expand_proc_macro_hack(args: Iter, input: TokenStream) -> Result<TokenStream, Error> {
    let ref mut input = iter::new(input);
    match parse_input(input)? {
        Input::Export(export) => {
            let args = parse_export_args(args)?;
            Ok(expand_export(export, args))
        }
        Input::Define(define) => {
            parse_define_args(args)?;
            Ok(expand_define(define))
        }
    }
}

pub(crate) struct FakeCallSite {
    derive: Ident,
    rest: TokenStream,
}

pub(crate) fn expand_fake_call_site(args: Iter, input: Iter) -> Result<TokenStream, Error> {
    let span = match args.next() {
        Some(token) => token.span(),
        None => return Ok(input.collect()),
    };

    let input = parse_fake_call_site(input)?;
    let mut derive = input.derive;
    derive.set_span(span);
    let rest = input.rest;

    Ok(quote! {
        #[derive(#derive)]
        #rest
    })
}

pub(crate) fn expand_export(export: Export, args: ExportArgs) -> TokenStream {
    let dummy = dummy_name_for_export(&export);

    let attrs = export.attrs;
    let ref vis = export.vis.map(|span| Ident::new("pub", span));
    let macro_export = match vis {
        Some(_) => quote!(#[macro_export]),
        None => quote!(),
    };
    let crate_prefix = vis.as_ref().map(|_| quote!($crate::));
    let enum_variant = if args.support_nested {
        if args.internal_macro_calls == 0 {
            Ident::new("Nested", Span::call_site())
        } else {
            let name = format!("Nested{}", args.internal_macro_calls);
            Ident::new(&name, Span::call_site())
        }
    } else {
        Ident::new("Value", Span::call_site())
    };

    let from = export.from;
    let mut actual_names = TokenStream::new();
    let mut export_dispatch = TokenStream::new();
    let mut export_call_site = TokenStream::new();
    let mut macro_rules = TokenStream::new();
    for Macro { name, export_as } in &export.macros {
        let actual_name = actual_proc_macro_name(&name);
        let dispatch = dispatch_macro_name(&name);
        let call_site = call_site_macro_name(&name);

        if !actual_names.is_empty() {
            actual_names.extend(quote!(,));
        }
        actual_names.extend(quote!(#actual_name));

        if !export_dispatch.is_empty() {
            export_dispatch.extend(quote!(,));
        }
        export_dispatch.extend(quote!(dispatch as #dispatch));

        if !export_call_site.is_empty() {
            export_call_site.extend(quote!(,));
        }
        export_call_site.extend(quote!(fake_call_site as #call_site));

        let do_derive = if !args.fake_call_site {
            quote! {
                #[derive(#crate_prefix #actual_name)]
            }
        } else if crate_prefix.is_some() {
            quote! {
                use #crate_prefix #actual_name;
                #[#crate_prefix #call_site ($($proc_macro)*)]
                #[derive(#actual_name)]
            }
        } else {
            quote! {
                #[#call_site ($($proc_macro)*)]
                #[derive(#actual_name)]
            }
        };

        let proc_macro_call = if args.support_nested {
            let extra_bangs = (0..args.internal_macro_calls)
                .map(|_| TokenTree::Punct(Punct::new('!', Spacing::Alone)))
                .collect::<TokenStream>();
            quote! {
                #crate_prefix #dispatch! { ($($proc_macro)*) #extra_bangs }
            }
        } else {
            quote! {
                proc_macro_call!()
            }
        };

        macro_rules.extend(quote! {
            #attrs
            #macro_export
            macro_rules! #export_as {
                ($($proc_macro:tt)*) => {{
                    #do_derive
                    #[allow(dead_code)]
                    enum ProcMacroHack {
                        #enum_variant = (stringify! { $($proc_macro)* }, 0).1,
                    }
                    #proc_macro_call
                }};
            }
        });
    }

    if export.macros.len() != 1 {
        export_dispatch = quote!({#export_dispatch});
        export_call_site = quote!({#export_call_site});
        actual_names = quote!({#actual_names});
    }

    let export_dispatch = if args.support_nested {
        quote! {
            #[doc(hidden)]
            #vis use proc_macro_nested::#export_dispatch;
        }
    } else {
        quote!()
    };

    let export_call_site = if args.fake_call_site {
        quote! {
            #[doc(hidden)]
            #vis use proc_macro_hack::#export_call_site;
        }
    } else {
        quote!()
    };

    let expanded = quote! {
        #[doc(hidden)]
        #vis use #from::#actual_names;

        #export_dispatch
        #export_call_site

        #macro_rules
    };

    wrap_in_enum_hack(dummy, expanded)
}

pub(crate) fn expand_define(define: Define) -> TokenStream {
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
            iter.next().unwrap(); // `#`
            iter.next().unwrap(); // `[allow(dead_code)]`

            let mut braces = match iter.next().unwrap() {
                #dummy::TokenTree::Group(group) => group.stream().into_iter(),
                _ => unimplemented!(),
            };
            let variant = braces.next().unwrap(); // `Value` or `Nested`
            let varname = variant.to_string();
            let support_nested = varname.starts_with("Nested");
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

            let output: #dummy::TokenStream = #name(inner.clone());

            fn count_bangs(input: #dummy::TokenStream) -> usize {
                let mut count = 0;
                for token in input {
                    match token {
                        #dummy::TokenTree::Punct(punct) => {
                            if punct.as_char() == '!' {
                                count += 1;
                            }
                        }
                        #dummy::TokenTree::Group(group) => {
                            count += count_bangs(group.stream());
                        }
                        _ => {}
                    }
                }
                count
            }

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
                    #dummy::Ident::new(
                        &if support_nested {
                            let extra_bangs = if varname == "Nested" {
                                0
                            } else {
                                varname["Nested".len()..].parse().unwrap()
                            };
                            format!("proc_macro_call_{}", extra_bangs + count_bangs(inner))
                        } else {
                            String::from("proc_macro_call")
                        },
                        #dummy::Span::call_site(),
                    ),
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
    Ident::new(
        &format!("proc_macro_hack_{}", conceptual),
        conceptual.span(),
    )
}

fn dispatch_macro_name(conceptual: &Ident) -> Ident {
    Ident::new(
        &format!("proc_macro_call_{}", conceptual),
        conceptual.span(),
    )
}

fn call_site_macro_name(conceptual: &Ident) -> Ident {
    Ident::new(
        &format!("proc_macro_fake_call_site_{}", conceptual),
        conceptual.span(),
    )
}

fn dummy_name_for_export(export: &Export) -> String {
    let mut dummy = String::new();
    let from = unraw(&export.from).to_string();
    write!(dummy, "_{}{}", from.len(), from).unwrap();
    for m in &export.macros {
        let name = unraw(&m.name).to_string();
        write!(dummy, "_{}{}", name.len(), name).unwrap();
    }
    dummy
}

fn unraw(ident: &Ident) -> Ident {
    let string = ident.to_string();
    if string.starts_with("r#") {
        Ident::new(&string[2..], ident.span())
    } else {
        ident.clone()
    }
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
