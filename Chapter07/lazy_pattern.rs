use std::sync::{Arc,Mutex};

#[derive(Clone)]
struct LazyList<A: Clone> {
   buffer: Arc<Mutex<Vec<A>>>,
   index: usize
}
impl<A: Clone> LazyList<A> {
   fn new(buf: Vec<A>) -> LazyList<A> {
      LazyList {
         buffer: Arc::new(Mutex::new(buf)),
         index: 0
      }
   }
   fn next(&self) -> Option<(LazyList<A>,A)> {
      let buf = self.buffer.lock().unwrap();
      if self.index < buf.len() {
         let new_item = buf[self.index].clone();
         let new_index = self.index + 1;
         Some((LazyList {
            buffer: Arc::clone(&self.buffer),
            index: new_index
         },new_item))
      } else {
         None
      }
   }
}

fn effects_bind<A,B,C,F,G>(f: F, g: G) -> impl Fn(A) -> C
   where F: Fn(A) -> B,
         G: Fn(B) -> C {
   move |a| g(f(a))
}     

fn launch_missiles(i: i32) -> i32 {
   println!("launching {} missiles", i);
   i
}

struct ReactiveUnit<St,A,B> {
   state: Arc<Mutex<St>>,
   event_handler: Arc<Fn(&mut St,A) -> B>
}
impl<St: 'static,A: 'static,B: 'static> ReactiveUnit<St,A,B> {
   fn new<F>(st: St, f: F) -> ReactiveUnit<St,A,B>
      where F: 'static + Fn(&mut St,A) -> B
   {
      ReactiveUnit {
         state: Arc::new(Mutex::new(st)),
         event_handler: Arc::new(f)
      }
   }
   fn bind<G,C>(&self, g: G) -> ReactiveUnit<St,A,C>
      where G: 'static + Fn(&mut St,B) -> C {
      let ev = Arc::clone(&self.event_handler);
      ReactiveUnit {
         state: Arc::clone(&self.state),
         event_handler: Arc::new(move |st: &mut St,a| {
            let r = ev(st,a);
            let r = g(st,r);
            r
         })
      }
   }
   fn rbind<St2: 'static,C: 'static>(&self, other: ReactiveUnit<St2,B,C>) -> ReactiveUnit<(Arc<Mutex<St>>,Arc<Mutex<St2>>),A,C> {
      let ev1 = Arc::clone(&self.event_handler);
      let st1 = Arc::clone(&self.state);
      let ev2 = Arc::clone(&other.event_handler);
      let st2 = Arc::clone(&other.state);
      ReactiveUnit {
         state: Arc::new(Mutex::new((st1,st2))),
         event_handler: Arc::new(move |stst: &mut (Arc<Mutex<St>>,Arc<Mutex<St2>>),a| {
            let mut st1 = stst.0.lock().unwrap();
            let r = ev1(&mut st1, a);
            let mut st2 = stst.1.lock().unwrap();
            let r = ev2(&mut st2, r);
            r
         })
      }
   }
   fn apply(&self, a: A) -> B {
      let mut st = self.state.lock().unwrap();
      (self.event_handler)(&mut st, a)
   }
}

fn main() 
{

   2 + 3;

   || 2 + 3;

   let ll = LazyList::new(vec![1,2,3]);
   let (ll1,a1) = ll.next().expect("expect 1 item");
   println!("lazy item 1: {}", a1);
   let (ll2,a2) = ll1.next().expect("expect 2 item");
   println!("lazy item 2: {}", a2);
   let (ll3,a3) = ll2.next().expect("expect 3 item");
   println!("lazy item 3: {}", a3);
   let (ll2,a2) = ll1.next().expect("expect 2 item");
   println!("lazy item 2: {}", a2);

   let e1 = |i| { println!("before {}", i); i };
   let e2 = |i| { launch_missiles(i) };
   let e3 = |i| { println!("after {}", i); i };
   let s = effects_bind(effects_bind(e1,e2),e3);
   s(22);
}
