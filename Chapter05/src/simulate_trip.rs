extern crate elevator;
extern crate floating_duration;

use elevator::buildings::{Building, Building1, Building2, Building3};
use elevator::trip_planning::{FloorRequests, RequestQueue};
use elevator::physics::{ElevatorState, simulate_elevator};
use elevator::motion_controllers::{SmoothMotionController, MotionController};
use elevator::data_recorders::{newSimpleDataRecorder, DataRecorder};

use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::cmp;

pub fn run_simulation()
{

   //1. Store location, velocity, and acceleration state
   //2. Store motor input target force
   let mut est = ElevatorState {
      timestamp: 0.0,
      location: 0.0,
      velocity: 0.0,
      acceleration: 0.0,
      motor_input: 0.0
   };

   //3. Store input building description and floor requests
   let mut esp: Box<Building> = Box::new(Building1);
   let mut floor_requests: Box<RequestQueue> = Box::new(FloorRequests {
      requests: Vec::new()
   });

   //4. Parse input and store as building description and floor requests
   match env::args().nth(1) {
      Some(ref fp) if *fp == "-".to_string()  => {
         let mut buffer = String::new();
         io::stdin().read_to_string(&mut buffer)
                    .expect("read_to_string failed");
        
         for (li,l) in buffer.lines().enumerate() {
            if li==0 {
               let building = l.parse::<u64>().unwrap();
               if building==0 {
                  esp = Box::new(Building1);
               } else if building==1 {
                  esp = Box::new(Building2);
               } else if building==2 {
                  esp = Box::new(Building3);
               } else {
                  panic!("unknown building code: {}", building);
               }
            } else {
               floor_requests.add_request(l.parse::<u64>().unwrap());
            }
         }
      },
      None => {
         let fp = "test1.txt";
         let mut buffer = String::new();
         File::open(fp)
              .expect("File::open failed")
              .read_to_string(&mut buffer)
              .expect("read_to_string failed");

         for (li,l) in buffer.lines().enumerate() {
            if li==0 {
               let building = l.parse::<u64>().unwrap();
               if building==0 {
                  esp = Box::new(Building1);
               } else if building==1 {
                  esp = Box::new(Building2);
               } else if building==2 {
                  esp = Box::new(Building3);
               } else {
                  panic!("unknown building code: {}", building);
               }
            } else {
               floor_requests.add_request(l.parse::<u64>().unwrap());
            }
         }
      },
      Some(fp) => {
         let mut buffer = String::new();
         File::open(fp)
              .expect("File::open failed")
              .read_to_string(&mut buffer)
              .expect("read_to_string failed");

         for (li,l) in buffer.lines().enumerate() {
            if li==0 {
               let building = l.parse::<u64>().unwrap();
               if building==0 {
                  esp = Box::new(Building1);
               } else if building==1 {
                  esp = Box::new(Building2);
               } else if building==2 {
                  esp = Box::new(Building3);
               } else {
                  panic!("unknown building code: {}", building);
               }
            } else {
               floor_requests.add_request(l.parse::<u64>().unwrap());
            }
         }
      }
   }

   let mut dr: Box<DataRecorder> = newSimpleDataRecorder(esp.clone());
   let mut mc: Box<MotionController> = Box::new(SmoothMotionController {
      timestamp: 0.0,
      esp: esp.clone()
   });

   simulate_elevator(esp, est, &mut floor_requests, &mut mc, &mut dr);
   dr.summary();

}

fn main()
{
   run_simulation()
}
