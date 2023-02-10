use image::GenericImageView;
use image::GenericImage;
use image::Rgba;
use image::RgbaImage;
use image::ImageBuffer;
mod lib;

use std::fs::File;
use std::io::Write;

fn main() {
    let coordinates = lib::sphere(3396190.0,3376200.0,"test/mars.jpg");
    println!("{:?}",coordinates[coordinates.len()-1]);
}