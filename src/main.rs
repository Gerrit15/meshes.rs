mod vector3;
mod triangle;
mod object;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;

fn main() {
    let a = Object::new_tetrahedron(1.0, None, None);
    println!("{}", a);
}
