#![allow(dead_code)]
mod vector3;
mod triangle;
mod object;
mod ray;
mod camera;
mod writer;

use std::{thread, time, sync::{Arc, Mutex}};
use rand::Rng;

//use ray::Ray;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;
//use writer::export_to_ppm;
//use camera::Camera;

fn main() {
    /*
    // CURRENT ISSUE:
    // SOMEWHERE ALONG THE PIPELINE NOT ALL TRIANGLES OF AN OBJECT ARE BEING RENDERED
    let scene = vec![
        Object::new_rect(5.0, 5.0, 5.0, Some(Vec3::new(0.0, 0.0, -25.0)), None).rotate_x(45.0)
    ];
    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 180.0, 0.0), 250, 250, true);
    let output = cam.render(&scene, 25, 1);
    export_to_ppm(output, Some("cube_test".to_string()));
    */
    let scene = vec![vec!["".to_string(); 5]; 5];
    let mut items = Mutex::new(vec![]);
    let mut handles = vec![];

    for i in 0..scene.len() {
        for j in 0..scene[i].len() {
            let x = j.clone();
            let y = i.clone();
            let handle = thread::spawn(move || {
                let a = "(".to_string() + &*x.to_string() + ", " + &*y.to_string() + ")";
                thread::sleep(time::Duration::from_secs(rand::thread_rng().gen_range(0..30)));
                items.lock().unwrap().push((x, y, a));
            });
            handles.push(handle);
        }
    }
    for handle in handles {handle.join().unwrap()}

    
/*    for i in items {
        scene[i.1][i.0] = i.2
    }*/
    for i in scene {
        for j in i {
            print!("{j}, ")
        }
        println!("")
    }
}
