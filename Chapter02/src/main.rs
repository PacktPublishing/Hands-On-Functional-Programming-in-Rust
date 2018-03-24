extern crate floating_duration;
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use std::time::SystemTime;
use std::{thread, time};

fn main()
{

   //1. Store location, velocity, and acceleration state
   let mut location: f64 = 0.0; // meters
   let mut velocity: f64 = 0.0; // meters per second
   let mut acceleration: f64 = 0.0; // meters per second squared

   //2. Store motor input voltage
   let mut up_input_voltage: f64 = 0.0;
   let mut down_input_voltage: f64 = 0.0;

   //3. Store input building description and floor requests
   let mut floor_count: u64 = 0;
   let mut floor_height: f64 = 0.0; // meters
   let mut floor_requests: Vec<u64> = Vec::new();

   //4. Parse input and store as building description and floor requests

   //5. Loop while there are remaining floor requests
   let mut prev_loop_time = SystemTime::now();
   while floor_requests.len() > 0
   {
      //5.1. Update location, velocity, and acceleration
      let dt = prev_loop_time.duration_since(prev_loop_time)
                             .expect("SystemTime::duration_since failed")
                             .as_fractional_secs();
      prev_loop_time = SystemTime::now();

      location = location + velocity * dt;
      velocity = velocity + acceleration * dt;
      acceleration = {
         let F = (up_input_voltage - down_input_voltage) * 8;
         let m = 1200000;
         -9.8 + F/m
      };

      //5.2. If next floor request in queue is satisfied, then remove from queue

      //5.3. Adjust motor control to process next floor request

      //5.4. Print realtime statistics

      thread::sleep(time::Duration::from_millis(10));
   }

   //6. Print summary
   println!("main");

}
