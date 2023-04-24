mod vector3;
mod triangle;
mod object;
mod ray;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;
use ray::Ray;

fn main() {
    let scene = vec![
        Object {
            verticies: vec![Vec3::new(-1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
            origin: Vec3::new(0.0, 0.0, 0.0),
            faces: vec![Triangle::new(&Vec3::new(-1.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0), &Vec3::new(0.0, 1.0, 0.0))]
        }
    ];
    let cast = Ray::new(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, -1.0)
    );
    let a = cast.cast(&scene, 100);
    match a.0 {
        Some(x) => {
            println!("Object hit: {}", x)
        },
        _ => println!("Nothing hit")
    }
    println!("Steps taken: {}", a.1.steps);
}
