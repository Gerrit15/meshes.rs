#![allow(dead_code)]
use core::fmt;
use crate::Vec3;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vertex1: Vec3,
    pub vertex2: Vec3,
    pub vertex3: Vec3,
    pub normal: Vec3
}

impl Triangle {
    pub fn new (a: &Vec3, b: &Vec3, c: &Vec3) -> Triangle {
        Triangle {
            vertex1: *a,
            vertex2: *b,
            vertex3: *c,
            normal: Self::gen_normal(*a, *b, *c),
        }
    }
    //based on the function N(a, b, c) = |(B - A) X (B - C)|
    //Because it generates the cross product of the two legs of a triangle, then normalizes it
    fn gen_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        let ba = b-a;
        let bc = b-c;
        ba.cross(bc).normalize()
    }
    pub fn rotate_x(&mut self, theta: f64) {
        self.vertex1 = self.vertex1.rotate_x(theta);
        self.vertex2 = self.vertex2.rotate_x(theta);
        self.vertex3 = self.vertex3.rotate_x(theta);
    }
    pub fn rotate_y(&mut self, theta: f64) {
        self.vertex1 = self.vertex1.rotate_x(theta);
        self.vertex2 = self.vertex2.rotate_x(theta);
        self.vertex3 = self.vertex3.rotate_x(theta);
    }
    pub fn rotate_z(&mut self, theta: f64) {
        self.vertex1 = self.vertex1.rotate_z(theta);
        self.vertex2 = self.vertex2.rotate_z(theta);
        self.vertex3 = self.vertex3.rotate_z(theta);
    }
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        //conceptually, this is generating the closest point on the base 
        //of the triangle, then taking that point and getting the closest point 
        //between that and the top point of the triangle 
        let base_point = Vec3::closest_on_line(self.vertex1, self.vertex2, point);
        return Vec3::closest_on_line(base_point, self.vertex3, point)

    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vertex 1: {}\nVertex 2: {}\nVertex 3: {}\nNormal: {}", &self.vertex1, &self.vertex2, &self.vertex3, &self.normal)
    }
}
