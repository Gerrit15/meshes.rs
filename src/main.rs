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
//use camera::Camera;

fn main() {
/*    let scene = vec![
        Object::new_rect(1.0, 1.0, 1.0, None, Some((45.0, 0.0, 0.0))),
        Object::new_rect(1.0, 1.0, 1.0, Some(Vec3::new(0.0, 5.0, 0.0)), None)
    ];
    let cast = Ray::new(
        Vec3::new(0.25, 0.0, 3.0),
        Vec3::new(0.0, 0.0, -1.0)
    );
    let a = cast.cast(&scene, 100);
    match a.0 {
        Some(x) => {
            println!("\nObject hit: object #{}\nData:\n{}", x.1, x.0)
        },
        _ => println!("Nothing hit")
    }
    println!("Steps taken: {}\nDistance travelled: {}", a.1.steps, a.1.distance);
*/
/*    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 4, 4);
    for i in cam.pixels {
        for j in i {
            print!("({}, {})", j.x, j.y)
        }
        println!()
    }*/
    let pixels = vec![
        vec![Vec3::new(0.0, 0.0, 0.0);3];6
    ];
    export_to_ppm(pixels, 3, 6);
}
