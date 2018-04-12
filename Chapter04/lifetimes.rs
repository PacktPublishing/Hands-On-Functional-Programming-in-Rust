fn ground_lifetime<'a>(x: &'a u64) -> &'a u64
{
   x
}

struct Ref<'a, T: 'a>(&'a T);

fn main()
{
   let x = 3;
   ground_lifetime(&x);
}
