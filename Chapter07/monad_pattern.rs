use std::fmt::{Debug};

struct LogMonad<T>(T);
impl<T> LogMonad<T> {
   fn _return(t: T) -> LogMonad<T>
      where T: Debug {
      println!("{:?}", t);
      LogMonad(t)
   }
   fn bind<R,F>(&self, f: F) -> LogMonad<R>
      where F: Fn(&T) -> R,
            R: Debug {
      let r = f(&self.0);
      println!("{:?}", r);
      LogMonad(r)
   }
}

struct LazyMonad<A,B>(Box<Fn(A) -> B>);
impl<A: 'static,B: 'static> LazyMonad<A,B> {
   fn _return(u: A) -> LazyMonad<B,B> {
      LazyMonad(Box::new(move |b: B| b))
   }
   fn bind<C,G: 'static>(self, g: G) -> LazyMonad<A,C>
      where G: Fn(B) -> C {
      LazyMonad(Box::new(move |a: A| g(self.0(a))))
   }
   fn apply(self, a: A) -> B {
      self.0(a)
   }
}

fn main() 
{
   let v1 = Some(2).and_then(|x| Some(x+x)).and_then(|y| Some(y*y));
   println!("{:?}", v1);

   let v2 = None.or_else(|| None).or_else(|| Some(222));
   println!("{:?}", v2);

   let v3 = Some(2).and_then(|x| Some("abc"));
   println!("{:?}", v3);

   // or_else is not quite a monad
   // does not permit polymorphic bind
   //let v4 = Some(2).or_else(|| Some("abc"));
   //println!("{:?}", v4);

   LogMonad::_return(4)
            .bind(|x| x+x)
            .bind(|y| y*y)
            .bind(|z| format!("{}{}{}", z, z, z));

   let notyet = LazyMonad::_return(())
                          .bind(|x| x+2)
                          .bind(|y| y*3)
                          .bind(|z| format!("{}{}", z, z));

   let nowdoit = notyet.apply(222);
   println!("nowdoit {}", nowdoit)
}
