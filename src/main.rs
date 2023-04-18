mod vector3;
mod triangle;
mod object;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;

fn main() {
    let vert1 = Vec3::new(0.0, 3.0, 0.0);
    let vert2 = Vec3::new(0.0, 0.0, 0.0);
    let vert3 = Vec3::new(0.0, 0.0, 1.0);
    let face = Triangle::new(&vert1, &vert2, &vert3);
    let point = Vec3::new(1.0, 1.5, 0.5);
    println!("{}", face.closest_point(point));
}
