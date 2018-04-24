//It can be helpful to create shorthand names for complex types
//alias
type Name = String;

//newtype
struct NewName(String);


//Structs can be repetitive if you just want a way to store multiple values together
struct Data1
{
    a: i32,
    b: f64,
    c: String
}
struct Data2
{
    a: u32,
    b: String,
    c: f64
}

//Tuples help eliminate redundant struct definitions
//no prior type definitions are needed here and the aliases are redundant
type Tuple1 = (i32, f64, String);
type Tuple2 = (u32, String, f64);

//Standard operators can be implemented with traits
//anyone coming from an ML family language may appreciate
use std::ops::Mul;

struct Point
{
    x: i32,
    y: i32
}

impl Mul for Point
{
    type Output = Point;
    fn mul(self, other: Point) -> Point
    {
        Point
        {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

//Standard library collections etc. are generic
use std::collections::HashMap;
type CustomHashMap = HashMap<i32,u32>;

//Tagged Unions can be used to create typesafe definitions of structures that can't be safely described in pure OOP
enum BTree<T>
{
    Branch { val:T, left:Box<BTree<T>>, right:Box<BTree<T>> },
    Leaf { val:T }
}

//Commonly, Tagged Unions are used for complex data structures with many possible union options
enum Term
{
    TermVal { value: String },
    TermVar { symbol: String },
    TermApp { f: Box<Term>, x: Box<Term> },
    TermAbs { arg: String, body: Box<Term> }
}

//Traits are a bit like Object Classes
trait Data1Trait
{
    //Traits can define constructors
    fn new(a: i32, b: f64, c: String) -> Self;

    //Traits can have methods, which reference "self"
    fn get_a(&self) -> i32;
    fn get_b(&self) -> f64;
    fn get_c(&self) -> String;
}

//Traits are also like Data Classes
trait BehaviourOfShow
{
    fn show(&self) -> String;
}

fn main() {
}
