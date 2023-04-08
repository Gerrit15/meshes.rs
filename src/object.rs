use crate::Vec3;
use crate::Triangle;

pub struct Object<'a> {
    pub verticies: Vec<Vec3>,
    pub faces: Vec<Triangle<'a>>
}

impl <'a>Object<'a> {
    pub fn new(verticies: Vec<Vec3>, faces: Vec<Triangle>) -> Object {
        Object { verticies, faces }
    }
    /*pub fn rect(length: f64, width: f64, height: f64, transform: Vec3, rotation: (f64, f64, f64)) -> Object {
    }*/
    pub fn sphere() {}
    pub fn taurus() {}
}
