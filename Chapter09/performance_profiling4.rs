fn a(n: u64) -> u64 {
   if n>0 {
      b(n);
      b(n);
   }
   n * n
}

fn b(n: u64) -> u64 {
   c(n);
   c(n);
   n + 2 / 3
}

fn c(n: u64) -> u64 {
   a(n-1);
   a(n-1);
   vec![1, 2, 3].into_iter().map(|x| x+2).sum()
}

fn main() {
   a(6);
}
