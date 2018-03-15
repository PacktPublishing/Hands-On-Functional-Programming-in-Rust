struct MyObject
{
    a: u32,
    b: f32,
    c: String
}

trait MyObjectTrait
{
    fn new(a: u32, b: f32, c: String) -> Self;

    fn get_a(&self) -> u32;
    fn get_b(&self) -> f32;
    fn get_c(&self) -> String;
}

impl MyObjectTrait for MyObject
{
    fn new(a: u32, b: f32, c: String) -> Self
    {
        MyObject { a: a, b: b, c: c }
    }
    fn get_a(&self) -> u32
    {
        self.a
    }
    fn get_b(&self) -> f32
    {
        self.b
    }
    fn get_c(&self) -> String
    {
        self.c.clone()
    }
}

trait MyObjectApply {
    fn apply<T,R>(&self, f: T) -> R
    where T: Fn(u32,f32,String) -> R;
}

impl MyObjectApply for MyObject {
    fn apply<T,R>(&self, f: T) -> R
    where T: Fn(u32,f32,String) -> R
    {
        f(self.a, self.b, self.c.clone())
    }
}

fn main() {
}
