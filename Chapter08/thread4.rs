use std::{thread,time};
use std::process;
use std::io::prelude::*;
extern crate thread_id;
use std::net::{TcpListener,TcpStream};
use std::sync::mpsc::{channel,Receiver};
use std::collections::VecDeque;

fn serve(receiver: Receiver<TcpStream>) {
   let t = time::Duration::from_millis(10);
   loop {
      let mut tcp = receiver.recv().unwrap();
      let mut buffer = [0; 2048];
      tcp.read(&mut buffer).expect("tcp read failed");
      let response = format!("respond from #{}:{}\n", process::id(), thread_id::get());
      tcp.write(response.as_bytes()).expect("tcp write failed");
      thread::sleep(t);
   }
}

fn main() {
   let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
   let mut channels = VecDeque::new();

   for _ in 0..3 {
      let (sender, receiver) = channel();
      channels.push_back(sender);
      thread::spawn(move || {
         serve(receiver);
      });
   }

   for stream in listener.incoming() {
      let round_robin = channels.pop_front().unwrap();
      round_robin.send(stream.unwrap()).unwrap();
      channels.push_back(round_robin);
   }
}
