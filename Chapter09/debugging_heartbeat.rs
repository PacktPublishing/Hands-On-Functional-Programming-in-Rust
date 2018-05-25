use std::process::Command;
use std::env::current_exe;
use std::{thread,time};

fn main() {
   let path = current_exe()
             .expect("could not find current executable");
   let path = path.with_file_name("debugging_buggy_worker");

   let mut children = Vec::new();
   for _ in 0..3 {
      children.push(
         Command::new(path.as_os_str())
                 .spawn()
                 .expect("failed to spawn child")
      );
   }

   let t = time::Duration::from_millis(1000);
   loop {
      thread::sleep(t);
      for ci in 0..children.len() {
         let is_dead = children[ci].try_wait().expect("failed to try_wait");
         if let Some(_exit_code) = is_dead {
            children[ci] = Command::new(path.as_os_str())
                                   .spawn()
                                   .expect("failed to spawn child");
            println!("starting a new process from parent.");
         }
      }
   }
}
