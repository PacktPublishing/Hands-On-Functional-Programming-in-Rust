fn ground_lifetime<'a>(x: &'a u64) -> &'a u64
{
   x
}

struct Ref<'a, T: 'a>(&'a T);

trait Red { }

struct Ball<'a> {
   diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

static num: i32 = 5;

struct Context<'s>(&'s mut String);

impl<'s> Context<'s>
{
   fn mutate<'c>(&mut self, cs: &'c mut String) -> &'c mut String
   {
      let swap_a = self.0.pop().unwrap();
      let swap_b = cs.pop().unwrap();
      self.0.push(swap_b);
      cs.push(swap_a);
      cs
   }
}

fn main()
{
   let x = 3;
   ground_lifetime(&x);

   let obj = Box::new(Ball { diameter: &num }) as Box<Red + 'static>;

   let mut s = "outside string context abc".to_string();
   {
      //temporary context
      let mut c = Context(&mut s);
      {
         //further temporary context
         let mut s2 = "inside string context def".to_string();
         c.mutate(&mut s2);
         println!("s2 {}", s2);
      }
   }
   println!("s {}", s);

}
