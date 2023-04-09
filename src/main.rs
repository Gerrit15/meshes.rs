mod vector3;
mod triangle;
mod object;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;

fn main() {
    let fred = Object::new_rect(3.0, 2.0, 2.0, Vec3::new(0.0, 0.0, 0.0), (0.0, 0.0, 0.0));
    println!("{}", fred);
}
