extern crate elevator;
extern crate floating_duration;

use elevator::buildings::{Building, Building1, Building2, Building3, getCumulativeFloorHeight};
use elevator::trip_planning::{FloorRequests, RequestQueue};
use elevator::physics::{ElevatorState, simulate_elevator};
use elevator::motion_controllers::{SmoothMotionController, MotionController};

use floating_duration::{TimeAsFloat, TimeFormat};
use std::{thread, time};
use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::cmp;

pub fn run_operator()
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

   //4. Parse input and store as building description and static floor requests
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

   let mut mc: Box<MotionController> = Box::new(SmoothMotionController {
      timestamp: 0.0,
      esp: esp.clone()
   });

   //initialize MotorController and DataController
   mc.init(esp.clone(), est.clone());

   //5. Loop while there are remaining floor requests
   let original_ts = Instant::now();
   thread::sleep(time::Duration::from_millis(1));
   let mut next_floor = floor_requests.pop_request();
   while true
   {
      if let Some(dst) = next_floor {
         //5.1. Update location, velocity, and acceleration
         let now = Instant::now();
         let ts = now.duration_since(original_ts)
                     .as_fractional_secs();
         let dt = ts - est.timestamp;
         est.timestamp = ts;

         est.location = est.location + est.velocity * dt;
         est.velocity = est.velocity + est.acceleration * dt;
         est.acceleration = {
            let F = est.motor_input;
            let m = esp.get_carriage_weight();
            -9.8 + F/m
         };

         //5.2. If next floor request in queue is satisfied, then remove from queue
         if (est.location - getCumulativeFloorHeight(esp.get_floor_heights(), dst)).abs() < 0.01 &&
            est.velocity.abs() < 0.01
         {
            est.velocity = 0.0;
            next_floor = floor_requests.pop_request();
         }

         //5.3. Adjust motor control to process next floor request
         est.motor_input = mc.poll(est.clone(), dst);

         //Adjust motor
         esp.get_motor_controller().adjust_motor(est.motor_input);

         thread::sleep(time::Duration::from_millis(1));
      } else {
         //Adjust motor to not move
         esp.get_motor_controller().adjust_motor(0.0);
      }

      //check for dynamic floor requests
      if let Some(dst) = esp.get_elevator_driver().poll_floor_request() {
         floor_requests.add_request(dst);
      }
   }

}

fn main()
{
   run_operator()
}
