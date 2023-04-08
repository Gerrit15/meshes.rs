mod vector3;
mod triangle;
use vector3::Vec3;
use triangle::Triangle;

fn main() {
    let points = vec![
        Vec3::new(1.0 , 1.0 , 0.0),
        Vec3::new(1.0 , 4.0 , 0.0),
        Vec3::new(6.0 , 4.0 , 0.0)
    ];
    let tri = Triangle::new(&points[0], &points[1], &points[2]);
    println!("{}", &tri);
}
