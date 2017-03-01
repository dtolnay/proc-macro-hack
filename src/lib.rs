extern crate proc_macro;

// Allow the "unused" #[macro_use] because there is a different un-ignorable
// warning otherwise:
//
//    proc macro crates and `#[no_link]` crates have no effect without `#[macro_use]`
#[allow(unused_imports)]
#[macro_use]
extern crate proc_macro_hack_impl;
pub use proc_macro_hack_impl::*;

#[doc(hidden)]
pub use proc_macro::TokenStream;

#[macro_export]
macro_rules! proc_macro_expr_decl {
    ($name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackExpr)]
        #[allow(unused, non_camel_case_types)]
        enum $name {
            $name_impl
        }
    }
}

#[macro_export]
macro_rules! proc_macro_item_decl {
    ($name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackItem)]
        #[allow(unused, non_camel_case_types)]
        enum $name {
            $name_impl
        }
    }
}

#[macro_export]
macro_rules! proc_macro_expr_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str) -> String $body:block
    )+) => {
        $(
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: $crate::TokenStream) -> $crate::TokenStream {
                let source = input.to_string();
                let source = source.trim();

                let prefix = "#[allow(unused)]\nenum ProcMacroHack {";
                let suffix = "}";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "Input =";
                let suffix = "0).1,";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "(stringify!(";
                let suffix = "),";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let tokens = &source[prefix.len() .. source.len() - suffix.len()].trim();

                fn func($input: &str) -> String $body

                format!("
                    macro_rules! proc_macro_call {{
                        () => {{
                            {}
                        }}
                    }}
                ", func(tokens)).parse().unwrap()
            }
        )+
    }
}

#[macro_export]
macro_rules! proc_macro_item_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str) -> String $body:block
    )+) => {
        $(
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: $crate::TokenStream) -> $crate::TokenStream {
                let source = input.to_string();
                let source = source.trim();

                let prefix = "#[allow(unused)]\nenum ProcMacroHack {";
                let suffix = "}";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "Input =";
                let suffix = "0).1,";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let source = &source[prefix.len() .. source.len() - suffix.len()].trim();

                let prefix = "(stringify!(";
                let suffix = "),";
                assert!(source.starts_with(prefix));
                assert!(source.ends_with(suffix));
                let tokens = &source[prefix.len() .. source.len() - suffix.len()].trim();

                fn func($input: &str) -> String $body

                func(tokens).parse().unwrap()
            }
        )+
    }
}
