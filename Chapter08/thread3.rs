use std::{thread,time};
use std::process;
extern crate thread_id;
use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use std::sync::{Arc,Mutex};

fn serve(incoming: Arc<Mutex<Vec<TcpStream>>>) {
   let t = time::Duration::from_millis(10);
   loop {
      {
         let mut incoming = incoming.lock().unwrap();
         for stream in incoming.iter() {
            let mut buffer = [0; 2048];
            let mut tcp = stream;
            tcp.read(&mut buffer).expect("tcp read failed");
            let response = format!("respond from #{}:{}\n", process::id(), thread_id::get());
            tcp.write(response.as_bytes()).expect("tcp write failed");
         }
         incoming.clear();
      }
      thread::sleep(t);
   }
}

fn main() {
   let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
   let incoming = Vec::new();
   let incoming = Arc::new(Mutex::new(incoming));

   for _ in 0..3 {
      let incoming = Arc::clone(&incoming);
      thread::spawn(move || {
         serve(incoming);
      });
   }

   for stream in listener.incoming() {
      let mut incoming = incoming.lock().unwrap();
      (*incoming).push(stream.unwrap());
   }
}
