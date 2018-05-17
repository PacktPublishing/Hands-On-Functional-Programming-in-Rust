use std::thread;

fn main() {
   static A: [u8; 100] = [22; 100];

   thread::spawn(|| {
      A[3];
   });

   thread::spawn(|| {
      A[3]
   });
}
