extern crate flame;
use std::fs::File;
use std::{thread,time};

fn a() {
   flame::start("fn a");
   let t = time::Duration::from_millis(1000);
   thread::sleep(t);
   b();
   b();
   b();
   flame::end("fn a");   
}

fn b() {
   flame::start("fn b");
   let t = time::Duration::from_millis(1000);
   thread::sleep(t);
   c();
   c();
   c();
   flame::end("fn b");   
}

fn c() {
   flame::start("fn c");
   let t = time::Duration::from_millis(1000);
   thread::sleep(t);
   flame::end("fn c");
}

fn main() {
   flame::start("fn main");
   let t = time::Duration::from_millis(1000);
   thread::sleep(t);
   a();
   a();
   a();
   flame::end("fn main");

   flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
