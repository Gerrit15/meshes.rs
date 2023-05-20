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
    pub fn rotate_x(&self, theta: f64) -> Triangle {
        let a = self.vertex1.rotate_x(theta);
        let b = self.vertex2.rotate_x(theta);
        let c = self.vertex3.rotate_x(theta);
        Triangle::new(&a, &b, &c)
    }
    pub fn rotate_y(&self, theta: f64) -> Triangle {
        let a = self.vertex1.rotate_y(theta);
        let b = self.vertex2.rotate_y(theta);
        let c = self.vertex3.rotate_y(theta);
        Triangle::new(&a, &b, &c)
    }
    pub fn rotate_z(&self, theta: f64) -> Triangle {
        let a = self.vertex1.rotate_z(theta);
        let b = self.vertex2.rotate_z(theta);
        let c = self.vertex3.rotate_z(theta);
        Triangle::new(&a, &b, &c)
    }
    pub fn closest_point(&self, origin: Vec3, point: Vec3) -> Vec3 {
        //conceptually, this is generating the closest point on the base 
        //of the triangle, then taking that point and getting the closest point 
        //between that and the top point of the triangle
        let base_point = Vec3::closest_on_line(self.vertex1 + origin, self.vertex2 + origin, point);
        return Vec3::closest_on_line(base_point, self.vertex3 + origin, point)
    }
    /*
    float udTriangle( vec3 p, vec3 a, vec3 b, vec3 c )
    {
      vec3 ba = b - a; vec3 pa = p - a;
      vec3 cb = c - b; vec3 pb = p - b;
      vec3 ac = a - c; vec3 pc = p - c;
      vec3 nor = cross( ba, ac );

      return sqrt(
        (sign(dot(cross(ba,nor),pa)) +
         sign(dot(cross(cb,nor),pb)) +
         sign(dot(cross(ac,nor),pc))<2.0)
         ?
         min( min(
         dot2(ba*clamp(dot(ba,pa)/dot2(ba),0.0,1.0)-pa),
         dot2(cb*clamp(dot(cb,pb)/dot2(cb),0.0,1.0)-pb) ),
         dot2(ac*clamp(dot(ac,pc)/dot2(ac),0.0,1.0)-pc) )
         :
         dot(nor,pa)*dot(nor,pa)/dot2(nor) );
    }
    */
    pub fn sdf(&self, p: Vec3, origin: Vec3) -> f64 {
        let ba = (self.vertex2 + origin) - (self.vertex1 + origin);
        let cb = (self.vertex3 + origin) - (self.vertex2 + origin);
        let ac = (self.vertex1 + origin) - (self.vertex3 + origin);
        let pa = p - (self.vertex1 + origin);
        let pb = p - (self.vertex2 + origin);
        let pc = p - (self.vertex3 + origin);
        let sign_sum = Self::sign(ba.cross(self.normal)*pa) + Self::sign(cb.cross(self.normal)*pb) + Self::sign(ac.cross(self.normal)*pc);
        let a = ba * ((ba*pa) / (ba*ba)).clamp(0.0, 1.0) - pa;
        let a = a*a;
        let b = cb * ((cb*pb) / (cb*cb)).clamp(0.0, 1.0) - pb;
        let b = b*b;
        let c = ac * ((ac*pc) / (ac*ac)).clamp(0.0, 1.0) - pc;
        let c = c*c;
        let min_abc = Self::minimum(a, b, c);
        if sign_sum < 2.0 {return min_abc.sqrt()}
        else {return ((self.normal*pa)*(self.normal*pa)*(self.normal*self.normal)).sqrt()} 
    }
    fn sign(a: f64) -> f64 {
        a.signum()
    }
    fn minimum(a: f64, b: f64, c:f64) -> f64 {
        let out = {
            if a > b {a}
            else if a < b {b}
            else {a}
        };
        if out > c {return out}
        else if out < c {return c}
        else {return out}
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vertex 1: {}\nVertex 2: {}\nVertex 3: {}\nNormal: {}", &self.vertex1, &self.vertex2, &self.vertex3, &self.normal)
    }
}
