use std::fmt::{Debug};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

struct ServerMonad<St> {
  state: St,
  handle: Box<Fn(&mut St,String) -> bool>
}
impl<St: Clone> ServerMonad<St> {
   fn _return(&self, st: St) -> ServerMonad<St> {
      ServerMonad {
        state: st,
        handle: Box::new(|st: &mut St, s| false)
      }
   }
   fn listen(&self, address: &str) {
      let listener = TcpListener::bind(address).unwrap();

      for stream in listener.incoming() {
         let mut st = self.state.clone();
         let mut buffer = String::new();
         stream.unwrap().read_to_string(&mut buffer);
         (self.handle)(&mut st,buffer);
      }
   }
}

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
