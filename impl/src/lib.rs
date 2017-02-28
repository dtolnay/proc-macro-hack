extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(ProcMacroHackExpr)]
pub fn hack_expr(input: TokenStream) -> TokenStream {
    let (name, name_impl) = names(input);

    let rules = format!("
        #[macro_export]
        macro_rules! {} {{
            ($($tt:tt)*) => {{{{
                #[derive({})]
                #[allow(unused)]
                enum ProcMacroHack {{
                    Input = (stringify!($($tt)*), 0).1
                }}

                proc_macro_call!()
            }}}}
        }}
    ", name, name_impl);

    rules.parse().unwrap()
}

#[proc_macro_derive(ProcMacroHackItem)]
pub fn hack_item(input: TokenStream) -> TokenStream {
    let (name, name_impl) = names(input);

    let rules = format!("
        #[macro_export]
        macro_rules! {} {{
            ($($tt:tt)*) => {{
                #[derive({})]
                #[allow(unused)]
                enum ProcMacroHack {{
                    Input = (stringify!($($tt)*), 0).1
                }}
            }}
        }}
    ", name, name_impl);

    rules.parse().unwrap()
}

fn names(input: TokenStream) -> (String, String) {
    let source = input.to_string();

    let mut words = source.split_whitespace();
    assert_eq!(Some("#[allow(unused,"), words.next());
    assert_eq!(Some("non_camel_case_types)]"), words.next());
    assert_eq!(Some("enum"), words.next());
    let name = words.next().unwrap();
    assert_eq!(Some("{"), words.next());
    let name_impl = words.next().unwrap();
    assert_eq!(Some("}"), words.next());
    assert_eq!(None, words.next());

    (name.to_owned(), name_impl.to_owned())
}
