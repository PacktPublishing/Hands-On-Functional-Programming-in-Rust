use std::collections::{HashSet};

struct WebCamera;
#[derive(Debug)]
enum VisibleEmotion {
   Anger,
   Contempt,
   Disgust,
   Fear,
   Happiness,
   Neutral,
   Sadness,
   Surprise
}
#[derive(Debug,Clone)]
struct BoundingBox {
   top: u64,
   left: u64,
   height: u64,
   width: u64
}
#[derive(Debug)]
enum CameraFilters {
   Sparkles,
   Rain,
   Fire,
   Disco
}

impl WebCamera {
   fn map_emotion<T,F>(&self, translate: F) -> Vec<(BoundingBox,T)>
      where F: Fn(VisibleEmotion) -> T {
      //Simulate emotion extracted from WebCamera
      vec![
         (BoundingBox { top: 1, left: 1, height: 1, width: 1 }, VisibleEmotion::Anger),
         (BoundingBox { top: 1, left: 1, height: 1, width: 1 }, VisibleEmotion::Sadness),
         (BoundingBox { top: 4, left: 4, height: 1, width: 1 }, VisibleEmotion::Surprise),
         (BoundingBox { top: 8, left: 1, height: 1, width: 1 }, VisibleEmotion::Neutral)
      ].into_iter().map(|(bb,emt)| {
        (bb, translate(emt))
      }).collect::<Vec<(BoundingBox,T)>>()
   }
   fn flatmap_emotion<T,F,U:IntoIterator<Item=T>>(&self, mut translate: F) -> Vec<(BoundingBox,T)>
      where F: FnMut(VisibleEmotion) -> U {
      //Simulate emotion extracted from WebCamera
      vec![
         (BoundingBox { top: 1, left: 1, height: 1, width: 1 }, VisibleEmotion::Anger),
         (BoundingBox { top: 1, left: 1, height: 1, width: 1 }, VisibleEmotion::Sadness),
         (BoundingBox { top: 4, left: 4, height: 1, width: 1 }, VisibleEmotion::Surprise),
         (BoundingBox { top: 8, left: 1, height: 1, width: 1 }, VisibleEmotion::Neutral)
      ].into_iter().flat_map(|(bb,emt)| {
        translate(emt).into_iter().map(move |t| (bb.clone(), t))
      }).collect::<Vec<(BoundingBox,T)>>()
   }
}

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

   let camera = WebCamera;
   let emotes: Vec<(BoundingBox,VisibleEmotion)> = camera.map_emotion(|emt| {
      match emt {
        VisibleEmotion::Anger |
        VisibleEmotion::Contempt |
        VisibleEmotion::Disgust |
        VisibleEmotion::Fear |
        VisibleEmotion::Sadness => VisibleEmotion::Happiness,
        VisibleEmotion::Neutral |
        VisibleEmotion::Happiness |
        VisibleEmotion::Surprise => VisibleEmotion::Sadness
      }
   });
   let filters: Vec<(BoundingBox,CameraFilters)> = camera.flatmap_emotion(|emt| {
      match emt {
        VisibleEmotion::Anger |
        VisibleEmotion::Contempt |
        VisibleEmotion::Disgust |
        VisibleEmotion::Fear |
        VisibleEmotion::Sadness => vec![CameraFilters::Sparkles, CameraFilters::Rain],
        VisibleEmotion::Neutral |
        VisibleEmotion::Happiness |
        VisibleEmotion::Surprise => vec![CameraFilters::Disco]
      }
   });
   println!("{:?}",emotes);
   println!("{:?}",filters);
}
