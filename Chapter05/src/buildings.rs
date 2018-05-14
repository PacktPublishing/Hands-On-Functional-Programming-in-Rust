use elevator_drivers::{ElevatorDriver, ElevatorDriver1, ElevatorDriver2, ElevatorDriver3};
use motor_controllers::{MotorController, newMotorController1, newMotorController2, newMotorController3};

pub trait Building
{
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>;
   fn get_motor_controller(&self) -> Box<MotorController>;
   fn get_floor_heights(&self) -> Vec<f64>;
   fn get_carriage_weight(&self) -> f64;
   fn clone(&self) -> Box<Building>;
   fn serialize(&self) -> u64;
}

pub fn deserialize(n: u64) -> Box<Building>
{
   if n==1 {
      Box::new(Building1)
   } else if n==2 {
      Box::new(Building2)
   } else {
      Box::new(Building3)
   }
}

pub fn getCarriageFloor(floorHeights: Vec<f64>, height: f64) -> u64
{
   let mut c = 0.0;
   for (fi, fht) in floorHeights.iter().enumerate() {
      c += fht;
      if height <= c {
         return (fi as u64)
      }
   }
   (floorHeights.len()-1) as u64
}

pub fn getCumulativeFloorHeight(heights: Vec<f64>, floor: u64) -> f64
{
   heights.iter().take(floor as usize).sum()
}

pub struct Building1;
impl Building for Building1 {
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>
   {
      Box::new(ElevatorDriver1)
   }
   fn get_motor_controller(&self) -> Box<MotorController>
   {
      newMotorController1()
   }
   fn get_floor_heights(&self) -> Vec<f64>
   {
      vec![8.0, 4.0, 4.0, 4.0, 4.0]
   }
   fn get_carriage_weight(&self) -> f64
   {
      1200.0
   }
   fn clone(&self) -> Box<Building> {
      Box::new(Building1)
   }
   fn serialize(&self) -> u64
   {
      2
   }
}

pub struct Building2;
impl Building for Building2 {
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>
   {
      Box::new(ElevatorDriver2)
   }
   fn get_motor_controller(&self) -> Box<MotorController>
   {
      newMotorController2()
   }
   fn get_floor_heights(&self) -> Vec<f64>
   {
      vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0]
   }
   fn get_carriage_weight(&self) -> f64
   {
      1350.0
   }
   fn clone(&self) -> Box<Building> {
      Box::new(Building2)
   }
   fn serialize(&self) -> u64
   {
      2
   }
}

pub struct Building3;
impl Building for Building3 {
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>
   {
      Box::new(ElevatorDriver3)
   }
   fn get_motor_controller(&self) -> Box<MotorController>
   {
      newMotorController3()
   }
   fn get_floor_heights(&self) -> Vec<f64>
   {
      vec![6.0, 4.0, 4.0, 4.0]
   }
   fn get_carriage_weight(&self) -> f64
   {
      1400.0
   }
   fn clone(&self) -> Box<Building> {
      Box::new(Building3)
   }
   fn serialize(&self) -> u64
   {
      3
   }
}
