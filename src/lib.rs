extern crate proc_macro;
extern crate uuid;

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

#[doc(hidden)]
pub use uuid::Uuid;

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

                let prefix = "#[allow(unused)]\nenum ProcMacroHack { Input = (stringify!(";
                let suffix = "), 0).1, }";
                if !(source.starts_with(prefix) && source.ends_with(suffix)) {
                    panic!("`{}` procedural macro failed", stringify!($func));
                }

                let tokens = &source[prefix.len() .. source.len() - suffix.len()];

                let uuid = $crate::Uuid::new_v4();

                fn func($input: &str) -> String $body

                let items = format!("
                    macro_rules! proc_macro_{uuid} {{
                        () => {{
                            {result}
                        }}
                    }}

                    #[macro_use]
                    mod proc_macro_mod {{
                        macro_rules! proc_macro_call {{
                            () => {{
                                proc_macro_{uuid}!()
                            }}
                        }}
                    }}
                ", result=func(tokens), uuid=uuid.simple());

                items.parse().unwrap()
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

                let prefix = "#[allow(unused)]\nenum ProcMacroHack { Input = (stringify!(";
                let suffix = "), 0).1, }";
                if !(source.starts_with(prefix) && source.ends_with(suffix)) {
                    panic!("`{}` procedural macro failed", stringify!($func));
                }

                let tokens = &source[prefix.len() .. source.len() - suffix.len()];

                fn func($input: &str) -> String $body

                func(tokens).parse().unwrap()
            }
        )+
    }
}
