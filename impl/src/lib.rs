extern crate proc_macro;
use proc_macro::TokenStream;

struct Hack {
    attrs: String,
    name: String,
    name_impl: String,
}

#[proc_macro_derive(ProcMacroHackExpr)]
pub fn hack_expr(input: TokenStream) -> TokenStream {
    let hack = parse(input);

    let rules = format!("
        {attrs}
        #[macro_export]
        macro_rules! {name} {{
            ($($tt:tt)*) => {{{{
                #[derive({name_impl})]
                #[allow(unused)]
                enum ProcMacroHack {{
                    Input = (stringify!($($tt)*), 0).1
                }}

                proc_macro_call!()
            }}}}
        }}
    ", attrs=hack.attrs, name=hack.name, name_impl=hack.name_impl);

    rules.parse().unwrap()
}

#[proc_macro_derive(ProcMacroHackItem)]
pub fn hack_item(input: TokenStream) -> TokenStream {
    let hack = parse(input);

    let rules = format!("
        {attrs}
        #[macro_export]
        macro_rules! {name} {{
            ($($tt:tt)*) => {{
                #[derive({name_impl})]
                #[allow(unused)]
                enum ProcMacroHack {{
                    Input = (stringify!($($tt)*), 0).1
                }}
            }}
        }}
    ", attrs=hack.attrs, name=hack.name, name_impl=hack.name_impl);

    rules.parse().unwrap()
}

/// Parses an input that looks like:
///
/// ```rust,ignore
/// #[allow(unused, non_camel_case_types)]
/// $( #[$attr] )*
/// enum NAME {
///     NAME_IMPL
/// }
/// ```
fn parse(input: TokenStream) -> Hack {
    let source = input.to_string();

    let mut front = source.split_whitespace();
    let next = front.next().unwrap();
    if next == "#[allow(unused" {
        assert_eq!(Some(","), front.next());
    } else {
        assert_eq!("#[allow(unused,", next);
    }
    assert_eq!(Some("non_camel_case_types)]"), front.next());

    let mut back = source.split_whitespace().rev();
    assert_eq!(Some("}"), back.next());
    let name_impl = back.next().unwrap();
    assert_eq!(Some("{"), back.next());
    let name = back.next().unwrap();
    assert_eq!(Some("enum"), back.next());

    let start_attrs = source.find(']').unwrap() + 1;
    let end = source.rfind('}').unwrap();
    let end = source[..end].rfind(name_impl).unwrap();
    let end = source[..end].rfind('{').unwrap();
    let end = source[..end].rfind(name).unwrap();
    let end = source[..end].rfind("enum").unwrap();

    Hack {
        attrs: source[start_attrs..end].trim().to_owned(),
        name: name.to_owned(),
        name_impl: name_impl.to_owned(),
    }
}
