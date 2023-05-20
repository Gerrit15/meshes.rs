#![allow(dead_code)]
mod vector3;
mod triangle;
mod object;
mod ray;
mod camera;
mod writer;
//use ray::Ray;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;
use writer::export_to_ppm;
use camera::Camera;
use std::sync::Arc;

fn main() {
    
    //so a few issues
    //first issue: at around 185 by 185 pixels, it freaks the fuck out, idk why, it's a full on OS error
    //second issue: figure out what the heck is going on with the 90 degrees of rotation on the camera
    //not an issue, third piece to work on, enumerating the object types
    
    let scene = vec![
        Object::new_triangle(Vec3::new(0.0, 0.0, 0.0), Vec3::new(5.0, 0.0, 0.0), Vec3::new(0.0, 5.0, 0.0), Some(Vec3::new(0.0, 0.0 , 25.0)))
//        Object::new_rect(5.0, 5.0, 5.0, Some(Vec3::new(0.0, 0.0, -25.0)), None).rotate_x(45.0)
    ];
    println!("Num threads to be spawned: {}", 185*185);
    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(180.0, 0.0, 0.0), 185, 185, true);
    let output = cam.render(Arc::new(scene), 25, 1);
    export_to_ppm(output, Some("multithread_test".to_string()));
   }
