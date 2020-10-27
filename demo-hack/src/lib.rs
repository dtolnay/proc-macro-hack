use proc_macro_hack::proc_macro_hack;

/// Add one to an expression.
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack(only_hack_old_rustc)]
pub use demo_hack_impl::add_one;
