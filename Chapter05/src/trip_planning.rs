use std::collections::VecDeque;

pub struct FloorRequests
{
   pub requests: VecDeque<u64>
}

pub trait RequestQueue
{
   fn add_request(&mut self, req: u64);
   fn add_requests(&mut self, reqs: &Vec<u64>);
   fn pop_request(&mut self) -> Option<u64>;
}

impl RequestQueue for FloorRequests
{
   fn add_request(&mut self, req: u64)
   {
      self.requests.push_back(req);
   }
   fn add_requests(&mut self, reqs: &Vec<u64>)
   {
      for req in reqs
      {
         self.requests.push_back(*req);
      }
   }
   fn pop_request(&mut self) -> Option<u64>
   {
      self.requests.pop_front()
   }
}
