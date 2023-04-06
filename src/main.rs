use vec3_test::Vec3;

fn main() {
    let rotation_degrees = 90.0;
    let test_vec = Vec3::new(1.0, 1.0, 1.0);
    println!("Initial: {}", test_vec);
    println!("Rotate {} degrees x:\n{}", rotation_degrees, test_vec.rotate_x(rotation_degrees));
    println!("Rotate {} degrees y:\n{}", rotation_degrees, test_vec.rotate_y(rotation_degrees));
    println!("Rotate {} degrees z:\n{}", rotation_degrees, test_vec.rotate_z(rotation_degrees));
    let test_vec = Vec3::new(3.0, 2.0, 1.0);
    println!("\nVector initial: {}", &test_vec);
    println!("Magnatude: {}", test_vec.magnatude());
    println!("Vector normalized: {}\n", test_vec.normalize());
    let test_vec = Vec3::new(1.0, 0.5, 1.5);
    let second_test = Vec3::new(1.0, 0.5, 1.5);
    let third_test = Vec3::new(-1.0, 0.5, 1.5);
    println!("1: {}\n2: {}\n3: {}", &test_vec, &second_test, &third_test);
    println!("1 = 2: {}\n1 = 3: {}", &test_vec == &second_test, &test_vec == &third_test);
    let test_vec = Vec3::new(1.0, 2.0, 0.0);
    let second_test = Vec3::new(4.0, 5.0, 0.0);
    println!("\nA: {}\nB: {}\nAxB: {}", &test_vec, &second_test, test_vec.cross(second_test.clone()));
}
