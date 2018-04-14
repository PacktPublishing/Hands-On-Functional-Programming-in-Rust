#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::fmt;
use serde::de::{self, Visitor};

struct Foo
{
   bar: Box<u64>
}

#[derive(Clone,Serialize,Deserialize)]
enum T_Enum
{
   S1(S1),
   S2(S2),
}

trait T {
   fn as_enum(&self) -> T_Enum;
}

#[derive(Clone,Serialize,Deserialize)]
struct S1;
impl T for S1 {
   fn as_enum(&self) -> T_Enum
   {
      T_Enum::S1(self.clone())
   }
}

#[derive(Clone,Serialize,Deserialize)]
struct S2;
impl T for S2 {
   fn as_enum(&self) -> T_Enum
   {
      T_Enum::S2(self.clone())
   }
}

#[derive(Serialize,Deserialize)]
struct Container
{
   field: Box<T>
}

impl Serialize for Box<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.as_enum().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Box<T> {
    fn deserialize<D>(deserializer: D) -> Result<Box<T>, D::Error>
        where D: Deserializer<'de>
    {
        let result = T_Enum::deserialize(deserializer);
        match result
        {
           Result::Ok(te) => {
              match te {
                 T_Enum::S1(s1) => Result::Ok(Box::new(s1.clone())),
                 T_Enum::S2(s2) => Result::Ok(Box::new(s2.clone()))
              }
           }
           Result::Err(err) => Result::Err(err)
        }
    }
}

fn main()
{
   let bt: Box<T> = Box::new(S1);
   let s = serde_json::to_string(&bt).unwrap();
   let bt: Box<T> = serde_json::from_str(s.as_str()).unwrap();
}
