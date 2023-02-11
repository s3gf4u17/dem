mod lib;

fn main() {
    let mut argumentpointer = 1;
    let mut issphere = false;
    let mut width = 0;
    let mut height = 0;

    let coordinates = match &std::env::args().nth(argumentpointer).expect("no base shape given") as &str {
        "--sphere"=>{
            issphere = true;
            argumentpointer+=1;
            let r = (&std::env::args().nth(argumentpointer).expect("no radius given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let hmax = (&std::env::args().nth(argumentpointer).expect("no max height given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let file = &std::env::args().nth(argumentpointer).expect("no input file given") as &str;
            let (c,w,h) = lib::sphere(r,r,hmax,file);
            width = w; height = h; c
        },
        "--ellipsoid"=>{
            issphere = true;
            argumentpointer+=1;
            let a = (&std::env::args().nth(argumentpointer).expect("no radius given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let b = (&std::env::args().nth(argumentpointer).expect("no radius given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let hmax = (&std::env::args().nth(argumentpointer).expect("no max height given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let file = &std::env::args().nth(argumentpointer).expect("no input file given") as &str;
            let (c,w,h) = lib::sphere(a,b,hmax,file);
            width = w; height = h; c
        },
        "--plain"=>{
            argumentpointer+=1;
            let deltah = (&std::env::args().nth(argumentpointer).expect("no max height given") as &str).parse::<f64>().unwrap();
            argumentpointer+=1;
            let file = &std::env::args().nth(argumentpointer).expect("no input file given") as &str;
            let (c,w,h) = lib::plain(deltah,file);
            width = w; height = h; c
        },
        _=>panic!("non existing base shape"),
    };

    lib::export(coordinates,issphere,height,width);
}