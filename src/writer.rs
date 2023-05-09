#![allow(dead_code)]
use crate::Vec3;
use std::{fs::File, io::Write};

//as per wikipedia
/*
P3           # "P3" means this is a RGB color image in ASCII
3 2          # "3 2" is the width and height of the image in pixels
255          # "255" is the maximum value for each color
# The part above is the header
# The part below is the image data: RGB triplets
255   0   0  # red
  0 255   0  # green
  0   0 255  # blue
255 255   0  # yellow
255 255 255  # white
  0   0   0  # black
*/

pub fn export_to_ppm(pixels: Vec<Vec<Vec3>>, name: Option<String>) {
    let vrez = pixels.len();
    let hrez = pixels[0].len();
    let header = "P3\n".to_owned() + &pixels[0].len().to_string() + " " + &pixels.len().to_string() + "\n" + "255\n"; 
    let mut output_pixels = vec![];
    for i in pixels {
        for j in i {
            let r = (j.x.floor() as u64).to_string();
            let g = (j.y.floor() as u64).to_string();
            let b = (j.z.floor() as u64).to_string();
            let rgb = r + " " + &g + " " + &b + "\n";
            output_pixels.push(rgb)
        }
    }
    let output = header + &output_pixels.concat();
    let title = match name {
        Some(x) => x,
        _ => "output".to_owned()
    } + ".ppm";
    File::create("output/".to_owned() + &title).unwrap().write_all(output.as_bytes()).unwrap();
}
