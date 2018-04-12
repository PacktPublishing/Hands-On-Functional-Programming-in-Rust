
type JSJIT = u64;

enum JSJITorExpr {
   Jit { label: JSJIT },
   Expr { expr: Box<JSExpr> }
}

enum JSExpr {
   Integer { value: u64 },
   String { value: String },
   OperatorAdd { lexpr: Box<JSJITorExpr>, rexpr: Box<JSJITorExpr> },
   OperatorMul { lexpr: Box<JSJITorExpr>, rexpr: Box<JSJITorExpr> }
}

fn jump(l: JSJIT) -> JSJITorExpr
{
   //jump to compiled code
   //this depends on implementation
   //so we will just leave this as a stub
   JSJITorExpr::Jit { label: 0 }
}
fn eval(e: JSJITorExpr) -> JSJITorExpr
{
   match e
   {
      JSJITorExpr::Jit { label: label } => jump(label),
      JSJITorExpr::Expr { expr: expr } => {
         let rawexpr = *expr;
         match rawexpr
         {
            JSExpr::Integer {..} => JSJITorExpr::Expr { expr: Box::new(rawexpr) },
            JSExpr::String {..} => JSJITorExpr::Expr { expr: Box::new(rawexpr) },
            JSExpr::OperatorAdd { lexpr: l, rexpr: r } => {
               let l = eval(*l);
               let r = eval(*r);
               //call add op codes for possible l,r representations
               //should return wrapped value from above
               JSJITorExpr::Jit { label: 0 }
            }
            JSExpr::OperatorMul { lexpr: l, rexpr: r } => {
               let l = eval(*l);
               let r = eval(*r);
               //call mul op codes for possible l,r representations
               //should return wrapped value from above
               JSJITorExpr::Jit { label: 0 }
            }
         }
      }
   }
}

pub trait HList: Sized {}

pub struct HNil;
impl HList for HNil {}

pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}
impl<H, T: HList> HList for HCons<H, T> {}
impl<H, T> HCons<H, T> {
    pub fn pop(self) -> (H, T) {
        (self.head, self.tail)
    }
}

fn main()
{
   let hl = HCons {
      head: 2,
      tail: HCons {
         head: "abcd".to_string(),
         tail: HNil
      }
   };
   let (h1,t1) = hl.pop();
   let (h2,t2) = t1.pop();
   //this would fail
   //HNil has no .pop method
   //t2.pop();

}
