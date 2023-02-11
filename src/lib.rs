use image::GenericImageView;
use std::f64::consts::PI;
use libm;
use std::fs::File;
use std::io::Write;

// INPUT - filename && elipsoid info && max height
// OUTPUT - vector of xyz coordinates
pub fn sphere(a:f64,b:f64,hmax:f64,file:&str) -> Vec<(f64,f64,f64)> {
    // calculate constant eccentricity of the ellipsoid
    let e2 = (a*a-b*b)/(a*a);
    // read raster input file and get its size
    let raster = image::open(file).unwrap();
    let (raster_width,raster_height) = raster.dimensions();
    // create place for new coordinates
    let mut coordinates : Vec<(f64,f64,f64)> = Vec::new();
    // insert into vector (phi lambda h) coordinates calculated from raster
    for i in 0..raster_height*raster_width {
        let phi = PI/2.0-(PI/raster_height as f64)*(i as f64 / raster_width as f64);
        let lambda = -PI+(2.0*PI/raster_width as f64)*(i as f64 % raster_width as f64);
        let pixel = raster.get_pixel(i%raster_width,i/raster_width);
        let h = (pixel[0] as f64 +pixel[1] as f64 +pixel[2] as f64)/3.0/255.0*hmax;
        coordinates.push((phi,lambda,h));
    }
    // transform coordinates to (x y z)
    for i in 0..coordinates.len() {
        let (phi,lambda,h) = coordinates.get(i).unwrap();
        let n = a/libm::sqrt(1.0-e2*libm::sin(*phi)*libm::sin(*phi));
        let x = (n+*h)*libm::cos(*phi)*libm::cos(*lambda);
        let y = (n+*h)*libm::cos(*phi)*libm::sin(*lambda);
        let z = (n*(1.0-e2)+*h)*libm::sin(*phi);
        coordinates[i]=(y,z,x);
    }
    coordinates
} // hirvonen algorithm used to transform (phi lambda h) to (x y z) coordinates

pub fn plain(deltah:f64,file:&str) -> Vec<(f64,f64,f64)> {
    let raster = image::open(file).unwrap();
    let (raster_width,raster_height) = raster.dimensions();
    let mut coordinates : Vec<(f64,f64,f64)> = Vec::new();
    for i in 0..raster_width*raster_height {
        let pixel = raster.get_pixel(i%raster_width,i/raster_width);
        let h = (pixel[0] as f64 +pixel[1] as f64 +pixel[2] as f64)/3.0/255.0*deltah;
        coordinates.push(((i%raster_width).into(),h,(i/raster_width).into()))
    }
    coordinates
}

pub fn export(coordinates:Vec<(f64,f64,f64)>) {
    let (raster_height,raster_width) = (300,300);
    println!("creating mesh");
    let mut file = File::create("topography.obj").expect("failed");
    let mut contents = String::new();
    contents.push_str("o Topography\n");
    // fill with vertices
    println!("writing vertices");
    for i in 0..coordinates.len() {
        let (x,y,z) = coordinates.get(i).unwrap();
        contents.push_str(&format!("v {} {} {}\n",x,y,z)as &str);
    }
    println!("writing faces");
    // fill with faces
    for y in 1..raster_height {
        for x in 1..raster_width {
            contents.push_str(&format!("f {} {} {} {}\n",raster_width*(y-1)+x,raster_width*(y-1)+x+1,raster_width*y+x+1,raster_width*y+x) as &str);
        }
    } // at this point we always have an unconnected space on meridian 180
    // println!("fixing meridian 180");
    // for y in 1..raster_height {
    //     contents.push_str(&format!("f {} {} {} {}\n",raster_width*y,raster_width*(y-1)+1,raster_width*(y)+1,raster_width*(y+1)) as &str);
    // }
    println!("saving mesh");
    file.write_all(contents.as_bytes()).expect("failed");
}

pub fn visualize() {
    
}