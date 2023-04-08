use core::fmt;

use crate::Vec3;

pub struct Triangle <'a> {
    pub vertex1: &'a Vec3,
    pub vertex2: &'a Vec3,
    pub vertex3: &'a Vec3,
    pub normal: Vec3
}

impl <'a>Triangle<'a> {
    pub fn new (a: &'a Vec3, b: &'a Vec3, c: &'a Vec3) -> Triangle<'a> {
        Triangle {
            vertex1: a,
            vertex2: b,
            vertex3: c,
            normal: Self::gen_normal(a, b, c),
        }
    }
    //based on the function N(a, b, c) = |(B - A) X (B - C)|
    //Because it generates the cross product of the two legs of a triangle, then normalizes it
    fn gen_normal(a: &'a Vec3, b: &'a Vec3, c: &'a Vec3) -> Vec3 {
        let ba = *b-*a;
        let bc = *b-*c;
        ba.cross(bc).normalize()
    }
}

impl<'a> fmt::Display for Triangle<'a> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vertex 1: {}\nVertex 2: {}\nVertex 3: {}\nNormal: {}", &self.vertex1, &self.vertex2, &self.vertex3, &self.normal)
    }
}
