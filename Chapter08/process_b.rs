use std::{thread,time};
use std::process;

fn main() {
   let t = time::Duration::from_millis(1000);
   loop {
      println!("process b #{}", process::id());
      thread::sleep(t);
   }
}
