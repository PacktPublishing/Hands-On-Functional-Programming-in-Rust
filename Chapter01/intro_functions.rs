fn main() {

    (0..10).map(|x| x*x);

    (0..10).map(|x| {

       fn f(y: u32) -> u32 {
           y*y
       }

       let z = f(x+1) * f(x+2);

       z*z

    });

    fn f<T>(g: T, x: u32) -> u32
    where T: Fn(u32) -> u32
    {
        g(x+1) * g(x+2)
    }
    f(|x|{x*x}, 2);

    (0..10).map(|x| x*x)
           .inspect(|x|{ println!("value {}", *x) })
           .filter(|x| *x<3)
           .filter_map(|x| Some(x))
           .fold(0, |x, y| x+y);

}
