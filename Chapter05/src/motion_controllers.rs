use physics::{ElevatorState};
use buildings::{Building};

pub trait MotionController
{
   fn init(&mut self, esp: Box<Building>, est: ElevatorState);
   fn poll(&mut self, est: ElevatorState, dst: u64) -> f64;
}

pub struct SmoothMotionController
{
   pub esp: Box<Building>,
   pub timestamp: f64
}

fn getCumulativeFloorHeight(heights: Vec<f64>, floor: u64) -> f64
{
   let mut c = 0.0;
   for fi in 0..floor
   {
      c += heights[fi as usize];
   };
   c
}

impl MotionController for SmoothMotionController
{
   fn init(&mut self, esp: Box<Building>, est: ElevatorState)
   {
      self.esp = esp;
      self.timestamp = est.timestamp;
   }

   fn poll(&mut self, est: ElevatorState, dst: u64) -> f64
   {
      //5.3. Adjust motor control to process next floor request

      let MAX_JERK = 0.2;
      let MAX_ACCELERATION = 2.0;
      let MAX_VELOCITY = 5.0;

      //it will take t seconds to reach max from max
      let t_accel = MAX_ACCELERATION / MAX_JERK;
      let t_veloc = MAX_VELOCITY / MAX_ACCELERATION;

      //it may take up to d meters to decelerate from current
      let decel_t = if (est.velocity>0.0) == (est.acceleration>0.0) {
         //this case deliberately overestimates d to prevent "back up"
         (est.acceleration.abs() / MAX_JERK) +
         (est.velocity.abs() / (MAX_ACCELERATION / 2.0)) +
         2.0 * (MAX_ACCELERATION / MAX_JERK)
      } else {
         //without the MAX_JERK, this approaches infinity and decelerates way too soon
         //MAX_JERK * 1s = acceleration in m/s^2
         est.velocity.abs() / (MAX_JERK + est.acceleration.abs())
      };
      let d = est.velocity.abs() * decel_t;

      let dst_height = getCumulativeFloorHeight(self.esp.get_floor_heights(), dst);

      //l = distance to next floor
      let l = (est.location - dst_height).abs();


      let target_acceleration = {
         //are we going up?
         let going_up = est.location < dst_height;

         //time elapsed since last poll
         let dt = est.timestamp - self.timestamp;
         self.timestamp = est.timestamp;

         //Do not exceed maximum acceleration
         if est.acceleration.abs() >= MAX_ACCELERATION {
            if est.acceleration > 0.0 {
               est.acceleration - (dt * MAX_JERK)
            } else {
               est.acceleration + (dt * MAX_JERK)
            }

         //Do not exceed maximum velocity
         } else if est.velocity.abs() >= MAX_VELOCITY
            || (est.velocity + est.acceleration * (est.acceleration.abs() / MAX_JERK)).abs() >= MAX_VELOCITY {
            if est.velocity > 0.0 {
               est.acceleration - (dt * MAX_JERK)
            } else {
               est.acceleration + (dt * MAX_JERK)
            }

         //if within comfortable deceleration range and moving in right direction, decelerate
         } else if l < d && (est.velocity>0.0) == going_up {
            if going_up {
               est.acceleration - (dt * MAX_JERK)
            } else {
               est.acceleration + (dt * MAX_JERK)
            }

         //else if not at peak velocity, accelerate smoothly
         } else {
            if going_up {
               est.acceleration + (dt * MAX_JERK)
            } else {
               est.acceleration - (dt * MAX_JERK)
            }
         }
      };

     let gravity_adjusted_acceleration = target_acceleration + 9.8;
     let target_force = gravity_adjusted_acceleration * self.esp.get_carriage_weight();
     if !target_force.is_finite() {
         //divide by zero etc.
         //may happen if time delta underflows
         0.0
     } else {
         0.0
     }
   }
}
