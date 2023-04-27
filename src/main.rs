#![allow(dead_code)]
mod vector3;
mod triangle;
mod object;
mod ray;
mod camera;
mod writer;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;
//use ray::Ray;
use writer::export_to_ppm;
use camera::Camera;

fn main() {
    let scene = vec![
        Object::new_triangle(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(10.0, 0.0, 0.0), 
            Vec3::new(0.0, 10.0, 0.0), 
            Some(Vec3::new(0.0, 0.0, -200.0))
        )
    ];
    let camera = Camera::new(Vec3::new(4.0, 4.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 100, 100);
    let render = camera.render(&scene, 10, 1);
    export_to_ppm(render,  Some("first_try".to_string()));
}
