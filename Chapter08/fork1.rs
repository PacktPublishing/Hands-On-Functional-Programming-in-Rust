extern crate nix;
use nix::unistd::{fork,ForkResult};
use std::{thread,time};
use std::process;

fn main() {
   let mut children = Vec::new();
   for _ in 0..3 {
      match fork().expect("fork failed") {
         ForkResult::Parent{ child: pid } => { children.push(pid); }
         ForkResult::Child => {
            let t = time::Duration::from_millis(1000);
            loop {
               println!("child process #{}", process::id());
               thread::sleep(t);
            }
         }
      }
   }

   let t = time::Duration::from_millis(1000);
   loop {
      println!("parent process #{}", process::id());
      thread::sleep(t);
   }
}
