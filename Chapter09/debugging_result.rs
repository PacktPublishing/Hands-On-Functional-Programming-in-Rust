fn expect_1or2or_other(n: u64) -> Option<u64> {
   match n {
     1|2 => Some(n),
     _ => None
   }
}

fn expect_1or2or_error(n: u64) -> Result<u64,()> {
   match n {
     1|2 => Ok(n),
     _ => Err(())
   }
}

fn mixed_1or2() -> Result<(),()> {
   expect_1or2or_other(1);
   expect_1or2or_other(2);
   expect_1or2or_other(3);

   expect_1or2or_error(1)?;
   expect_1or2or_error(2)?;
   expect_1or2or_error(3).unwrap_or(222);

   Ok(())
}

use std::fs::File;
use std::io::prelude::*;
use std::io;

fn lots_of_io() -> io::Result<()> {
   {
      let mut file = File::create("data.txt")?;
      file.write_all(b"data\ndata\ndata")?;
   }
   {
      let mut file = File::open("data.txt")?;
      let mut data = String::new();
      file.read_to_string(&mut data)?;
      println!("{}", data);
   }

   Ok(())
}

fn main() {
   mixed_1or2().expect("mixed 1 or 2 is OK.");

   lots_of_io().expect("lots of io is OK.");
}
