use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;

fn scoped() {
    vec![1, 2, 3];
}

fn scoped2() -> Vec<u32> {
    vec![1, 2, 3]
}

fn scoped3() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    //it is now illegal to reference v1,
    //because ownership has been transferred to v2
}

fn scoped4() {
    vec![1, 2, 3].clone();
    "".to_string().clone();
}

fn scoped5() {
    fn foo(v1: &Vec<u32>){}
    let v1 = vec![1, 2, 3];
    foo(&v1);
    //v1 is still valid, ownership has been returned
    v1;
}

fn thread1() {
   let v = vec![1, 2, 3];
   let handle = thread::spawn(move || {
      println!("Here's a vector: {:?}", v);
   });
   handle.join();
}

fn thread2()
{
   let counter = Arc::new(Mutex::new(0));
   let mut handles = vec![];
   for _ in 0..10 {
      let counter = Arc::clone(&counter);
      let handle = thread::spawn(move || {
         let mut num = counter.lock().unwrap();
         *num += 1;
      });
      handles.push(handle);
   }
   for handle in handles {
      handle.join().unwrap();
   }
   println!("Result: {}", *counter.lock().unwrap());
}

fn thread3() {
   let (sender, receiver) = channel();
   let handle = thread::spawn(move || {
      //do work
      let v = vec![1, 2, 3];
      sender.send(v).unwrap();
   });
   handle.join();
   receiver.recv().unwrap();
}

fn main() {
}
