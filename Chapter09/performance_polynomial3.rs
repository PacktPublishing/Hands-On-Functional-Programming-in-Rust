fn a(n: u64) {
   //Is this O(n)?
   for _ in 0..n {
      b(n)
   }
}

fn b(n: u64) {
   //Is this O(n)?
   for _ in 0..n {
      c(n)
   }
}

fn c(n: u64) {
   //This is O(n)?
   for _ in 0..n {
      let _ = 1 + 1;
   }
}

fn main() {
   //What time complexity is this?
   a(1000)
}
