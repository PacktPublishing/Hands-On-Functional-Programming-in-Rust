//Data Type Definitions
struct PointU32
{
    x: u32,
    y: u32
}
struct PointF32
{
    x: f32,
    y: f32
}

struct PointI32
{
    x: i32,
    y: i32
}

//can be written with generics
struct Point<T>
{
    x: T,
    y: T
}

//Function Definitions
fn foo_u32(x: u32) -> u32
{
    x*x
}
fn foo_f32(x: f32) -> f32
{
    x*x
}

//can be written with generics
fn foo<T>(x: T) -> T
   where T: std::ops::Mul<Output = T> + Copy
{
    x*x
}

//even functions can be sent to generics
//we call these "higher order functions"
fn bar<F,T>(f: F, x: T) -> T
   where F: Fn(T) -> T
{
    f(x)
}

fn main()
{
    PointU32 { x:1, y:1 };
    PointF32 { x:1.0, y:1.0 };
    Point { x:1, y:1 };
    Point { x:1.0, y:1.0 };
    foo_u32(1);
    foo_f32(1.0);
    foo(1);
    foo(1.0);
    bar(|x|{x}, 1);
}
