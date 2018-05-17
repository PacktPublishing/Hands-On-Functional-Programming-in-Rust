use std::thread;
#[macro_use] extern crate lazy_static;

lazy_static! {
   static ref A: Vec<u32> = {
      vec![1, 2, 3]
   };
}

fn main() {
   thread::spawn(|| {
      A[1];
   });

   thread::spawn(|| {
      A[2];
   });
}
