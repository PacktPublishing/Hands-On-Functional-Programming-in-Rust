use libc::c_int;

#[link(name = "elevator1")]
extern {
   pub fn elevator1_poll_floor_request() -> c_int;
}

#[link(name = "elevator2")]
extern {
   pub fn elevator2_poll_floor_request() -> c_int;
}

#[link(name = "elevator3")]
extern {
   pub fn elevator3_poll_floor_request() -> c_int;
}

pub trait ElevatorDriver
{
   fn poll_floor_request(&self) -> Option<u64>;
}

pub struct ElevatorDriver1;
impl ElevatorDriver for ElevatorDriver1
{
   fn poll_floor_request(&self) -> Option<u64>
   {
      unsafe {
         let req = elevator1_poll_floor_request();
         if req > 0 {
            Some(req as u64)
         } else {
            None
         }
      }
   }
}

pub struct ElevatorDriver2;
impl ElevatorDriver for ElevatorDriver2
{
   fn poll_floor_request(&self) -> Option<u64>
   {
      unsafe {
         let req = elevator2_poll_floor_request();
         if req > 0 {
            Some(req as u64)
         } else {
            None
         }
      }
   }
}

pub struct ElevatorDriver3;
impl ElevatorDriver for ElevatorDriver3
{
   fn poll_floor_request(&self) -> Option<u64>
   {
      unsafe {
         let req = elevator3_poll_floor_request();
         if req > 0 {
            Some(req as u64)
         } else {
            None
         }
      }
   }
}
