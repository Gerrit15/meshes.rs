mod vector3;
mod triangle;
mod object;
use vector3::Vec3;
use triangle::Triangle;

fn main() {
    let points = vec![
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(3.0, 3.0, 1.0),
        Vec3::new(5.0, 1.0, -1.0)
    ];
    let tri = Triangle::new(&points[0], &points[1], &points[2]);
    println!("{}", &tri);
}
