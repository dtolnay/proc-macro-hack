extern crate proc_macro;

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
                #[derive(ProcMacroHackExpr)]
                struct Ignored;
                let _ = Ignored;

                let source = input.to_string();
                let source = source.trim();

                let prefix = "#[allow(unused)]\nenum ProcMacroHack { Input = (stringify!(";
                let suffix = "), 0).1, }";
                if !(source.starts_with(prefix) && source.ends_with(suffix)) {
                    panic!("`{}` procedural macro failed", stringify!($func));
                }

                let tokens = &source[prefix.len() .. source.len() - suffix.len()];

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
                #[derive(ProcMacroHackItem)]
                struct Ignored;
                let _ = Ignored;

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
