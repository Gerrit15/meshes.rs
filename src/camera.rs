#![allow(dead_code)]
use std::{sync::{Arc, Mutex}, thread};

use crate::{Vec3, ray::Ray, object::Object};

pub struct Camera {
    pub location: Vec3,
    pub angle: Vec3,
    pub resolution: (u64, u64),
    pub pixels: Vec<Vec<Vec3>>
    //focal length?
}

impl Camera {
    pub fn new(location: Vec3, angle: Vec3, vres: u64, hres: u64, is_plane: bool) -> Camera {
        let mut pixels = vec![];
        let xstep = 1.0/hres as f64;
        let ystep = 1.0/vres as f64;
        let mut i = 0;
        while i < vres {
            let mut buff = vec![];
            let mut j = 0;
            while j < hres {
                let x = xstep * j as f64 - 0.5;
                let y = ystep * i as f64 - 0.5;
                let z = match is_plane {
                    true => {1.0}
                    false => {(1.0 - (x*x) - (y*y)).sqrt()}
                };
                let pixel = Vec3::new(x, y, z)
                    .rotate_x(angle.x)
                    .rotate_y(angle.y)
                    .rotate_z(angle.z);
//                println!("{pixel}");
                buff.push(pixel);
                j += 1;
            }
            pixels.push(buff);
            i += 1;
        }
/*        for i in &pixels {
           for j in i {
               println!("Pixel: {}", j);
           }
       }*/
        let cam = Camera {location, angle: Vec3::new(0.0, 0.0, 0.0), resolution: (vres, hres), pixels}.rotate_x(angle.x).rotate_y(angle.y).rotate_z(angle.z);
        return cam
    }
    pub fn render(&self, scene: Arc<Vec<Object>>, max_steps: u64, _rays_per_pixel: u32) -> Vec<Vec<Vec3>> {
/*        let mut output = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                let ray = Ray::new(self.location, *j);
                let cast_ray = ray.cast(Arc::clone(&scene), max_steps);
                buff.push(
                    match cast_ray.0 {
                        Some(_) => {
                            println!("hit!");
                            Vec3::new(255.0, 255.0, 255.0)
                        },
                        _ => {
                            println!("miss :(");
                            Vec3::new(0.0, 0.0, 0.0)
                        }
                    }
                );
            }
            output.push(buff)
        }
        output*/
        let pixel_stack: Arc<Mutex<Vec<(usize, usize, Vec3)>>> = Arc::new(Mutex::new(vec![]));
        let mut handles = vec![];
        for i in 0..self.pixels.len() {
            let y = i.clone();
            for j in 0..self.pixels[i].len() {
                let x = j.clone();
                let max_steps = max_steps.clone();
                let ray = Ray::new(self.location, *&self.pixels[y][x]);
                let scene_clone = Arc::clone(&scene);
                let stack_clone = Arc::clone(&pixel_stack);

                handles.push(thread::spawn(move || {
                    let cast_ray = ray.cast(scene_clone, max_steps.clone());
                    let color = match cast_ray.0 {
                        Some(_) => {
                            Vec3::new(255.0, 255.0, 255.0)
                        },
                        None => {
                            Vec3::new(0.0, 0.0, 0.0)
                        }
                    };
                    stack_clone.lock().unwrap().push((x, y, color))
                }));
            }
        }
        for handle in handles {handle.join().unwrap()}
        let mut output = vec![vec![Vec3::new(0.0, 0.0, 0.0); self.pixels[0].len()]; self.pixels.len()];
        for i in &*pixel_stack.lock().unwrap() {
            output[i.0][i.1] = i.2;
        }
        output
    }

    pub fn rotate_x(&self, theta: f64) -> Camera {
        let angle = self.angle + (theta, 0.0, 0.0);
        let mut pixels = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                buff.push(j.rotate_x(theta))
            }
            pixels.push(buff);
        }
        Camera { location: self.location, angle, resolution: self.resolution, pixels }
    }
    pub fn rotate_y(&self, theta: f64) -> Camera {
        let angle = self.angle + (0.0, theta, 0.0);
        let mut pixels = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                buff.push(j.rotate_y(theta))
            }
            pixels.push(buff);
        }
        Camera { location: self.location, angle, resolution: self.resolution, pixels }
    }
    pub fn rotate_z(&self, theta: f64) -> Camera {
        let angle = self.angle + (0.0, 0.0, theta);
        let mut pixels = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                buff.push(j.rotate_z(theta))
            }
            pixels.push(buff);
        }
        Camera { location: self.location, angle, resolution: self.resolution, pixels }
    }
    //impl focal dist (a final multiplier on the pixel vec<vec<>>)
    //impl addition, subtraction, etc
    //impl higher res, aspect ratios
}
