#![allow(dead_code)]
use crate::{Vec3, ray::Ray, object::Object};

pub struct Camera {
    pub location: Vec3,
    pub direction: Vec3,
    pub resolution: (u64, u64),
    pub pixels: Vec<Vec<Vec3>>
    //focal length?
}

impl Camera {
    pub fn new(location: Vec3, direction: Vec3, vres: u64, hres: u64) -> Camera {
        let z_rot = {
            if direction.x != 0.0 {
                f64::atan(direction.y/direction.x).to_degrees()
            }
            else {0.0}
        };
        let y_rot = {
            if direction.x != 0.0 {
                f64::atan(direction.z/direction.x).to_degrees()
            }
            else {0.0}
        };
        let x_rot = {
            if direction.y != 0.0 {
                f64::atan(direction.z/direction.y).to_degrees()
            }
            else {0.0}
        };

        let mut pixels = vec![];
        let xstep = 1.0/hres as f64;
        let ystep = 1.0/vres as f64;
        let mut i = 0;
        while i < vres {
            let mut buff = vec![];
            let mut j = 0;
            while j < hres {
                buff.push(
                    Vec3::new(xstep * j as f64 - 0.5, ystep * i as f64 - 0.5, 1.0)
                    .rotate_x(x_rot)
                    .rotate_y(y_rot)
                    .rotate_z(z_rot)
                );
                j += 1;
            }
            pixels.push(buff);
            i += 1;
        }

        Camera {
            location,
            direction,
            resolution: (vres, hres),
            pixels
        }
    }
    pub fn render(&self, scene: &Vec<Object>, max_steps: u64, _rays_per_pixel: u32) -> Vec<Vec<Vec3>> {
        let mut output = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                let ray = Ray::new(self.location + *j, *j);
                let cast_ray = ray.cast(scene, max_steps);
                buff.push(match cast_ray.0 {
                    Some(_) => {
                        println!("Hit");
                        Vec3::new(255.0, 255.0, 255.0)
                    },
                    _ => {
                        Vec3::new(0.0, 0.0, 255.0)
                    }
                });
            }
            output.push(buff)
        }
        output
    }

    pub fn rotate_x(&self, theta: f64) -> Camera {
        let direction = self.direction.rotate_x(theta);
        let mut pixels = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                buff.push(j.rotate_x(theta))
            }
            pixels.push(buff);
        }
        Camera { location: self.location, direction, resolution: self.resolution, pixels }
    }
    pub fn rotate_y(&self, theta: f64) -> Camera {
        let direction = self.direction.rotate_y(theta);
        let mut pixels = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                buff.push(j.rotate_y(theta))
            }
            pixels.push(buff);
        }
        Camera { location: self.location, direction, resolution: self.resolution, pixels }
    }
    pub fn rotate_z(&self, theta: f64) -> Camera {
        let direction = self.direction.rotate_z(theta);
        let mut pixels = vec![];
        for i in &self.pixels {
            let mut buff = vec![];
            for j in i {
                buff.push(j.rotate_z(theta))
            }
            pixels.push(buff);
        }
        Camera { location: self.location, direction, resolution: self.resolution, pixels }
    }
    //impl focal dist (a final multiplier on the pixel vec<vec<>>)
    //impl addition, subtraction, etc
    //impl higher res, aspect ratios
}
