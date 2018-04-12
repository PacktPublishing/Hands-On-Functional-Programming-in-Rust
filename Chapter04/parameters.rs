type TFoo<'a, A: 'a> = (&'a A, u64);

struct SFoo<'a, A: 'a>(&'a A);

struct SBar<'a, A: 'a>
{
   x: &'a A
}

enum EFoo<'a, A: 'a>
{
   X { x: &'a A },
   Y { y: &'a A },
}

struct SBaz<'a, 'b, A: 'a, B: 'b>
{
   a: &'a A,
   b: &'b B,
}

trait TBaz<'a, 'b, A: 'a, B: 'b>
{
   fn baz(&self);
}

impl<'a, 'b, A: 'a, B: 'b> TBaz<'a, 'b, A, B> for SBaz<'a, 'b, A, B>
{
   fn baz(&self){}
}

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

fn main()
{
   let b = Baz;

   Foo::f(&b);
   Bar::f(&b);

   <Baz as Foo>::f(&b);
   <Baz as Bar>::f(&b);
}
