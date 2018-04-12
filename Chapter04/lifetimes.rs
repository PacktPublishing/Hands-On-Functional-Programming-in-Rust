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

fn main()
{
   let x = 3;
   ground_lifetime(&x);

   let obj = Box::new(Ball { diameter: &num }) as Box<Red + 'static>;
}
