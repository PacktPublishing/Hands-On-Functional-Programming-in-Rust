#[macro_use] extern crate cached;
#[macro_use] extern crate lazy_static;

trait Monad<A> {
    fn return_(t: A) -> Self;
    //:: A -> Monad<A>

    fn bind<MB,B>(m: Self, f: Fn(A) -> MB) -> MB
    where MB: Monad<B>;
    //:: Monad<A> -> (A -> Monad<B>)) -> Monad<B>
}

fn not_curried(p1: u32, p2: u32) -> u32
{
    p1 + p2
}


fn curried(p1: u32) -> Box<Fn(u32) -> u32>
{
    Box::new(move |p2: u32| {
        p1 + p2
    })
}

cached!{
    FIB;
    fn fib(n: u64) -> u64 = {
        if n == 0 || n == 1 { return n }
        fib(n-1) + fib(n-2)
    }
}

fn main()
{
    let fsin = |x: f64| x.sin();
    let fabs = |x: f64| x.abs();
    let transform = |x: f64| fabs(fsin(x));

    not_curried(1, 2);
    curried(1)(2);

    let immutable_v1 = 1;
    //immutable_v1 = 2; //invalid
    let mut mutable_v2 = 1;
    mutable_v2 = 2;

    let x = { println!("side effect"); 1 + 2 };
    let y = ||{ println!("side effect"); 1 + 2 };

    fib(30);
}
