fn bomb(n: u64) -> u64 {
   if n > 0 {
      bomb(n-1);
      bomb(n-1);
   }
   n
}

fn main() {
   bomb(1000);
}
