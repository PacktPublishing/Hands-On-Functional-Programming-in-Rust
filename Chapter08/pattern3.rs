use std::thread;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::time;
extern crate rand;

enum Address {
   Ping,
   Pong
}
enum Message {
   PingPlus(u64),
   PongPlus(u64),
}

fn new_ping() -> (Sender<Message>, Receiver<(Address,Message)>) {
   let (pinginsend,pinginrecv) = channel();
   let (pingoutsend,pingoutrecv) = channel();
   let mut ping = 1;
   thread::spawn(move || {
      let t = time::Duration::from_millis(1000);
      loop {
         let msg = pinginrecv.recv().unwrap();
         match msg {
            Message::PingPlus(n) => { ping += n; },
            _ => panic!("Unexpected message")
         }
         println!("ping {}", ping);
         thread::sleep(t);
         pingoutsend.send((
            Address::Pong,
            Message::PongPlus(ping)
         )).unwrap();
         pingoutsend.send((
            Address::Pong,
            Message::PongPlus(ping)
         )).unwrap();
      }
   });
   (pinginsend, pingoutrecv)
}

fn new_pong() -> (Sender<Message>, Receiver<(Address,Message)>) {
   let (ponginsend,ponginrecv) = channel();
   let (pongoutsend,pongoutrecv) = channel();
   let mut pong = 1;
   thread::spawn(move || {
      let t = time::Duration::from_millis(1000);
      loop {
         let msg = ponginrecv.recv().unwrap();
         match msg {
            Message::PongPlus(n) => { pong += n; },
            _ => panic!("Unexpected message")
         }
         println!("pong {}", pong);
         thread::sleep(t);
         pongoutsend.send((
            Address::Ping,
            Message::PingPlus(pong)
         )).unwrap();
         pongoutsend.send((
            Address::Ping,
            Message::PingPlus(pong)
         )).unwrap();
      }
   });
   (ponginsend, pongoutrecv)
}

fn main() {
   let pings = vec![new_ping(), new_ping(), new_ping()];
   let pongs = vec![new_pong(), new_pong(), new_pong()];

   //Start the action
   pings[0].0.send(Message::PingPlus(1)).unwrap();

   //This thread will be the router
   let t = time::Duration::from_millis(10);
   loop {
      let mut mail = Vec::new();

      for (_,r) in pings.iter() {
         for (addr,msg) in r.try_iter() {
            mail.push((addr,msg));
         }
      }
      for (_,r) in pongs.iter() {
         for (addr,msg) in r.try_iter() {
            mail.push((addr,msg));
         }
      }

      for (addr,msg) in mail.into_iter() {
         match addr {
            Address::Ping => {
               let (ref s,_) = pings[(rand::random::<u32>() as usize) % pings.len()];
               s.send(msg).unwrap();
            },
            Address::Pong => {
               let (ref s,_) = pongs[(rand::random::<u32>() as usize) % pongs.len()];
               s.send(msg).unwrap();
            }
         }
      }

      thread::sleep(t);
   }
}
