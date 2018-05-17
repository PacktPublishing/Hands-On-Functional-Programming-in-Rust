use std::thread;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::time;
extern crate rand;

fn new_ping() -> (Sender<u64>, Receiver<u64>) {
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

fn new_pong() -> (Sender<u64>, Receiver<u64>) {
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
   let pings = vec![new_ping(), new_ping(), new_ping()];
   let pongs = vec![new_pong(), new_pong(), new_pong()];
   loop {
      let mut d = 3;

      let (ref pingin,ref pingout) = pings[(rand::random::<u64>() % 3) as usize];
      pingin.send(d).unwrap();
      d = pingout.recv().unwrap();

      let (ref pongin,ref pongout) = pongs[(rand::random::<u64>() % 3) as usize];
      pongin.send(d).unwrap();
      pongout.recv().unwrap();
   }
}
