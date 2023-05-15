#![allow(dead_code)]
mod vector3;
mod triangle;
mod object;
mod ray;
mod camera;
mod writer;
//use ray::Ray;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;
use writer::export_to_ppm;
use camera::Camera;
use std::sync::Arc;

fn main() {
    
    // CURRENT ISSUE:
    // SOMEWHERE ALONG THE PIPELINE NOT ALL TRIANGLES OF AN OBJECT ARE BEING RENDERED
    // that and the fact that I can't seem to render anything anymore, idk man
    let scene = vec![
//        Object::new_triangle(Vec3::new(-5.0, 0.0, 0.0), Vec3::new(5.0, 0.0, 0.0), Vec3::new(0.0, 5.0, 0.0), Some(Vec3::new(0.0, 0.0 , -25.0)))
        Object::new_rect(5.0, 5.0, 5.0, Some(Vec3::new(0.0, 0.0, -25.0)), None).rotate_x(45.0)
    ];
    let cam = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 180.0, 0.0), 250, 250, true);
    let output = cam.render(Arc::new(scene), 25, 1);
    export_to_ppm(output, Some("multithread_test".to_string()));
    
/*    let mut scene = vec![vec!["".to_string(); 5]; 5];
    let items: Arc<Mutex<Vec<(usize, usize, String)>>> = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..scene.len() {
        for j in 0..scene[i].len() {
            let x = j.clone();
            let y = i.clone();
            let arc_clone = Arc::clone(&items);
            let handle = thread::spawn(move || {
                let a = "(".to_string() + &*x.to_string() + ", " + &*y.to_string() + ")";
                thread::sleep(time::Duration::from_secs(rand::thread_rng().gen_range(0..2)));
                arc_clone.lock().unwrap().push((x, y, a));
            });
            handles.push(handle);
        }
    }
    for handle in handles {handle.join().unwrap()}

    
    for i in &*items.lock().unwrap() {
        scene[i.1][i.0] = i.2.to_string()
    }
    for i in scene {
        for j in i {
            print!("{j}, ")
        }
        println!("")
    }*/
}
