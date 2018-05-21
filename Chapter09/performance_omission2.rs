use std::mem::forget;

fn main() {
   for _ in 0..10000 {
      let mut a = vec![2; 10000000];
      a[2] = 2;
      forget(a);
   }
}
