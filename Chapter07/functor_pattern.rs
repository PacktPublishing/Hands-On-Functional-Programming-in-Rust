use std::collections::{HashSet};

fn main()
{
   let m: Vec<u64> = vec![1, 2, 3];
   let n: Vec<u64> = m.iter().map(|x| { x*x }).collect();
   println!("{:?}", m);
   println!("{:?}", n);

   let mut a: HashSet<u64> = HashSet::new();
   a.insert(1);
   a.insert(2);
   a.insert(3);
   a.insert(4);
   let b: HashSet<u64> = a.iter().cloned().map(|x| x/2).collect();
   println!("{:?}", a);
   println!("{:?}", b);

   let sentences = vec!["this is a sentence","paragraphs have many sentences"];
   let words:Vec<&str> = sentences.iter().flat_map(|&x| x.split(" ")).collect();
   println!("{:?}", sentences);
   println!("{:?}", words);

   let v: Vec<u64> = vec![1, 2, 3];
   let s: HashSet<u64> = v.iter().cloned().map(|x| x/2).collect();
   println!("{:?}", v);
   println!("{:?}", s);
}
