use std::{thread,time};

fn initialization() -> Vec<i32> {
   let t = time::Duration::from_millis(15000);
   thread::sleep(t);
   println!("Initialize data.");
   vec![1, 2, 3]
}

fn work(x: i32) -> i32 {
   let t = time::Duration::from_millis(150);
   thread::sleep(t);
   println!("Work.");
   x * x
}

fn main() {

   for _ in 0..10 {
      let data = initialization();
      data.iter().map(|x| work(*x)).for_each(drop);
   }
   
}
