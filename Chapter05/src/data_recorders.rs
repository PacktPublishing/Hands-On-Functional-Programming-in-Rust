use buildings::{Building, getCarriageFloor};
use physics::{ElevatorState};
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::prelude::*;
use termion;
use termion::{clear, cursor, style};
use termion::raw;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use serde_json;

pub trait DataRecorder
{
   fn init(&mut self, esp: Box<Building>, est: ElevatorState);
   fn record(&mut self, est: ElevatorState, dst: u64);
   fn summary(&mut self);
}

struct SimpleDataRecorder<W: Write>
{
   esp: Box<Building>,
   termwidth: u64,
   termheight: u64,
   stdout: raw::RawTerminal<W>,
   log: File,
   record_location: Vec<f64>,
   record_velocity: Vec<f64>,
   record_acceleration: Vec<f64>,
   record_force: Vec<f64>,
}

pub fn newSimpleDataRecorder(esp: Box<Building>) -> Box<DataRecorder>
{
   let termsize = termion::terminal_size().ok();
   Box::new(SimpleDataRecorder {
      esp: esp.clone(),
      termwidth: termsize.map(|(w,_)| w-2).expect("termwidth") as u64,
      termheight: termsize.map(|(_,h)| h-2).expect("termheight") as u64,
      stdout: io::stdout().into_raw_mode().unwrap(),
      log: File::create("simulation.log").expect("log file"),
      record_location: Vec::new(),
      record_velocity: Vec::new(),
      record_acceleration: Vec::new(),
      record_force: Vec::new()
   })
}

impl<W: Write> DataRecorder for SimpleDataRecorder<W>
{
   fn init(&mut self, esp: Box<Building>, est: ElevatorState)
   {
      self.esp = esp.clone();
      self.log.write_all(serde_json::to_string(&esp.serialize()).unwrap().as_bytes()).expect("write spec to log");
      self.log.write_all(b"\r\n").expect("write spec to log");
   }
   fn record(&mut self, est: ElevatorState, dst: u64)
   {
      let datum = serde_json::to_string(&(est.clone(), dst)).unwrap();
      self.log.write_all(datum.as_bytes()).expect("write state to log");
      self.log.write_all(b"\r\n").expect("write state to log");

      self.record_location.push(est.location);
      self.record_velocity.push(est.velocity);
      self.record_acceleration.push(est.acceleration);
      self.record_force.push(est.motor_input);

      //5.4. Print realtime statistics
      print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
      let carriage_floor = getCarriageFloor(self.esp.get_floor_heights(), est.location);
      let floor_count = self.esp.get_floor_heights().len() as u64;
      let mut terminal_buffer = vec![' ' as u8; (self.termwidth*self.termheight) as usize];
      for ty in 0..floor_count
      {
         terminal_buffer[ (ty*self.termwidth + 0) as usize ] = '[' as u8;
         terminal_buffer[ (ty*self.termwidth + 1) as usize ] =
            if   (ty as u64)==((floor_count-1)-carriage_floor) { 'X' as u8 }
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
         format!("Force [up-down]   {:.06}", est.motor_input),
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
   fn summary(&mut self)
   {
      //6 Calculate and print summary statistics
      write!(self.stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();
      variable_summary(&mut self.stdout, "location".to_string(), &self.record_location);
      variable_summary(&mut self.stdout, "velocity".to_string(), &self.record_velocity);
      variable_summary(&mut self.stdout, "acceleration".to_string(), &self.record_acceleration);
      variable_summary(&mut self.stdout, "force".to_string(), &self.record_force);
      self.stdout.flush().unwrap();
   }
}

fn variable_summary<W: Write>(stdout: &mut raw::RawTerminal<W>, vname: String, data: &Vec<f64>) {
   let (avg, dev) = variable_summary_stats(data);
   variable_summary_print(stdout, vname, avg, dev);
}

fn variable_summary_stats(data: &Vec<f64>) -> (f64, f64)
{
   //calculate statistics
   let N = data.len();
   let sum = data.iter().sum::<f64>();
   let avg = sum / (N as f64);
   let dev = (
       data.clone().into_iter()
       .map(|v| (v - avg).powi(2))
       .sum::<f64>()
       / (N as f64)
   ).sqrt();
   (avg, dev)
}

fn variable_summary_print<W: Write>(stdout: &mut raw::RawTerminal<W>, vname: String, avg: f64, dev: f64)
{
   //print formatted output
   writeln!(stdout, "Average of {:25}{:.6}", vname, avg);
   writeln!(stdout, "Standard deviation of {:14}{:.6}", vname, dev);
   writeln!(stdout, "");
}
