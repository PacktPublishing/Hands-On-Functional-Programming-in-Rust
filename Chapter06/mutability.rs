use std::sync::{Mutex, Arc};

fn f(x: &mut i32) {
   *x = 2;
}

#[derive(Clone)]
struct TimeBomb {
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

fn main()
{
   let a = 5;
   let mut b = 5;
   
   //a = 4; not valid
   b = 4;

   //*(&mut a) = 3; not valid
   *(&mut b) = 3;

   let a = 5;
   let mut b = 5;

   //f(&mut a); not valid
   f(&mut b);

   {
      let t3 = TimeBomb {
         countdown: Arc::new(Mutex::new(3))
      };
      let t2 = t3.clone();
      let t1 = t2.clone();
      let t0 = t1.clone();
   }
}
