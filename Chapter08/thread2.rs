use std::{thread,time};
use std::sync::{Mutex, Arc};

fn main() {
   let mut big_data: Vec<u8> = Vec::with_capacity(200000000);
   big_data.push(1);
   big_data.push(2);
   big_data.push(3);
   let big_data = Arc::new(Mutex::new(big_data));

   for _ in 0..512 {
      let big_data = Arc::clone(&big_data);
      thread::spawn(move || {
         let t = time::Duration::from_millis(1000);
         loop {
            let d = big_data.lock().unwrap();
            (*d)[2];
            thread::sleep(t);
         }
      });
   }

   let t = time::Duration::from_millis(1000);
   loop {
      thread::sleep(t);
   }
}
