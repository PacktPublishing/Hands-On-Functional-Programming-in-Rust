use buildings::{Building};
use trip_planning::{RequestQueue};
use motion_controllers::{MotionController};
use data_recorders::{DataRecorder};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ElevatorState {
   pub timestamp: f64,
   pub location: f64,
   pub velocity: f64,
   pub acceleration: f64,
   pub motor_input: f64
}

pub const MAX_JERK: f64 = 0.2;
pub const MAX_ACCELERATION: f64 = 2.0;
pub const MAX_VELOCITY: f64 = 5.0;

pub fn simulate_elevator(esp: Box<Building>, est: ElevatorState, floor_requests: &mut Box<RequestQueue>,
                         mc: &mut Box<MotionController>, dr: &mut Box<DataRecorder>)
{
}
