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

fn main() {
    let scene = vec![
        /*Object::new_triangle(
            Vec3::new(5.0, 0.0, 0.0), 
            Vec3::new(-5.0, 0.0, 0.0), 
            Vec3::new(0.0, -5.0, 0.0), 
            Some(Vec3::new(0.0, 0.0, -25.0))
        ),
        Object::new_triangle(
            Vec3::new(5.0, 0.0, 0.0), 
            Vec3::new(-5.0, 0.0, 0.0), 
            Vec3::new(0.0, 5.0, 0.0), 
            Some(Vec3::new(5.0, -1.0, -25.0))
        )*/
        Object::new_rect(5.0, 5.0, 5.0, Some(Vec3::new(0.0, 0.0, -25.0)), None).rotate_x(45.0)
    ];
    println!("{}", scene[0].faces.len());
    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 180.0, 0.0), 250, 250, true);
    let output = cam.render(&scene, 25, 1);
    export_to_ppm(output, Some("cube_test".to_string()));
    
}
