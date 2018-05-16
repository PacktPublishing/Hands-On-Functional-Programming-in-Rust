extern crate nix;
use nix::unistd::{fork};
use std::{thread,time};

fn main() {
   let mut big_data: Vec<u8> = Vec::with_capacity(200000000);
   big_data.push(1);
   big_data.push(2);
   big_data.push(3);

   //Both sides of the fork, will continue to fork
   //This is called a fork bomb
   for _ in 0..9 {
      fork().expect("fork failed");
   }
   //2^9 = 512

   let t = time::Duration::from_millis(1000);
   loop {
      big_data[2];
      thread::sleep(t);
   }
}
