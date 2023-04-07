mod vector3;
use vector3::Vec3;
use vec3_test;

fn main() {
    let b = Vec3::new(1.0, 2.0, 3.0);
    let n = Vec3::new(1.0, 1.0, 0.0);
    let b_prime = b.reflect(n);
    println!("b': {}", b_prime);
}
