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
   fn adjust_motor(input: MotorInput);
}

struct Motor1;
impl MotorDriver for Motor1
{
   fn adjust_motor(input: MotorInput)
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
   fn adjust_motor(input: MotorInput)
   {
      if let MotorInput::Motor1 { target_force: target_force } = input {
         unsafe {
            motor2_adjust_motor(target_force);
         }
      }
   }
}

struct Motor3;
impl MotorDriver for Motor3
{
   fn adjust_motor(input: MotorInput)
   {
      if let MotorInput::Motor1 { target_force: target_force } = input {
         unsafe {
            motor3_adjust_motor(target_force);
         }
      }
   }
}
