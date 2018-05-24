use std::rc::Rc;
use std::sync::{Arc,Mutex};

#[derive(Clone)]
struct LazyList<A: Clone> {
   buffer: Rc<Vec<A>>,
   index: usize
}
impl<A: Clone> LazyList<A> {
   fn new(buf: Vec<A>) -> LazyList<A> {
      LazyList {
         buffer: Rc::new(buf),
         index: 0
      }
   }
   fn next(&self) -> Option<(LazyList<A>,A)> {
      if self.index < self.buffer.len() {
         let new_item = self.buffer[self.index].clone();
         let new_index = self.index + 1;
         Some((LazyList {
            buffer: Rc::clone(&self.buffer),
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
   fn plus<St2: 'static,C: 'static>(&self, other: ReactiveUnit<St2,B,C>) -> ReactiveUnit<(Arc<Mutex<St>>,Arc<Mutex<St2>>),A,C> {
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

   println!("\nrender 1:");
   let render1 = ReactiveUnit::new((),|(),()| {
      let html = r###"$('body').innerHTML = '
        <header>
          <h3 data-section="1" class="active">Section 1</h3>
          <h3 data-section="2">Section 2</h3>
          <h3 data-section="3">Section 3</h3>
        </header>
        <div>page content</div>
        <footer>Copyright</footer>
      ';"###;
      html.to_string()
   });
   println!("{}", render1.apply(()));

   println!("\nrender 2:");
   let render2 = ReactiveUnit::new((),|(),section: usize| {
      let section_1 = r###"$('body').innerHTML = '
        <header>
          <h3 data-section="1" class="active">Section 1</h3>
          <h3 data-section="2">Section 2</h3>
          <h3 data-section="3">Section 3</h3>
        </header>
        <div>section 1 content</div>
        <footer>Copyright</footer>
      ';"###;

      let section_2 = r###"$('body').innerHTML = '
        <header>
          <h3 data-section="1">Section 1</h3>
          <h3 data-section="2" class="active">Section 2</h3>
          <h3 data-section="3">Section 3</h3>
        </header>
        <div>section 2 content</div>
        <footer>Copyright</footer>
      ';"###;

      let section_3 = r###"$('body').innerHTML = '
        <header>
          <h3 data-section="1">Section 1</h3>
          <h3 data-section="2">Section 2</h3>
          <h3 data-section="3" class="active">Section 3</h3>
        </header>
        <div>section 3 content</div>
        <footer>Copyright</footer>
      ';"###;

      if section==1 {
         section_1.to_string()
      } else if section==2 {
         section_2.to_string()
      } else if section==3 {
         section_3.to_string()
      } else {
         panic!("unknown section")
      }
   });
   println!("{}", render2.apply(1));
   println!("{}", render2.apply(2));
   println!("{}", render2.apply(3));

   let render3header = ReactiveUnit::new(None,|opsec: &mut Option<usize>,section: usize| {
      let section_1 = r###"$('header').innerHTML = '
         <h3 data-section="1" class="active">Section 1</h3>
         <h3 data-section="2">Section 2</h3>
         <h3 data-section="3">Section 3</h3>
      ';"###;
      let section_2 = r###"$('header').innerHTML = '
         <h3 data-section="1">Section 1</h3>
         <h3 data-section="2" class="active">Section 2</h3>
         <h3 data-section="3">Section 3</h3>
      ';"###;
      let section_3 = r###"$('header').innerHTML = '
         <h3 data-section="1">Section 1</h3>
         <h3 data-section="2">Section 2</h3>
         <h3 data-section="3" class="active">Section 3</h3>
      ';"###;

      let changed = if section==1 {
         section_1
      } else if section==2 {
         section_2
      } else if section==3 {
         section_3
      } else {
         panic!("invalid section")
      };

      if let Some(sec) = *opsec {
         if sec==section { "" }
         else {
           *opsec = Some(section);
           changed
         }
      } else {
         *opsec = Some(section);
         changed
      }
   });

   let render3content = ReactiveUnit::new(None,|opsec: &mut Option<usize>,section: usize| {
      let section_1 = r###"$('div#content').innerHTML = '
         section 1 content
      ';"###;
      let section_2 = r###"$('div#content').innerHTML = '
         section 2 content
      ';"###;
      let section_3 = r###"$('div#content').innerHTML = '
         section 3 content
      ';"###;

      let changed = if section==1 {
         section_1
      } else if section==2 {
         section_2
      } else if section==3 {
         section_3
      } else {
         panic!("invalid section")
      };

      if let Some(sec) = *opsec {
         if sec==section { "" }
         else {
           *opsec = Some(section);
           changed
         }
      } else {
         *opsec = Some(section);
         changed
      }
   });

   let render3 = ReactiveUnit::new((render3header,render3content), |(rheader,rcontent),section: usize| {
      let header = rheader.apply(section);
      let content = rcontent.apply(section);
      format!("{}{}", header, content)
   });

   println!("section 1: {}", render3.apply(1));
   println!("section 2: {}", render3.apply(2));
   println!("section 2: {}", render3.apply(2));
   println!("section 3: {}", render3.apply(3));

   let database = ("hello world", 5, 2);
   let react1 = ReactiveUnit::new((database,render3), |(database,render),evt:(&str,&str)| {
      match evt {
         ("header button click",n) => render.apply(n.parse::<usize>().unwrap()),
         ("text submission",s) => { database.0 = s; format!("db.textfield1.set(\"{}\")",s) },
         ("number 1 submission",n) => { database.1 += n.parse::<i32>().unwrap(); format!("db.numfield1.set(\"{}\")",database.1) },
         ("number 2 submission",n) => { database.2 += n.parse::<i32>().unwrap(); format!("db.numfield2.set(\"{}\")",database.2) },
         _ => "".to_string()
      }
   });
   println!("react 1: {}", react1.apply(("header button click","2")));
   println!("react 1: {}", react1.apply(("header button click","2")));
   println!("react 1: {}", react1.apply(("text submission","abc def")));
   println!("react 1: {}", react1.apply(("number 1 submission","123")));
   println!("react 1: {}", react1.apply(("number 1 submission","234")));
   println!("react 1: {}", react1.apply(("number 2 submission","333")));
   println!("react 1: {}", react1.apply(("number 2 submission","222")));
}
