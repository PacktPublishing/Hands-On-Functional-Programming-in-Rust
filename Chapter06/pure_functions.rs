#[macro_use] extern crate lazy_static;
#[macro_use] extern crate cached;
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::cell::Cell;

fn p0() {}

fn p1() -> u64 {
   444
}

fn p2(x: u64) -> u64 {
   x * 444
}

fn p3(x: u64, y: u64) -> u64 {
   x * 444 + y
}

static mut blah: u64 = 3;
fn ip0() {
   unsafe {
      blah = 444;
   }
}

fn ip1(c: &Cell<u64>) {
   c.set(333);
}

cached!{
   FIB;
   fn fib(n: u64) -> u64 = {
      if n == 0 || n == 1 { return n }
      fib(n-1) + fib(n-2)
   }
}

lazy_static! {
   static ref BUCKET_COUNTER: Mutex<HashMap<u64, u64>> = {
      Mutex::new(HashMap::new())
   };    
}
cached!{
   BUCK;
   fn bucket_count(n: u64) -> u64 = {
      let mut counter = BUCKET_COUNTER.lock().unwrap();
      let r = match counter.get(&n) {
        Some(c) => { c+1 }
        None => { 1 }
      };
      counter.insert(n, r);
      r
   }
}

#[derive(Clone)]
pub struct TimeBomb {
   countdown: Arc<Mutex<i32>>
}
impl Drop for TimeBomb
{
   fn drop(&mut self) {
      let mut c = self.countdown.lock().unwrap();
      *c -= 1;
      if *c <= 0 {
         panic!("BOOM!!")
      }
   }
}
cached!{
   TICKING_BOX;
   fn tick_tock(v: i32) -> TimeBomb = {
      TimeBomb {
         countdown: Arc::new(Mutex::new(v))
      }
   }
}

fn main()
{
   p0();
   p1();
   p2(3);
   p3(3,4);

   ip0();
   let r = Cell::new(3);
   ip1(&r);
   ip1(&r);

   fib(30); //call 1, generates correct value and returns it
   fib(30); //call 2, finds correct value and returns it

   bucket_count(30); //call 1, generates correct value and returns it
   bucket_count(30); //call 2, finds stale value and returns it

   tick_tock(3);
   tick_tock(3);
   tick_tock(3);
}
