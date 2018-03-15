struct MyStruct {
    a: u32,
    b: f32,
    c: String
}

enum Term {
    TermVal { value: String },
    TermVar { symbol: String },
    TermApp { f: Box<Term>, x: Box<Term> },
    TermAbs { arg: String, body: Box<Term> }
}

fn main() {

    let x = {
        fn f(x: u32) -> u32 {
            x*x
        }
        let y = f(5);
        y*3
    };

    let x;
    if true {
        x = 1;
    } else {
        x = 2;
    }

    let x = if true { 1 } else { 2 };

    MyStruct {
        a: 1,
        b: 1.0,
        c: "".to_string()
    };

    (1, 1.0, "".to_string());

    let t = Term::TermVar {
        symbol: "".to_string()
    };
    match t {
       Term::TermVal { value: v1 } => v1,
       Term::TermVar { symbol: v1 } => v1,
       Term::TermApp { f: v1, x: v2 } => "TermApp(?,?)".to_string(),
       Term::TermAbs { arg: v1, body: v2 } => "TermAbs(?,?)".to_string()
    };

}
