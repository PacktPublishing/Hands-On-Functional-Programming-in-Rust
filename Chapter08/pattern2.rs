use std::thread;
use std::sync::mpsc::{channel};
use std::time;

fn new_ping() {
   let (pinginsend,pinginrecv) = channel();
   let (pingoutsend,pingoutrecv) = channel();
   let mut ping = 1;
   thread::spawn(move || {
      let t = time::Duration::from_millis(1000);
      loop {
         let n = pinginrecv.recv().unwrap();
         ping += n;
         println!("ping {}", ping);
         thread::sleep(t);
         pingoutsend.send(ping).unwrap();
      }
   });
   (pinginsend, pingoutrecv)
}

fn new_pong() {
   let (ponginsend,ponginrecv) = channel();
   let (pongoutsend,pongoutrecv) = channel();
   let mut pong = 2;
   thread::spawn(move || {
      let t = time::Duration::from_millis(1000);
      loop {
         let n = ponginrecv.recv().unwrap();
         pong += n;
         println!("pong {}", pong);
         thread::sleep(t);
         pongoutsend.send(pong).unwrap();
      }
   });
   (ponginsend, pongoutrecv)
}

fn main() {
   let mut d = 3;
   let pings = vec![ping_new(), ping_new(), ping_new()];
   let pongs = vec![pong_new(), pong_new(), pong_new()];
   loop {
      pinginsend.send(d).unwrap();
      d = pingoutrecv.recv().unwrap();
      ponginsend.send(d).unwrap();
      d = pongoutrecv.recv().unwrap();
   }
}
