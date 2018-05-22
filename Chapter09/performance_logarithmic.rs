extern crate rand;
extern crate flame;
use std::fs::File;

fn main() {

   let mut data = vec![0; 1000];
   for di in 0..data.len() {
      data[di] = rand::random::<u64>();
   }

   flame::start("sort n=1000");
   data.sort();
   flame::end("sort n=1000");

   flame::start("binary search n=1000 100 times");
   for _ in 0..100 {
      let c = rand::random::<u64>();
      data.binary_search(&c).ok();
   }
   flame::end("binary search n=1000 100 times");

   let mut data = vec![0; 10000];
   for di in 0..data.len() {
      data[di] = rand::random::<u64>();
   }

   flame::start("sort n=10000");
   data.sort();
   flame::end("sort n=10000");

   flame::start("binary search n=10000 100 times");
   for _ in 0..100 {
      let c = rand::random::<u64>();
      data.binary_search(&c).ok();
   }
   flame::end("binary search n=10000 100 times");

   flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

}
