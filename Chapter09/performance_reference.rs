extern crate flame;
use std::fs::File;

fn byref(n: u64, data: &[u64; 1024]) {
   if n>0 {
      byref(n-1, data);
      byref(n-1, data);
   }
}

fn bycopy(n: u64, data: [u64; 1024]) {
   if n>0 {
      bycopy(n-1, data);
      bycopy(n-1, data);
   }
}

struct DataClonable([u64; 1024]);
impl Clone for DataClonable {
   fn clone(&self) -> Self {
      let mut newdata = [0; 1024];
      for i in 0..1024 {
         newdata[i] = self.0[i];
      }
      DataClonable(newdata)
   }
}
fn byclone<T: Clone>(n: u64, data: T) {
   if n>0 {
      byclone(n-1, data.clone());
      byclone(n-1, data.clone());
   }
}

fn main() {
   let data = [0; 1024];
   flame::start("by reference");
   byref(15, &data);
   flame::end("by reference");

   let data = [0; 1024];
   flame::start("by copy");
   bycopy(15, data);
   flame::end("by copy");

   let data = [0; 1024];
   flame::start("by clone");
   byclone(15, data);
   flame::end("by clone");

   let data = DataClonable([0; 1024]);
   flame::start("by clone (with extras)");
   //2^4 instead of 2^15!!!!
   byclone(4, data);
   flame::end("by clone (with extras)");

   flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
