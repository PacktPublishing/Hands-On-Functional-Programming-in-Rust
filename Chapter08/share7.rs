use std::thread;

struct MyBox(u8);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

static A: MyBox = MyBox(22);

fn main() {
   thread::spawn(move || {
      A.0
   });

   thread::spawn(move || {
      A.0
   });
}
