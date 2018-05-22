use std::{thread,time};

fn initialization() {
   let t = time::Duration::from_millis(15000);
   thread::sleep(t);
}

fn work() {
   let t = time::Duration::from_millis(15000);
   loop {
      thread::sleep(t);
      println!("Work.");
   }
}

fn main() {
   initialization();
   println!("Done initializing, start work.");
   work();
}
