#![feature(proc_macro)]
#![crate_type = "proc-macro"]
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
use proc_macro::TokenStream;

#[proc_macro]
pub fn f(input: TokenStream) -> TokenStream {
    assert!(input.is_empty());

    (quote! {
       1 + 2
    }).into()
}
