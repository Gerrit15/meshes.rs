use core::fmt;

use crate::Vec3;

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
}

impl fmt::Display for Triangle {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vertex 1: {}\nVertex 2: {}\nVertex 3: {}\nNormal: {}", &self.vertex1, &self.vertex2, &self.vertex3, &self.normal)
    }
}

impl Copy for Triangle {}
impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        *self
    }
}
