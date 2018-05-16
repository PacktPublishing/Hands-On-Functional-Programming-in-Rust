use std::thread;

fn main() {
   let a = vec![1, 2, 3];

   thread::spawn(move || {
      println!("a = {:?}", a);
   });
}
