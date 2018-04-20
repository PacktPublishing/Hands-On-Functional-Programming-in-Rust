use buildings::{Building,getCumulativeFloorHeight};
use trip_planning::{RequestQueue};
use motion_controllers::{MotionController};
use data_recorders::{DataRecorder};
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use std::{thread, time};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ElevatorState {
   pub timestamp: f64,
   pub location: f64,
   pub velocity: f64,
   pub acceleration: f64,
   pub motor_input: f64
}

pub const MAX_JERK: f64 = 0.2;
pub const MAX_ACCELERATION: f64 = 2.0;
pub const MAX_VELOCITY: f64 = 5.0;

pub fn simulate_elevator(esp: Box<Building>, est: ElevatorState, floor_requests: &mut Box<RequestQueue>,
                         mc: &mut Box<MotionController>, dr: &mut Box<DataRecorder>)
{
   //immutable input becomes mutable local state
   let mut esp = esp.clone();
   let mut est = est.clone();

   //initialize MotorController and DataController
   mc.init(esp.clone(), est.clone());
   dr.init(esp.clone(), est.clone());

   //5. Loop while there are remaining floor requests
   let original_ts = Instant::now();
   thread::sleep(time::Duration::from_millis(1));
   let mut next_floor = floor_requests.pop_request();
   while let Some(dst) = next_floor
   {
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

      //5.4. Print realtime statistics
      dr.poll(est.clone(), dst);

      //5.3. Adjust motor control to process next floor request
      est.motor_input = mc.poll(est.clone(), dst);

      thread::sleep(time::Duration::from_millis(1));
   }

}
