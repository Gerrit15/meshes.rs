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
        Object::new_rect(1.0, 1.0, 1.0, None, None)
    ];
    let cast = Ray::new(
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0)
    );
    let _ = cast.cast(&scene);
}
