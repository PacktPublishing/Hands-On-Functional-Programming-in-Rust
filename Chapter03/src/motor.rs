extern crate floating_duration;
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use physics::{ElevatorSpecification, ElevatorState, MotorInput, MotorController};

pub struct SimpleMotorController
{
   esp: ElevatorSpecification
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

      //l = distance to next floor
      let l = (est.location - (dst as f64)*self.esp.floor_height).abs();

      let target_acceleration = {
         //are we going up?
         let going_up = est.location < (dst as f64)*self.esp.floor_height;

         //Do not exceed maximum velocity
         if est.velocity.abs() >= 5.0 {
            if going_up==(est.velocity>0.0) {
               0.0
            //decelerate if going in wrong direction
            } else if going_up {
               1.0
            } else {
               -1.0
            }

         //if within comfortable deceleration range and moving in right direction, decelerate
         } else if l < d && going_up==(est.velocity>0.0) {
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

q
q
