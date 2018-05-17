use std::thread;
use std::sync::{Arc};

fn main() {
   let a = Arc::new(vec![1, 2, 3]);

   {
      let a = Arc::clone(&a);
      thread::spawn(move || {
         a[1];
      });
   }

   {
      let a = Arc::clone(&a);
      thread::spawn(move || {
         a[1];
      });
   }
}
