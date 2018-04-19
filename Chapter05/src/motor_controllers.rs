use libc::c_double;

#[link(name = "motor1")]
extern {
   pub fn motor1_adjust_motor(target_force: c_double) -> c_double;
}

#[link(name = "motor2")]
extern {
   pub fn motor2_adjust_motor(target_force: c_double) -> c_double;
}

#[link(name = "motor3")]
extern {
   pub fn motor3_adjust_motor(target_force: c_double) -> c_double;
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub enum MotorInput
{
   Motor1 { target_force: f64 },
   Motor2 { target_force: f64 },
   Motor3 { target_force: f64 },
}

pub trait MotorDriver
{
   fn adjust_motor(&self, input: MotorInput);
}

struct Motor1;
impl MotorDriver for Motor1
{
   fn adjust_motor(&self, input: MotorInput)
   {
      if let MotorInput::Motor1 { target_force: target_force } = input {
         unsafe {
            motor1_adjust_motor(target_force);
         }
      }
   }
}

struct Motor2;
impl MotorDriver for Motor2
{
   fn adjust_motor(&self, input: MotorInput)
   {
      if let MotorInput::Motor2 { target_force: target_force } = input {
         unsafe {
            motor2_adjust_motor(target_force);
         }
      }
   }
}

struct Motor3;
impl MotorDriver for Motor3
{
   fn adjust_motor(&self, input: MotorInput)
   {
      if let MotorInput::Motor3 { target_force: target_force } = input {
         unsafe {
            motor3_adjust_motor(target_force);
         }
      }
   }
}

pub trait MotorController
{
   fn adjust_motor(&self, f: f64);
   fn max_force(&self) -> f64;
}

pub struct MotorController1
{
   motor: Motor1
}
pub fn newMotorController1() -> Box<MotorController>
{
   Box::new(MotorController1 {
      motor: Motor1
   })
}
impl MotorController for MotorController1
{
   fn adjust_motor(&self, f: f64)
   {
      self.motor.adjust_motor(MotorInput::Motor1 {
         target_force: f
      })
   }
   fn max_force(&self) -> f64
   {
      50000.0
   }
}

pub struct MotorController2
{
   motor: Motor2
}
pub fn newMotorController2() -> Box<MotorController>
{
   Box::new(MotorController2 {
      motor: Motor2
   })
}
impl MotorController for MotorController2
{
   fn adjust_motor(&self, f: f64)
   {
      self.motor.adjust_motor(MotorInput::Motor2 {
         target_force: f
      })
   }
   fn max_force(&self) -> f64
   {
      100000.0
   }
}

pub struct MotorController3
{
   motor: Motor3
}
pub fn newMotorController3() -> Box<MotorController>
{
   Box::new(MotorController3 {
      motor: Motor3
   })
}
impl MotorController for MotorController3
{
   fn adjust_motor(&self, f: f64)
   {
      self.motor.adjust_motor(MotorInput::Motor3 {
         target_force: f
      })
   }
   fn max_force(&self) -> f64
   {
      90000.0
   }
}
