#[macro_use]
extern crate metaderive;

macro_rules! my_vec_macro {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! my_macro_branch
{
    (1 $e:expr) => (println!("mode 1: {}", $e));
    (2 $e:expr) => (println!("mode 2: {}", $e));
}

enum DSLTerm {
    TVar { symbol: String },
    TAbs { param: String, body: Box<DSLTerm> },
    TApp { f: Box<DSLTerm>, x: Box<DSLTerm> }
}

macro_rules! dsl
{
    ( ( $($e:tt)* ) ) => (dsl!( $($e)* ));
    ( $e:ident ) => (DSLTerm::TVar { symbol: stringify!($e).to_string() });
    ( fn $p:ident . $b:tt ) => (DSLTerm::TAbs { param: stringify!($p).to_string(), body: Box::new(dsl!($b)) });
    ( $f:tt $x:tt ) => (DSLTerm::TApp { f: Box::new(dsl!($f)), x: Box::new(dsl!($x)) });
}

pub trait TypeName {
    fn typename() -> String;
}

#[derive(TypeName)]
struct MyStructA
{
    a: u32,
    b: f32
}

fn main()
{

    println!("this is a macro {} {}", 1, 2);

    my_vec_macro!(1, 2, 3);

    my_macro_branch!(1 "abc");
    my_macro_branch!(2 "def");

    dsl!( a );
    dsl!( fn a . a );
    dsl!( f a );
    dsl!( (f a) );
}
