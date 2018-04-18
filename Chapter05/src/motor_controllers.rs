use libc::c_int;

#[link(name = "motor1")]
extern {
   pub fn motor1_adjust_motor(target_force: c_int) -> c_int;
}

#[link(name = "motor2")]
extern {
   pub fn motor2_adjust_motor(target_force: c_int) -> c_int;
}

#[link(name = "motor3")]
extern {
   pub fn motor3_adjust_motor(target_force: c_int) -> c_int;
}
