use std::{thread,time,process};

fn main() {
   let life_expectancy = process::id() % 8;
   let t = time::Duration::from_millis(1000);
   for _ in 0..life_expectancy {
      thread::sleep(t);
   }
   println!("process {} dies unexpectedly.", process::id());
}
