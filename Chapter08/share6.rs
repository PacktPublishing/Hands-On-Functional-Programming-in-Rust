use std::thread;
use std::sync::{Arc,Mutex};

fn main() {
   let a = Arc::new(Mutex::new(vec![1, 2, 3]));
   {
      let a = Arc::clone(&a);
      thread::spawn(move || {
         let mut a = a.lock().unwrap();
         (*a)[1] = 2;
      });
   }

   {
      let a = Arc::clone(&a);
      thread::spawn(move || {
         let mut a = a.lock().unwrap();
         (*a)[1] = 3;
      });
   }
}
