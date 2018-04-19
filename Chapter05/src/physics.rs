
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ElevatorState {
   pub timestamp: f64,
   pub location: f64,
   pub velocity: f64,
   pub acceleration: f64,
   pub motor_input: f64
}
