mod physics;
mod motor;

use physics::{ElevatorSpecification, ElevatorState, MotorInput, simulate_elevator, DataRecorder, MotorController, MotorVoltage};
use motor::{SmoothMotorController, SimpleMotorController};

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate floating_duration;
use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::prelude::*;
extern crate termion;
use termion::{clear, cursor, style};
use termion::raw;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use std::cmp;

fn variable_summary<W: Write>(stdout: &mut raw::RawTerminal<W>, vname: String, data: &Vec<f64>) {
   let (avg, dev) = variable_summary_stats(data);
   variable_summary_print(stdout, vname, avg, dev);
}

fn variable_summary_stats(data: &Vec<f64>) -> (f64, f64)
{
   //calculate statistics
   let N = data.len();
   let sum = data.clone().into_iter()
            .fold(0.0, |a, b| a+b);
   let avg = sum / (N as f64);
   let dev = (
       data.clone().into_iter()
       .map(|v| (v - avg).powi(2))
       .fold(0.0, |a, b| a+b)
       / (N as f64)
   ).sqrt();
   (avg, dev)
}

fn variable_summary_print<W: Write>(stdout: &mut raw::RawTerminal<W>, vname: String, avg: f64, dev: f64)
{
   //print formatted output
   write!(stdout, "Average of {:25}{:.6}\r\n", vname, avg);
   write!(stdout, "Standard deviation of {:14}{:.6}\r\n", vname, dev);
   write!(stdout, "\r\n");
}

struct SimpleDataRecorder<'a, W: 'a + Write>
{
   esp: ElevatorSpecification,
   termwidth: u64,
   termheight: u64,
   stdout: &'a mut raw::RawTerminal<W>,
   log: File,
   record_location: Vec<f64>,
   record_velocity: Vec<f64>,
   record_acceleration: Vec<f64>,
   record_voltage: Vec<f64>,
}
impl<'a, W: Write> DataRecorder for SimpleDataRecorder<'a, W>
{
   fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState)
   {
      self.esp = esp.clone();
      self.log.write_all(serde_json::to_string(&esp.clone()).unwrap().as_bytes()).expect("write spec to log");
      self.log.write_all(b"\r\n").expect("write spec to log");
   }
   fn poll(&mut self, est: ElevatorState, dst: u64)
   {
      let datum = (est.clone(), dst);
      self.log.write_all(serde_json::to_string(&datum).unwrap().as_bytes()).expect("write state to log");
      self.log.write_all(b"\r\n").expect("write state to log");

      self.record_location.push(est.location);
      self.record_velocity.push(est.velocity);
      self.record_acceleration.push(est.acceleration);
      self.record_voltage.push(est.motor_input.voltage());

      //5.4. Print realtime statistics
      print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
      let carriage_floor = (est.location / self.esp.floor_height).floor();
      let carriage_floor = if carriage_floor < 1.0 { 0 } else { carriage_floor as u64 };
      let carriage_floor = cmp::min(carriage_floor, self.esp.floor_count-1);
      let mut terminal_buffer = vec![' ' as u8; (self.termwidth*self.termheight) as usize];
      for ty in 0..self.esp.floor_count
      {
         terminal_buffer[ (ty*self.termwidth + 0) as usize ] = '[' as u8;
         terminal_buffer[ (ty*self.termwidth + 1) as usize ] =
            if   (ty as u64)==((self.esp.floor_count-1)-carriage_floor) { 'X' as u8 }
            else { ' ' as u8 };
         terminal_buffer[ (ty*self.termwidth + 2) as usize ] = ']' as u8;
         terminal_buffer[ (ty*self.termwidth + self.termwidth-2) as usize ] = '\r' as u8;
         terminal_buffer[ (ty*self.termwidth + self.termwidth-1) as usize ] = '\n' as u8;
      }
      let stats = vec![
         format!("Carriage at floor {}", carriage_floor+1),
         format!("Location          {:.06}", est.location),
         format!("Velocity          {:.06}", est.velocity),
         format!("Acceleration      {:.06}", est.acceleration),
         format!("Voltage [up-down] {:.06}", est.motor_input.voltage()),
      ];
      for sy in 0..stats.len()
      {
         for (sx,sc) in stats[sy].chars().enumerate()
         {
            terminal_buffer[ sy*(self.termwidth as usize) + 6 + sx ] = sc as u8;
         }
      }
      write!(self.stdout, "{}", String::from_utf8(terminal_buffer).ok().unwrap());
      self.stdout.flush().unwrap();
   }
}

trait DataRecorderSummary {
   fn summary(&mut self);
}
impl<'a, W: Write> DataRecorderSummary for SimpleDataRecorder<'a, W> {
   fn summary(&mut self)
   {
      //6 Calculate and print summary statistics
      write!(self.stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();
      variable_summary(&mut self.stdout, "location".to_string(), &self.record_location);
      variable_summary(&mut self.stdout, "velocity".to_string(), &self.record_velocity);
      variable_summary(&mut self.stdout, "acceleration".to_string(), &self.record_acceleration);
      variable_summary(&mut self.stdout, "voltage".to_string(), &self.record_voltage);
      self.stdout.flush().unwrap();
   }
}

pub fn run_simulation()
{

   //1. Store location, velocity, and acceleration state
   //2. Store motor input voltage
   let mut est = ElevatorState {
      timestamp: 0.0,
      location: 0.0,
      velocity: 0.0,
      acceleration: 0.0,
      motor_input: MotorInput::Up {
         //zero is positive force to counter gravity
         voltage: 9.8 * (120000.0 / 8.0)
      }
   };

   //3. Store input building description and floor requests
   let mut esp = ElevatorSpecification {
      floor_count: 0,
      floor_height: 0.0,
      carriage_weight: 120000.0
   };
   let mut floor_requests = Vec::new();

   //4. Parse input and store as building description and floor requests
   let buffer = match env::args().nth(1) {
      Some(ref fp) if *fp == "-".to_string()  => {
         let mut buffer = String::new();
         io::stdin().read_to_string(&mut buffer)
                    .expect("read_to_string failed");
         buffer
      },
      None => {
         let fp = "test1.txt";
         let mut buffer = String::new();
         File::open(fp)
              .expect("File::open failed")
              .read_to_string(&mut buffer)
              .expect("read_to_string failed");
         buffer
      },
      Some(fp) => {
         let mut buffer = String::new();
         File::open(fp)
              .expect("File::open failed")
              .read_to_string(&mut buffer)
              .expect("read_to_string failed");
         buffer
      }
   };

   for (li,l) in buffer.lines().enumerate() {
      if li==0 {
         esp.floor_count = l.parse::<u64>().unwrap();
      } else if li==1 {
         esp.floor_height = l.parse::<f64>().unwrap();
      } else {
         floor_requests.push(l.parse::<u64>().unwrap());
      }
   }

   let termsize = termion::terminal_size().ok();
   let mut dr = SimpleDataRecorder {
      esp: esp.clone(),
      termwidth: termsize.map(|(w,_)| w-2).expect("termwidth") as u64,
      termheight: termsize.map(|(_,h)| h-2).expect("termheight") as u64,
      stdout: &mut io::stdout().into_raw_mode().unwrap(),
      log: File::create("simulation.log").expect("log file"),
      record_location: Vec::new(),
      record_velocity: Vec::new(),
      record_acceleration: Vec::new(),
      record_voltage: Vec::new()
   };
   /*
   let mut mc = SimpleMotorController {
      esp: esp.clone()
   };
   */
   let mut mc = SmoothMotorController {
      timestamp: 0.0,
      esp: esp.clone()
   };

   simulate_elevator(esp, est, floor_requests, &mut mc, &mut dr);
   dr.summary();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_stats() {
        let test_data = vec![
           (vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0, 1.41),
           (vec![1.0, 3.0, 5.0, 7.0, 9.0], 5.0, 2.83),
           (vec![1.0, 9.0, 1.0, 9.0, 1.0], 4.2, 3.92),
           (vec![1.0, 0.5, 0.7, 0.9, 0.6], 0.74, 0.19),
           (vec![200.0, 3.0, 24.0, 92.0, 111.0], 86.0, 69.84),
        ];
        for (data, avg, dev) in test_data
        {
           let (ravg, rdev) = variable_summary_stats(data);
           assert!( (avg-ravg).abs() < 0.1 );
           assert!( (dev-rdev).abs() < 0.1 );
        }
    }
}
