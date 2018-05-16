extern crate nix;
use nix::unistd::{fork,ForkResult};
use std::{thread,time};
use std::process;
use std::io::prelude::*;
use std::net::TcpListener;

fn serve(listener: TcpListener) -> ! {
   for stream in listener.incoming() {
      let mut buffer = [0; 2048];
      let mut tcp = stream.unwrap();
      tcp.read(&mut buffer).expect("tcp read failed");
      let response = format!("respond from #{}\n", process::id());
      tcp.write(response.as_bytes()).expect("tcp write failed");
   }
   panic!("unreachable");
}

fn main() {
   let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
   let mut children = Vec::new();

   for _ in 0..3 {
      match fork().expect("fork failed") {
         ForkResult::Parent{ child: pid } => { children.push(pid); }
         ForkResult::Child => {
            serve(listener)
         }
      }
   }

   let t = time::Duration::from_millis(1000);
   loop {
      thread::sleep(t);
   }
}
