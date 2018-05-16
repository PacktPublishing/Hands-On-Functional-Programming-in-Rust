use std::{thread,time};
extern crate rand;
use std::sync::{Arc,Mutex};
#[macro_use] extern crate lazy_static;

lazy_static! {
   static ref NEURAL_NET_WEIGHTS: Vec<Arc<Mutex<Vec<f64>>>> = {
      let mut nn = Vec::with_capacity(10000);
      for _ in 0..10000 {
         let mut mm = Vec::with_capacity(100);
         for _ in 0..100 {
            mm.push(rand::random::<f64>());
         }
         let mm = Arc::new(Mutex::new(mm));
         nn.push(mm);
      }
      nn
   };
}

fn train() {
   let t = time::Duration::from_millis(100);
   loop {
      for _ in 0..100 {
         let update_position = rand::random::<u64>() % 1000000;
         let update_column = update_position / 10000;
         let update_row = update_position % 100;
         let update_value = rand::random::<f64>();
         let mut update_column = NEURAL_NET_WEIGHTS[update_column as usize].lock().unwrap();
         update_column[update_row as usize] = update_value;
      }
      thread::sleep(t);
   }
}

fn main() {
   let t = time::Duration::from_millis(1000);

   for _ in 0..500 {
      thread::spawn(train);
   }

   loop {
      thread::sleep(t);
   }
}
