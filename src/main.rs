mod lib;

fn main() {
    let mut argumentpointer = 1;

    let coordinates = match &std::env::args().nth(argumentpointer).expect("no base shape given") as &str {
        "--sphere"=>{
            argumentpointer+=1;
            let r = (&std::env::args().nth(argumentpointer).expect("no radius given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let hmax = (&std::env::args().nth(argumentpointer).expect("no max height given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let file = &std::env::args().nth(argumentpointer).expect("no input file given") as &str;
            lib::sphere(r,r,hmax,file)
        },
        "--ellipsoid"=>{
            argumentpointer+=1;
            let a = (&std::env::args().nth(argumentpointer).expect("no radius given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let b = (&std::env::args().nth(argumentpointer).expect("no radius given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let hmax = (&std::env::args().nth(argumentpointer).expect("no max height given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let file = &std::env::args().nth(argumentpointer).expect("no input file given") as &str;
            lib::sphere(a,b,hmax,file)
        },
        "--plain"=>{
            argumentpointer+=1;
            let deltah = (&std::env::args().nth(argumentpointer).expect("no max height given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let file = &std::env::args().nth(argumentpointer).expect("no input file given") as &str;
            lib::plain(deltah,file)
        },
        _=>panic!("non existing base shape"),
    };

    argumentpointer+=1;
    match &std::env::args().nth(argumentpointer).expect("no action given") as &str {
        "--visualize"=>println!("visualize"),
        "--export"=>println!("export"),
        _=>panic!("non existing action"),
    }
}