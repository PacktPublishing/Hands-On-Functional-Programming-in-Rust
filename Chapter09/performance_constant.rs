fn allocate() -> [u64; 1000] {
   [22; 1000]
}

fn flop(x: f64, y: f64) -> f64 {
   x * y
}

fn lookup(x: &[u64; 1000]) -> u64 {
   x[234] * x[345]
}

fn main() {
   let mut data = allocate();
   for _ in 0..1000 {
      //constant size memory allocation
      data = allocate();
   }

   for _ in 0..1000000 {
      //reference data
      lookup(&data);
   }

   for _ in 0..1000000 {
      //floating point operation
      flop(2.0, 3.0);
   }
}
