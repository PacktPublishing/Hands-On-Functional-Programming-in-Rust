extern crate floating_duration;
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use std::{thread, time};
use serde;

pub trait Motor
{
   fn force_of_voltage(&self, v: f64) -> f64;
   fn voltage_of_force(&self, v: f64) -> f64;
}

pub struct SimpleMotor;
impl Motor for SimpleMotor
{
   fn force_of_voltage(&self, v: f64) -> f64
   {
      8.0 * v
   }
   fn voltage_of_force(&self, v: f64) -> f64
   {
      v / 8.0
   }
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub enum SimpleMotorInput
{
   Up { voltage: f64 },
   Down { voltage: f64 }
}

pub trait MotorInput: MotorForce + MotorVoltage
{
}
impl MotorInput for SimpleMotorInput {}

pub struct ElevatorSpecification
{
   pub floor_count: u64,
   pub floor_height: f64,
   pub carriage_weight: f64,
   pub motor: Box<Motor>
}
pub trait ElevatorSpecificationClone
{
   fn clone(&self) -> ElevatorSpecification;
   fn dump(&self) -> (u64,f64,f64,u64);
   fn load((u64,f64,f64,u64)) -> ElevatorSpecification;
}
impl ElevatorSpecificationClone for ElevatorSpecification
{
   fn clone(&self) -> ElevatorSpecification
   {
      ElevatorSpecification
      {
         floor_count: self.floor_count,
         floor_height: self.floor_height,
         carriage_weight: self.carriage_weight,
         motor: Box::new(SimpleMotor)
      }
   }
   fn dump(&self) -> (u64,f64,f64,u64)
   {
      (self.floor_count,
       self.floor_height,
       self.carriage_weight,
       0)
   }
   fn load(esp: (u64,f64,f64,u64)) -> ElevatorSpecification
   {
      ElevatorSpecification
      {
         floor_count: esp.0,
         floor_height: esp.1,
         carriage_weight: esp.2,
         motor: Box::new(SimpleMotor)
      }
   }
}

pub struct ElevatorState
{
   pub timestamp: f64,
   pub location: f64,
   pub velocity: f64,
   pub acceleration: f64,
   pub motor_input: Box<MotorInput>
}
pub trait ElevatorStateClone
{
   fn clone(&self) -> ElevatorState;
   fn dump(&self) -> (f64,f64,f64,f64,f64);
   fn load((f64,f64,f64,f64,f64)) -> ElevatorState;
}
impl ElevatorStateClone for ElevatorState
{
   fn clone(&self) -> ElevatorState
   {
      ElevatorState
      {
         timestamp: self.timestamp,
         location: self.location,
         velocity: self.velocity,
         acceleration: self.acceleration,
         motor_input: simple_from_voltage(self.motor_input.voltage())
      }
   }
   fn dump(&self) -> (f64,f64,f64,f64,f64)
   {
      (self.timestamp,
       self.location,
       self.velocity,
       self.acceleration,
       self.motor_input.voltage())
   }
   fn load(est: (f64,f64,f64,f64,f64)) -> ElevatorState
   {
      ElevatorState
      {
         timestamp: est.0,
         location: est.1,
         velocity: est.2,
         acceleration: est.3,
         motor_input: simple_from_voltage(est.4)
      }
   }
}

pub type FloorRequests = Vec<u64>;

pub trait MotorController
{
   fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState);
   fn poll(&mut self, est: ElevatorState, dst: u64) -> Box<MotorInput>;
}

pub trait DataRecorder
{
   fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState);
   fn poll(&mut self, est: ElevatorState, dst: u64);
}

pub trait MotorForce {
   fn calculate_force(&self) -> f64;
}
impl MotorForce for SimpleMotorInput {
   fn calculate_force(&self) -> f64
   {
      match *self {
         SimpleMotorInput::Up { voltage: v } => { v * 8.0 }
         SimpleMotorInput::Down { voltage: v } => { v * -8.0 }
      }
   }
}

fn simple_from_voltage(v: f64) -> Box<MotorInput>
{
   if v>0.0 {
      Box::new(SimpleMotorInput::Up { voltage: v })
   } else {
      Box::new(SimpleMotorInput::Down { voltage: v.abs() })
   }
}

pub trait MotorVoltage {
   fn voltage(&self) -> f64;
}
impl MotorVoltage for SimpleMotorInput {
   fn voltage(&self) -> f64
   {
      match *self {
         SimpleMotorInput::Up { voltage: v } => { v }
         SimpleMotorInput::Down { voltage: v } => { -v }
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
   let original_ts = Instant::now();
   thread::sleep(time::Duration::from_millis(1));
   while req.len() > 0
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

      //5.4. Print realtime statistics
      dr.poll(est.clone(), next_floor);

      //5.3. Adjust motor control to process next floor request
      est.motor_input = mc.poll(est.clone(), next_floor);

      thread::sleep(time::Duration::from_millis(1));
   }
}
