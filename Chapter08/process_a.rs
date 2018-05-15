use std::process::Command;
use std::env::current_exe;

fn main() {
   let mut path = current_exe()
                 .expect("could not find current executable");
   path.pop();
   path.push("process_b");

   let mut children = Vec::new();
   for _ in 0..3 {
      children.push(
         Command::new(path.as_os_str())
                 .spawn()
                 .expect("failed to execute process")
      );
   }

   for mut c in children {
      c.wait()
       .expect("failed to wait on child");
   }
}
