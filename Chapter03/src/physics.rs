extern crate floating_duration;
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use std::{thread, time};

#[derive(Clone)]
pub enum MotorInput
{
   Up { voltage: f64 },
   Down { voltage: f64 }
}

#[derive(Clone)]
pub struct ElevatorSpecification
{
   pub floor_count: u64,
   pub floor_height: f64,
   pub carriage_weight: f64
}

#[derive(Clone)]
pub struct ElevatorState
{
   pub timestamp: Instant,
   pub location: f64,
   pub velocity: f64,
   pub acceleration: f64,
   pub motor_input: MotorInput
}

pub type FloorRequests = Vec<u64>;

pub trait MotorController
{
   fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState);
   fn poll(&mut self, est: ElevatorState, dst: u64) -> MotorInput;
}

pub trait DataRecorder
{
   fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState);
   fn poll(&mut self, est: ElevatorState, dst: u64);
}

pub trait MotorForce {
   fn calculate_force(&self) -> f64;
}
impl MotorForce for MotorInput {
   fn calculate_force(&self) -> f64
   {
      match *self {
         MotorInput::Up { voltage: v } => { v * 8.0 }
         MotorInput::Down { voltage: v } => { v * -8.0 }
      }
   }
}

pub fn simulate_elevator<MC: MotorController, DR: DataRecorder>(esp: ElevatorSpecification, est: ElevatorState, req: FloorRequests,
                         mc: &mut MC, dr: &mut DR) {

   //immutable input becomes mutable local state
   let mut esp = esp.clone();
   let mut est = est.clone();
   let mut req = req.clone();

   //initialize MotorController and DataController
   mc.init(esp.clone(), est.clone());
   dr.init(esp.clone(), est.clone());

   //5. Loop while there are remaining floor requests
   while req.len() > 0
   {
      //5.1. Update location, velocity, and acceleration
      let now = Instant::now();
      let dt = now.duration_since(est.timestamp)
                  .as_fractional_secs();
      est.timestamp = now;

      est.location = est.location + est.velocity * dt;
      est.velocity = est.velocity + est.acceleration * dt;
      est.acceleration = {
         let F = est.motor_input.calculate_force();
         let m = esp.carriage_weight;
         -9.8 + F/m
      };

      //5.2. If next floor request in queue is satisfied, then remove from queue
      let next_floor = req[0];
      if (est.location - (next_floor as f64)*esp.floor_height).abs() < 0.01 &&
         est.velocity.abs() < 0.01
      {
         est.velocity = 0.0;
         req.remove(0);
      }

      //5.3. Adjust motor control to process next floor request
      est.motor_input = mc.poll(est.clone(), next_floor);

      //5.4. Print realtime statistics
      dr.poll(est.clone(), next_floor);

      thread::sleep(time::Duration::from_millis(10));
   }
}
