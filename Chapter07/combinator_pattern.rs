#[macro_use]
extern crate chomp;
use chomp::prelude::*;

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


trait ParseState<A> {
   fn next(&self) -> (Box<ParseState<A>>,Option<char>);
}
struct ParseRCon<A,B>(A,Result<Option<B>,String>);
enum ParseOutput<A> {
   Success(A),
   Failure(String)
}

fn parse_token<St,A,T>(t: T) -> impl (Fn(Box<ParseState<St>>) -> ParseRCon<Box<ParseState<St>>,A>)
   where T: 'static + Fn(char) -> Option<A> {
   move |st: Box<ParseState<St>>| {
      let (next_state,next_char) = st.next();
      match next_char {
         Some(c) => ParseRCon(next_state,Ok(t(c))),
         None => ParseRCon(st,Err("end of input".to_string()))
      }
   }
}

fn parse_satisfy<St,T>(t: T) -> impl (Fn(Box<ParseState<St>>) -> ParseRCon<Box<ParseState<St>>,char>)
   where T: 'static + Fn(char) -> bool {
   parse_token(move |c| if t(c) {Some(c)} else {None})
}

fn parse_return<St,A: Clone>(a: A) -> impl (Fn(Box<ParseState<St>>) -> ParseRCon<Box<ParseState<St>>,A>) {
   move |st| { ParseRCon(st,Ok(Some(a.clone()))) }
}

fn parse_bind<St,A,B,P1,P2,B1>(p1: P1, b1: B1)
   -> impl Fn(Box<ParseState<St>>) -> ParseRCon<Box<ParseState<St>>,B>
   where P1: Fn(Box<ParseState<St>>) -> ParseRCon<Box<ParseState<St>>,A>,
         P2: Fn(Box<ParseState<St>>) -> ParseRCon<Box<ParseState<St>>,B>,
         B1: Fn(A) -> P2 {
   move |st| {
      match p1(st) {
         ParseRCon(nst,Ok(Some(a))) => b1(a)(nst),
         ParseRCon(nst,Ok(None)) => ParseRCon(nst,Err("bind failed".to_string())),
         ParseRCon(nst,Err(err)) => ParseRCon(nst,Err(err))
      }
   }
}

fn mzero<St,A>(st: Box<ParseState<St>>) -> ParseRCon<Box<ParseState<St>>,A> {
   ParseRCon(st,Err("mzero".to_string()))
}

/*
TODO
  val mplus  : ('st, 'a) parser -> ('st, 'a) parser -> 
               ('st, 'a) parser
  val (>>)   : ('st, 'a) parser -> ('st, 'b) parser ->
               ('st, 'b) parser

  val chars  : char list -> ('st,char list) char_parser
  many
  sepBy
  <|>
*/

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
}
