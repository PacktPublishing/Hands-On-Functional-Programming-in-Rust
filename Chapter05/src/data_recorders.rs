use buildings::{Building};
use physics::{ElevatorState};

pub trait DataRecorder
{
   fn init(&mut self, esp: Box<Building>, est: ElevatorState);
   fn poll(&mut self, est: ElevatorState, dst: u64);
   fn summary(&mut self);
}

pub struct SimpleDataRecorder;

pub fn newSimpleDataRecorder() -> SimpleDataRecorder
{
   SimpleDataRecorder
}

/*
use physics::{ElevatorSpecification, ElevatorState, MotorInput, SimpleMotorInput, simulate_elevator, DataRecorder, MotorController, MotorVoltage,
              ElevatorStateClone, ElevatorSpecificationClone, Motor, SimpleMotor};
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
      self.log.write_all(serde_json::to_string(&esp.clone().dump()).unwrap().as_bytes()).expect("write spec to log");
      self.log.write_all(b"\r\n").expect("write spec to log");
   }
   fn poll(&mut self, est: ElevatorState, dst: u64)
   {
      let datum = (est.clone().dump(), dst);
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
*/

