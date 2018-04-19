
pub struct FloorRequests
{
   pub requests: Vec<u64>
}

pub trait RequestQueue
{
   fn add_request(&mut self, req: u64);
   fn add_requests(&mut self, reqs: Vec<u64>);
   fn pop_request(&mut self) -> Option<u64>;
}

impl RequestQueue for FloorRequests
{
   fn add_request(&mut self, req: u64)
   {
      self.requests.push(req);
   }
   fn add_requests(&mut self, reqs: Vec<u64>)
   {
      for req in reqs
      {
         self.requests.push(req);
      }
   }
   fn pop_request(&mut self) -> Option<u64>
   {
      if self.requests.len()>0 {
         Some(self.requests.remove(0))
      } else {
         None
      }
   }
}
