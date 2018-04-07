mod physics;
mod motor;

use physics::{ElevatorSpecification, ElevatorState, MotorInput, simulate_elevator, DataRecorder, MotorController, MotorVoltage};

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate floating_duration;
use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader};
use std::io::prelude::*;
use std::cmp;

fn main()
{
   let simlog = File::open("simulation.log").expect("read simulation log");
   let mut simlog = BufReader::new(&simlog);
   let mut esp = None;
   for line in simlog.lines() {
      let l = line.unwrap();
      match esp.clone() {
         None => {
            let spec: ElevatorSpecification = serde_json::from_str(&l).unwrap();
            esp = Some(spec);
         },
         Some(esp) => {
            let (est, dst): (ElevatorState,u64) = serde_json::from_str(&l).unwrap();
         }
      }
   }
}
