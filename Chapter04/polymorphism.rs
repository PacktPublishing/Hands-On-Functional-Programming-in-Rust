use std::ops::Mul;

fn raise_by_three<T: Mul + Copy>(x: T) -> T
where T: std::ops::Mul<Output=T>
{
   x * x * x
}

#[derive(Copy, Clone)]
struct Raiseable<T: Mul + Copy>
{
  x: T
}
impl<T: Mul + Copy> std::ops::Mul for Raiseable<T>
where T: std::ops::Mul<Output=T>
{
   type Output = Raiseable<T>;
   fn mul(self, rhs: Self) -> Self::Output
   {
      Raiseable { x: self.x * rhs.x }
   }
}

fn foo<X>(x: X) -> X
{
   x
}

fn bar<X>(f: fn(X) -> X, x: X) -> X
{
   f(x)
}

fn baz<X,F>(f: F, x: X) -> X
where F: Fn(X) -> X
{
   f(x)
}

fn main()
{

   raise_by_three(10);
   (10 as u64).pow(3);

   raise_by_three(3.0);
   (3.0 as f64).powi(3);

   let x = Raiseable { x: 10 as u64 };
   raise_by_three(x);
   //no method named pow
   //x.pow(3);

   let x = Raiseable { x: 3.0 as f64 };
   raise_by_three(x);
   //no method named powi
   //x.powi(3);

   foo(1);
   bar(foo,1);

   baz(|x| x, 1);
   baz(|x| foo(x), 1);
}
