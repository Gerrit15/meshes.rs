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
use ray::Ray;
use writer::export_to_ppm;
use camera::Camera;

fn main() {
    let scene = vec![
        Object::new_triangle(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(10.0, 0.0, 0.0), 
            Vec3::new(0.0, 10.0, 0.0), 
            Some(Vec3::new(0.0, 0.0, -10.0))
        )
    ];
    let camera = Camera::new(Vec3::new(4.0, 4.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 10, 10);
/*    for i in &camera.pixels {
        for j in i {
            print!("({:.2}, {:.2}) ", j.x, j.y)
        }
        println!()
    }*/ 
    println!("{}", camera.pixels[5][5]);
    let pixel = Ray::new(camera.location, camera.pixels[5][5]);
    println!("direction: {}", &pixel.direction);
    let closest_point = scene[0].faces[0].closest_point(camera.location);
    println!("Closest point: {closest_point}");
    let cast_pixel = pixel.cast(&scene, 10);
    match cast_pixel.0 {
        Some(_) => println!("HIT!"),
        _ => println!("not hit :(")
    }
//    let render = camera.render(&scene, 10, 1);
//    export_to_ppm(render,  Some("first_try".to_string()));
}
