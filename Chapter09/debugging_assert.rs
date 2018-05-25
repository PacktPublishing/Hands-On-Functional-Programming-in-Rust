use std::io;

fn debug_precondition(n: u64) -> u64 {
   debug_assert!(n < 100);
   n * n
}

fn debug_postcondition(n: u64) -> u64 {
   let r = n * n;
   debug_assert!(r > 10);
   r
}

fn runtime_precondition(n: u64) -> Result<u64,()> {
   if !(n<100) { return Err(()) };
   Ok(n * n)
}

fn runtime_postcondition(n: u64) -> Result<u64,()> {
   let r = n * n;
   if !(r>10) { return Err(()) };
   Ok(r)
}

fn main() {
   //inward facing code should assert expectations
   debug_precondition(5);
   debug_postcondition(5);

   //outward facing code should handle errors
   let mut s = String::new();
   println!("Please input a positive integer greater or equal to 4:");
   io::stdin().read_line(&mut s).expect("error reading input");
   let i = s.trim().parse::<u64>().expect("error parsing input as integer");
   runtime_precondition(i).expect("runtime precondition violated");
   runtime_postcondition(i).expect("runtime postcondition violated");
}
