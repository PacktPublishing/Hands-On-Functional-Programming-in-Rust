use std::{thread,time};
use std::process;
extern crate thread_id;

fn main() {
   for _ in 0..3 {
      thread::spawn(|| {
         let t = time::Duration::from_millis(1000);
         loop {
            println!("child thread #{}:{}", process::id(), thread_id::get());
            thread::sleep(t);
         }
      });
   }

   let t = time::Duration::from_millis(1000);
   loop {
      println!("parent thread #{}:{}", process::id(), thread_id::get());
      thread::sleep(t);
   }
}
