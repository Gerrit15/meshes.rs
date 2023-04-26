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
//use writer::export_to_ppm;
//use camera::Camera;

fn main() {
    let scene = vec![
        Object {
            verticies: vec![
                Vec3::new(10.0, 0.0, 5.0),
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::new(0.0, 10.0, 5.0)
            ],
            origin: Vec3::new(0.0, 0.0, 0.0),
            faces: vec![
                Triangle::new(&Vec3::new(-10.0, 0.0, 0.0), &Vec3::new(10.0, 0.0, 0.0), &Vec3::new(0.0, 10.0, 0.0))
            ]
        }
    ];
    let origin = Vec3::new(5.0, 2.0, 0.0);
    let closest_point = scene[0].faces[0].closest_point(origin);
    let test = Ray::new(closest_point, Vec3::new(0.0, 0.0, 1.0));
    println!("Closest point between (5, 2, 0) and object with a height of 10, length of 10, pushed back 5: {}", closest_point);

    let test_cast = test.cast(&scene, 100);
    match test_cast.0 {
        Some(_) => println!("Hit!"),
        None => println!("no hit :(")
    }
    //
    //let camera = Camera::new(Vec3::new(0.0, 0.5, 2.0), Vec3::new(0.0, 0.0, 1.0), 100, 100);
    //let render = camera.render(&scene, 10, 1);
    //export_to_ppm(render, camera.resolution.0 as usize, camera.resolution.1 as usize, Some("first_try".to_string()));
}
