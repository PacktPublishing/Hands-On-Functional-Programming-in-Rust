extern crate elevator;
use elevator::motor_controllers::{motor1_adjust_motor,motor2_adjust_motor,motor3_adjust_motor};

fn main(){
   unsafe {
      println!("motor1 adjust");
      motor1_adjust_motor(0);

      println!("motor2 adjust");
      motor2_adjust_motor(0);

      println!("motor3 adjust");
      motor3_adjust_motor(0);
   }
}
