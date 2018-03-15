fn main() {

    (0..10).chain(10..20);

    (0..10).zip(10..20);

    (0..10).enumerate();

    (0..10).inspect(|x|{ println!("value {}", *x) });

    (0..10).map(|x| x*x);

    (0..10).filter(|x| *x<3);

    (0..10).fold(0, |x,y| x+y);

    for i in (0..10) {}

    let v: Vec<u32> = (0..10).collect();

}
