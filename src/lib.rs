use image::GenericImageView;
use std::f64::consts::PI;
use libm;

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
        coordinates[i]=(x,y,z);
    }
    coordinates
} // hirvonen algorithm used to transform (phi lambda h) to (x y z) coordinates

pub fn plain(deltah:f64,file:&str) -> Vec<(f64,f64,f64)> {
    Vec::<(f64,f64,f64)>::new()
}