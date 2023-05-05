#![allow(dead_code)]
mod vector3;
mod triangle;
mod object;
mod ray;
mod camera;
mod writer;
use ray::Ray;
use vector3::Vec3;
use triangle::Triangle;
use object::Object;
use writer::export_to_ppm;
//use camera::Camera;

fn main() {
    let scene = vec![
        Object::new_triangle(
            Vec3::new(100.0, 0.0, 0.0), 
            Vec3::new(00.0, 0.0, 0.0), 
            Vec3::new(0.0, -100.0, 0.0), 
//            None
            Some(Vec3::new(0.0, 0.0, -500.0))
        )
    ];
    let hres = 250;
    let vres = 250;
    let xstep = 1.0/(hres as f64);
    let ystep = 1.0/(vres as f64);

    let mut projection_matrix = vec![];
    let mut i = 0;
    while i < vres{
        let mut buff = vec![];
        let mut j = 0;
        while j < hres {
            let x = xstep * j as f64 - 0.5;
            let y = ystep * i as f64 - 0.5;
            let z = (1.0 - (x*x) - (y*y)).sqrt() * -1.0;
            buff.push(Vec3::new(x, y, z));
            j += 1;
        }
        i += 1;
        projection_matrix.push(buff);
    }
   let mut output = vec![];

    for i in projection_matrix {
        let mut buff = vec![];
        for j in i {
            let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), j);
            let cast_ray = ray.cast(&scene, 15);
            match cast_ray.0 {
                Some(_) => {
                    buff.push(Vec3::new(255.0, 255.0, 255.0))
                },
                None => {
                    buff.push(Vec3::new(0.0, 0.0, 0.0))
                }
            }
        }
        output.push(buff);
    }
    export_to_ppm(output, Some("Messing with distance".to_string()));
    
}
