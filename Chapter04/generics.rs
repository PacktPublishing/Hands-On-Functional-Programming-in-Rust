struct Cat
{
   weight: f64,
   speed: f64
}

struct Dog
{
   weight: f64,
   speed: f64
}

trait Animal
{
   fn max_speed(&self) -> f64;
}

impl Animal for Cat
{
   fn max_speed(&self) -> f64
   {
      self.speed
   }
}

impl Animal for Dog
{
   fn max_speed(&self) -> f64
   {
      self.speed
   }
}

struct SqueakyToy
{
   weight: f64
}

struct Stick
{
   weight: f64
}

trait Toy
{
   fn weight(&self) -> f64;
}

impl Toy for SqueakyToy
{
   fn weight(&self) -> f64
   {
      self.weight
   }
}

impl Toy for Stick
{
   fn weight(&self) -> f64
   {
      self.weight
   }
}

struct AnimalChasingToy<A: Animal, T: Toy>
{
   animal: A,
   toy: T
}

trait AnimalChasesToy<A: Animal, T: Toy>
{
   fn chase(&self);
}

impl<A: Animal, T: Toy> AnimalChasesToy<A, T> for AnimalChasingToy<A, T>
{
   fn chase(&self)
   {
      println!("chase")
   }
}

fn main()
{
}
