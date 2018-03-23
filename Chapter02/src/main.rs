fn main()
{

   //1. Store location, velocity, and acceleration state
   let mut location: f64 = 0.0; // meters
   let mut velocity: f64 = 0.0; // meters per second
   let mut acceleration: f64 = 0.0; // meters per second squared

   //2. Store motor input voltage
   let mut up_input_voltage: f64 = 0.0;
   let mut down_input_voltage: f64 = 0.0;

   //3. Store input building description and floor requests
   let mut floor_count: u64 = 0;
   let mut floor_height: f64 = 0.0; // meters
   let mut floor_requests: Vec<u64> = Vec::new();

   //4. Loop while there are remaining floor requests
   while floor_requests.len() > 0
   {
      //4.1. Update location, velocity, and acceleration

      //4.2. If next floor request in queue is satisfied, then remove from queue

      //4.3. Adjust motor control to process next floor request

      //4.4. Print realtime statistics
   }

   //5. Print summary

   println!("main");

}
