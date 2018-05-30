#![feature(proc_macro)]
#![crate_type = "proc-macro"]
extern crate proc_macro;
#[macro_use] extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, Type, Expr, WhereClause, TypeSlice, Path};
use syn::synom::Synom;

struct MiscSyntax {
   id: Ident,
   ty: Type,
   expr: Expr,
   where_clause: WhereClause,
   type_slice: TypeSlice,
   path: Path
}

impl Synom for MiscSyntax {
    named!(parse -> Self, do_parse!(
        keyword!(where) >>
        keyword!(while) >>
        id: syn!(Ident) >>
        punct!(:) >>
        ty: syn!(Type) >>
        punct!(>>) >>
        expr: syn!(Expr) >>
        punct!(;) >>
        where_clause: syn!(WhereClause) >>
        punct!(;) >>
        type_slice: syn!(TypeSlice) >>
        punct!(;) >>
        path: syn!(Path) >>
        (MiscSyntax { id, ty, expr, where_clause, type_slice, path })
    ));
}

#[proc_macro]
pub fn misc_syntax(input: TokenStream) -> TokenStream {
    let m: MiscSyntax = syn::parse(input).expect("expected Miscellaneous Syntax");
    let MiscSyntax { id, ty, expr, where_clause, type_slice, path } = m;

    (quote! {
       let #id: #ty = #expr;
       println!("variable = {}", #id);
    }).into()
}
