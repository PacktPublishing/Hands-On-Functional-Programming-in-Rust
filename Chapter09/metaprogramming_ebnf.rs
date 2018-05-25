//this is a grammar sequence
macro_rules! abc {
  (a b c) => { println!("'a b c' is the only correct syntax.") };
}

//this is a grammar alternative
macro_rules! a_or_b {
  (a) => { println!("'a' is one correct syntax.") };
  (b) => { println!("'b' is also correct syntax.") };
}

//this is a grammar alternative
macro_rules! abc_or_aaa {
  (a b c) => { println!("'a b c' is one correct syntax.") };
  (a a a) => { println!("'a a a' is also correct syntax.") };
}

//this is a grammar sequence matching many of one token
macro_rules! many_a {
  ( $($a:ident)* ) => {{ $( print!("one {} ", stringify!($a)); )* println!(""); }};
  ( $($a:ident),* ) => {{ $( print!("one {} comma ", stringify!($a)); )* println!(""); }};
}

fn main() {
   abc!(a b c);

   a_or_b!(a);
   a_or_b!(b);

   abc_or_aaa!(a b c);
   abc_or_aaa!(a a a);

   many_a!(a a a);
   many_a!(a, a, a);
}
