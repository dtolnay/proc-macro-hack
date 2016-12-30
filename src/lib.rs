#![feature(proc_macro_lib)]

extern crate proc_macro;

#[doc(hidden)]
pub use proc_macro::TokenStream;

#[macro_export]
macro_rules! proc_macro_expr_decl {
    ($name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackExpr)]
        #[allow(unused)]
        enum $name {
            $name_impl
        }
    }
}

#[macro_export]
macro_rules! proc_macro_item_decl {
    ($name:ident ! => $name_impl:ident) => {
        #[derive(ProcMacroHackItem)]
        #[allow(unused)]
        enum $name {
            $name_impl
        }
    }
}

#[macro_export]
macro_rules! proc_macro_expr_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str ) -> Result<String, String> $body:block
    )+) => {
        $(
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: $crate::TokenStream) -> $crate::TokenStream {
                let source = input.to_string();

                let prefix = "#[allow(unused)]\nenum ProcMacroHack { Input = { stringify!(";
                let suffix = "); 0 }, }\n";
                if !(source.starts_with(prefix) && source.ends_with(suffix)) {
                    panic!("`{}` procedural macro failed", stringify!($func));
                }

                let tokens = &source[prefix.len() .. source.len() - suffix.len()];

                fn func($input: &str) -> Result<String, String> $body

                let wrap = match func(tokens) {
                    Ok(expr) => {
                        format!("
                            macro_rules! proc_macro_call {{
                                ($($tt:tt)*) => {{
                                    {}
                                }}
                            }}
                        ", expr)
                    }
                    Err(msg) => panic!(msg)
                };

                wrap.parse().unwrap()
            }
        )+
    }
}

#[macro_export]
macro_rules! proc_macro_item_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str ) -> Result<String, String> $body:block
    )+) => {
        $(
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: $crate::TokenStream) -> $crate::TokenStream {
                let source = input.to_string();

                let prefix = "#[allow(unused)]\nenum ProcMacroHack { Input = { stringify!(";
                let suffix = "); 0 }, }\n";
                if !(source.starts_with(prefix) && source.ends_with(suffix)) {
                    panic!("`{}` procedural macro failed", stringify!($func));
                }

                let tokens = &source[prefix.len() .. source.len() - suffix.len()];

                fn func($input: &str) -> Result<String, String> $body

                func(tokens).unwrap_or_else(|msg| panic!(msg)).parse().unwrap()
            }
        )+
    }
}
