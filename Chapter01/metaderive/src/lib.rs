#![crate_type = "proc-macro"]
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
use proc_macro::TokenStream;

#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    // Parse token stream into input AST
    let ast = syn::parse(input).unwrap();

    // Generate output AST
    impl_typename(&ast).into()
}

fn impl_typename(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl TypeName for #name {
            fn typename() -> String {
                stringify!(#name).to_string()
            }
        }
    }
}
