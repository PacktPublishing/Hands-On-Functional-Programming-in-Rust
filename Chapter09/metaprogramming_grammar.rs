macro_rules! match_tt {
   ($e: tt) => { println!("match_tt: {}", stringify!($e)) }
}

macro_rules! match_ident {
   ($e: ident) => { println!("match_ident: {}", stringify!($e)) }
}

macro_rules! match_expr {
   ($e: expr) => { println!("match_expr: {}", stringify!($e)) }
}

macro_rules! match_ty {
   ($e: ty) => { println!("match_ty: {}", stringify!($e)) }
}

macro_rules! match_stmt {
   ($e: stmt) => { println!("match_stmt: {}", stringify!($e)) }
}

macro_rules! match_block {
   ($e: block) => { println!("match_block: {}", stringify!($e)) }
}

macro_rules! match_item {
   ($e: item) => { println!("match_item: {}", stringify!($e)) }
}

macro_rules! match_pat {
   ($e: pat) => { println!("match_pat: {}", stringify!($e)) }
}

macro_rules! match_path {
   ($e: path) => { println!("match_path: {}", stringify!($e)) }
}

macro_rules! match_meta {
   ($e: meta) => { println!("match_meta: {}", stringify!($e)) }
}

fn main() {
   match_tt!(a);
   match_tt!(let);
   match_tt!(+);

   match_ident!(a);
   match_ident!(bcd);
   match_ident!(_def);

   match_expr!(1.2);
   match_expr!(bcd);
   match_expr!(1.2 + bcd / "b" - [1, 3, 4] .. vec![1, 2, 3]);

   match_ty!(A);
   match_ty!(B + 'static);
   match_ty!(A<&(B + 'b),&mut (C + 'c)> + 'static);

   match_stmt!(let x = y);
   match_stmt!(());
   match_stmt!(fn f(){});

   match_block!({});
   match_block!({1; 2});
   match_block!({1; 2 + 3});

   match_item!(struct A(u64););
   match_item!(enum B { C, D });
   match_item!(fn C(n: NotAType) -> F<F<F<F<F>>>> { a + b });

   match_pat!(_);
   match_pat!(1);
   match_pat!(A {b, c:D( d@3 )} );

   match_path!(A);
   match_path!(::A);
   match_path!(std::A);
   match_path!(a::<A,_>);

   match_meta!(A);   
   match_meta!(Property(B,C));
}
