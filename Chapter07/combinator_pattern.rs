#[macro_use]
extern crate chomp;
use chomp::prelude::*;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
struct Name<B: Buffer> {
    first: B,
    last:  B,
}

fn name<I: U8Input>(i: I) -> SimpleResult<I, Name<I::Buffer>> {
    parse!{i;
        let first = take_while1(|c| c != b' ');
                    token(b' ');  // skipping this char
        let last  = take_while1(|c| c != b'\n');

        ret Name{
            first: first,
            last:  last,
        }
    }
}


#[derive(Clone)]
struct ParseState<A: Clone> {
   buffer: Rc<Vec<char>>,
   index: usize,
   a: A
}
impl<A: Clone> ParseState<A> {
   fn new(a: A, buffer: String) -> ParseState<A> {
      let buffer: Vec<char> = buffer.chars().collect();
      ParseState {
         buffer: Rc::new(buffer),
         index: 0,
         a: a
      }
   }
   fn next(&self) -> (ParseState<A>,Option<char>) {
      if self.index < self.buffer.len() {
         let new_char = self.buffer[self.index];
         let new_index = self.index + 1;
         (ParseState {
            buffer: Rc::clone(&self.buffer),
            index: new_index,
            a: self.a.clone()
         }, Some(new_char))
      } else {
         (ParseState {
            buffer: Rc::clone(&self.buffer),
            index: self.index,
            a: self.a.clone()
         },None)
      }
   }
}

#[derive(Debug)]
struct ParseRCon<A,B>(A,Result<Option<B>,String>);

#[derive(Debug)]
enum ParseOutput<A> {
   Success(A),
   Failure(String)
}

fn parse<St: Clone,A,P>(p: &P, st: &ParseState<St>) -> ParseOutput<A>
   where P: Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A> {
   match p(st.clone()) {
      ParseRCon(_,Ok(Some(a))) => ParseOutput::Success(a),
      ParseRCon(_,Ok(None)) => ParseOutput::Failure("expected input".to_string()),
      ParseRCon(_,Err(err)) => ParseOutput::Failure(err)
   }
}

fn parse_mzero<St: Clone,A>(st: ParseState<St>) -> ParseRCon<ParseState<St>,A> {
   ParseRCon(st,Err("mzero failed".to_string()))
}

fn parse_return<St: Clone,A: Clone>(a: A) -> impl (Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A>) {
   move |st| { ParseRCon(st,Ok(Some(a.clone()))) }
}

fn parse_token<St: Clone,A,T>(t: T) -> impl (Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A>)
   where T: 'static + Fn(char) -> Option<A> {
   move |st: ParseState<St>| {
      let (next_state,next_char) = st.clone().next(); 
      match next_char {
         Some(c) => ParseRCon(next_state,Ok(t(c))),
         None => ParseRCon(st,Err("end of input".to_string()))
      }
   }
}

fn parse_satisfy<St: Clone,T>(t: T) -> impl (Fn(ParseState<St>) -> ParseRCon<ParseState<St>,char>)
   where T: 'static + Fn(char) -> bool {
   parse_token(move |c| if t(c) {Some(c)} else {None})
}

fn parse_bind<St: Clone,A,B,P1,P2,B1>(p1: P1, b1: B1)
   -> impl Fn(ParseState<St>) -> ParseRCon<ParseState<St>,B>
   where P1: Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A>,
         P2: Fn(ParseState<St>) -> ParseRCon<ParseState<St>,B>,
         B1: Fn(A) -> P2 {
   move |st| {
      match p1(st) {
         ParseRCon(nst,Ok(Some(a))) => b1(a)(nst),
         ParseRCon(nst,Ok(None)) => ParseRCon(nst,Err("bind failed".to_string())),
         ParseRCon(nst,Err(err)) => ParseRCon(nst,Err(err))
      }
   }
}

fn parse_sequence<St: Clone,A,B,P1,P2>(p1: P1, p2: P2)
   -> impl Fn(ParseState<St>) -> ParseRCon<ParseState<St>,B>
   where P1: Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A>,
         P2: Fn(ParseState<St>) -> ParseRCon<ParseState<St>,B> {
   move |st| {
      match p1(st) {
         ParseRCon(nst,Ok(_)) => p2(nst),
         ParseRCon(nst,Err(err)) => ParseRCon(nst,Err(err))
      }
   }
}

fn parse_or<St: Clone,A,P1,P2>(p1: P1, p2: P2)
   -> impl Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A>
   where P1: Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A>,
         P2: Fn(ParseState<St>) -> ParseRCon<ParseState<St>,A> {
   move |st| {
      match p1(st.clone()) {
         ParseRCon(nst,Ok(Some(a))) => ParseRCon(nst,Ok(Some(a))),
         ParseRCon(_,Ok(None)) => p2(st),
         ParseRCon(nst,Err(err)) => ParseRCon(nst,Err(err))
      }
   }
}

fn compose<A,B,C,F,G>(f: F, g: G) -> impl Fn(A) -> C
   where F: 'static + Fn(A) -> B,
         G: 'static + Fn(B) -> C {
   move |x| g(f(x))
}

fn main() 
{
   let fa = |x| x+1;
   let fb = |y| y*2;
   let fc = |z| z/3;

   let g = compose(compose(fa,fb),fc);
   println!("g(1) = {}", g(1));
   println!("g(12) = {}", g(12));
   println!("g(123) = {}", g(123));

   let parse_result = parse_only(name, "Martin Wernstål\n".as_bytes()).unwrap();
   println!("first:{} last:{}",
            String::from_utf8_lossy(parse_result.first),
            String::from_utf8_lossy(parse_result.last));

   let input1 = ParseState::new((), "1 + 2 * 3".to_string());
   let input2 = ParseState::new((), "3 / 2 - 1".to_string());

   let p1 = parse_mzero::<(),()>;
   println!("p1 input1: {:?}", parse(&p1,&input1));
   println!("p1 input2: {:?}", parse(&p1,&input2));

   let p2 = parse_return(123);
   println!("p2 input1: {:?}", parse(&p2,&input1));
   println!("p2 input2: {:?}", parse(&p2,&input2));

   let p3 = parse_satisfy(|c| c=='1');
   println!("p3 input1: {:?}", parse(&p3,&input1));
   println!("p3 input2: {:?}", parse(&p3,&input2));

   let digit = parse_satisfy(|c| c.is_digit(10));
   println!("digit input1: {:?}", parse(&digit,&input1));
   println!("digit input2: {:?}", parse(&digit,&input2));

   let space = parse_satisfy(|c| c==' ');
   println!("space input1: {:?}", parse(&space,&input1));
   println!("space input2: {:?}", parse(&space,&input2));

   let operator = parse_satisfy(|c| c=='+' || c=='-' || c=='*' || c=='/');
   println!("operator input1: {:?}", parse(&operator,&input1));
   println!("operator input2: {:?}", parse(&operator,&input2));

   let ps1 = parse_sequence(digit,space);
   let ps2 = parse_sequence(ps1,operator);
   println!("digit,space,operator input1: {:?}", parse(&ps2,&input1));
   println!("digit,space,operator input2: {:?}", parse(&ps2,&input2));
}
