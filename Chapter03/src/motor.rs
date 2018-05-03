extern crate floating_duration;
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use physics::{ElevatorSpecification, ElevatorState, MotorInput, MotorController};

pub struct SimpleMotorController
{
   pub esp: ElevatorSpecification
}

impl MotorController for SimpleMotorController
{
   fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState)
   {
      self.esp = esp;
   }

   fn poll(&mut self, est: ElevatorState, dst: u64) -> MotorInput
   {
      //5.3. Adjust motor control to process next floor request

      //it will take t seconds to decelerate from velocity v at -1 m/s^2
      let t = est.velocity.abs() / 1.0;

      //during which time, the carriage will travel d=t * v/2 meters
      //at an average velocity of v/2 before stopping
      let d = t * (est.velocity/2.0);

      let dst_height = (dst as f64) * self.esp.floor_height;

      //l = distance to next floor
      let l = (est.location - dst_height).abs();

      let target_acceleration = {
         //are we going up?
         let going_up = est.location < dst_height;

         //Do not exceed maximum velocity
         if est.velocity.abs() >= 5.0 {
            if (going_up && est.velocity>0.0)
            || (!going_up && est.velocity<0.0) {
               0.0
            //decelerate if going in wrong direction
            } else if going_up {
               1.0
            } else {
               -1.0
            }

         //if within comfortable deceleration range and moving in right direction, decelerate
         } else if l < d && ((going_up && est.velocity>0.0)
                             || (!going_up && est.velocity<0.0)) {
            if going_up {
               -1.0
            } else {
               1.0
            }

         //else if not at peak velocity, accelerate
         } else {
            if going_up {
               1.0
            } else {
               -1.0
            }
         }
      };

      let gravity_adjusted_acceleration = target_acceleration + 9.8;
      let target_force = gravity_adjusted_acceleration * self.esp.carriage_weight;
      let target_voltage = target_force / 8.0;
      if target_voltage > 0.0 {
         MotorInput::Up { voltage: target_voltage }
      } else {
         MotorInput::Down { voltage: target_voltage.abs() }
      }
   }
}

const MAX_JERK: f64 = 0.2;
const MAX_ACCELERATION: f64 = 2.0;
const MAX_VELOCITY: f64 = 5.0;

pub struct SmoothMotorController
{
   pub esp: ElevatorSpecification,
   pub timestamp: f64
}

impl MotorController for SmoothMotorController
{
   fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState)
   {
      self.esp = esp;
      self.timestamp = est.timestamp;
   }

   fn poll(&mut self, est: ElevatorState, dst: u64) -> MotorInput
   {
      //5.3. Adjust motor control to process next floor request

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

      //l = distance to next floor
      let l = (est.location - (dst as f64)*self.esp.floor_height).abs();

      let target_acceleration = {
         //are we going up?
         let going_up = est.location < (dst as f64)*self.esp.floor_height;

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
      let target_force = gravity_adjusted_acceleration * self.esp.carriage_weight;
      let target_voltage = target_force / 8.0;
      if !target_voltage.is_finite() {
         //divide by zero etc.
         //may happen if time delta underflows
         MotorInput::Up { voltage: 0.0 }
      } else if target_voltage > 0.0 {
         MotorInput::Up { voltage: target_voltage }
      } else {
         MotorInput::Down { voltage: target_voltage.abs() }
      }
   }
}
