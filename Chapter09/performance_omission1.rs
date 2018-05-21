extern crate flame;
use std::fs::File;

fn main() {
   let v: Vec<u64> = vec![2; 1000000];

   flame::start("Iterator .collect");
   let mut _z = vec![];
   for _ in 0..1000 {
      _z = v.iter().map(|x| x*x).collect::<Vec<u64>>();
   }
   flame::end("Iterator .collect");

   flame::start("Iterator iterate");
   for _ in 0..1000 {
      v.iter().map(|x| x * x).for_each(drop);
   }
   flame::end("Iterator iterate");

   flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
