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

fn main()
{
   let simlog = File::open("simulation.log").expect("read simulation log");
   let mut simlog = BufReader::new(&simlog);
   let mut esp = None;
   let mut jerk = 0.0;
   let mut prev_est: Option<ElevatorState> = None;
   for line in simlog.lines() {
      let l = line.unwrap();
      match esp.clone() {
         None => {
            let spec: ElevatorSpecification = serde_json::from_str(&l).unwrap();
            esp = Some(spec);
         },
         Some(esp) => {
            let (est, dst): (ElevatorState,u64) = serde_json::from_str(&l).unwrap();

            if let Some(prev_est) = prev_est {
               let dt = est.timestamp - prev_est.timestamp;
               let da = (est.acceleration - prev_est.acceleration).abs();
               jerk = (jerk * (1.0 - dt)) + (da * dt);
               if jerk.abs() > 0.22 {
                  panic!("jerk is outside of acceptable limits: {} {:?}", jerk, est)
               }
            }
            if est.acceleration.abs() > 2.2 {
               panic!("acceleration is outside of acceptable limits: {:?}", est)
            }
            if est.velocity.abs() > 5.5 {
               panic!("velocity is outside of acceptable limits: {:?}", est)
            }
            prev_est = Some(est);

            //elevator should never backup
            //trips should finish within 20% of theoretical limit

         }
      }
   }
}
