#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

struct Foo
{
   bar: Box<u64>
}

trait T {}

#[derive(Serialize,Deserialize)]
struct S1;
impl T for S1 {}

#[derive(Serialize,Deserialize)]
struct S2;
impl T for S2 {}

#[derive(Serialize,Deserialize)]
struct Container
{
   field: Box<T>
}

fn main()
{
}
