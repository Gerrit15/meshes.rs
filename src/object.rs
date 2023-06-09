#![allow(dead_code)]
use crate::Vec3;
use crate::Triangle;
use std::{ops, fmt};

#[derive(Clone)]
pub struct Object {
    pub verticies: Vec<Vec3>,
    pub origin: Vec3,
    pub faces: Vec<Triangle>
}

impl Object {
    /*
    * Look into Delaunay Triangulation, this should do the trick
    *
    pub fn new(verticies: Vec<Vec3>, origin: Option<Vec3>) -> Object {
        Object { verticies, origin faces }
    }*/

    pub fn new(verticies: Vec<Vec3>, faces: Vec<Triangle>, origin: Vec3) -> Object {
        Object {verticies, origin, faces}
    }
    pub fn new_triangle(v1: Vec3, v2: Vec3, v3: Vec3, origin: Option<Vec3>) -> Object {
        let faces = vec![Triangle::new(&v1, &v2, &v3)];
        let verts = vec![v1, v2, v3];
        let origin = match origin {
            Some(x) => x,
            _ => Vec3::new(0.0, 0.0, 0.0)
        };
        Object { verticies: verts, origin, faces }
    }
    pub fn new_rect(length: f64, width: f64, height: f64, origin: Option<Vec3>, rotation: Option<(f64, f64, f64)>) -> Object {
        let mut verts = vec![
            Vec3::new(-(length/2.0), width/2.0, -height/2.0),
            Vec3::new(-(length/2.0), -(width/2.0), -height/2.0),
            Vec3::new(length/2.0, width/2.0, -height/2.0),
            Vec3::new(length/2.0, -(width/2.0), -height/2.0)
        ];
        verts.push(verts[0].clone() + (0.0, 0.0, height));
        verts.push(verts[1].clone() + (0.0, 0.0, height));
        verts.push(verts[2].clone() + (0.0, 0.0, height));
        verts.push(verts[3].clone() + (0.0, 0.0, height));

        match rotation {
            Some(rotation) => {
                let mut i = 0;
                while (i as usize) < verts.len() {
                    let v = verts[i].rotate_x(rotation.0).rotate_y(rotation.1).rotate_z(rotation.2);
                    verts[i] = v;
                    i += 1
                }
            },
            None => ()
        }
        let triangles = vec![
            //bottom
            Triangle::new(&verts[0], &verts[1], &verts[2]),
            Triangle::new(&verts[1], &verts[2], &verts[3]),
            //top
            Triangle::new(&verts[4], &verts[5], &verts[6]),
            Triangle::new(&verts[5], &verts[6], &verts[7]),
            //front
            Triangle::new(&verts[1], &verts[3], &verts[5]),
            Triangle::new(&verts[3], &verts[5], &verts[7]),
            //back
            Triangle::new(&verts[0], &verts[2], &verts[4]),
            Triangle::new(&verts[2], &verts[4], &verts[6]),
            //left
            Triangle::new(&verts[0], &verts[1], &verts[4]),
            Triangle::new(&verts[1], &verts[4], &verts[5]),
            //right
            Triangle::new(&verts[2], &verts[3], &verts[6]),
            Triangle::new(&verts[3], &verts[6], &verts[7])
        ];
        let origin = match origin {
            Some(x) => x,
            None => Vec3::new(0.0, 0.0, 0.0),
        };
        Object {
            verticies: verts,
            origin,
            faces: triangles
        }
    }

    pub fn new_tetrahedron(radius: f64, origin: Option<Vec3>, rotation: Option<(f64, f64, f64)>) -> Object {
        let mut verts = vec![
            Vec3::new(-radius, 0.0, 0.0).rotate_y(-45.0),
            Vec3::new(0.0, -radius, 0.0).rotate_x(45.0).rotate_z(45.0),
            Vec3::new(0.0, radius, 0.0).rotate_x(-45.0).rotate_z(-45.0),
            Vec3::new(0.0, 0.0, radius)
        ];
        match rotation {
            Some(rotation) => {
                let mut i = 0;
                while (i as usize) < verts.len() {
                    let v = verts[i].rotate_x(rotation.0).rotate_y(rotation.1).rotate_z(rotation.2);
                    verts[i] = v;
                    i += 1
                }
            },
            None => ()
        }
        let triangles = vec![
            Triangle::new(&verts[0], &verts[1], &verts[2]),
            Triangle::new(&verts[0], &verts[2], &verts[3]),
            Triangle::new(&verts[0], &verts[1], &verts[3]),
            Triangle::new(&verts[1], &verts[2], &verts[3]),
        ]; 
        let origin = match origin {
            Some(x) => x,
            None => Vec3::new(0.0, 0.0, 0.0)
        };
        Object { 
            verticies: verts,
            origin,
            faces: triangles
        }
    }
    /*
    pub fn sphere() {}
    pub fn pyramid() {}
    pub fn taurus() {}
    pub fn triangulate() {}
    //use obj crate, I draw the line at parsing and dispensing files
    pub fn from_obj {}
    */

    //consider rewriting allllll of this post triangulate()
    pub fn rotate_x(&self, theta: f64) -> Object {
        let mut verts = vec![];
        for v in &self.verticies {
            verts.push(v.rotate_x(theta))
        }
        let mut faces = vec![];
        for f in &self.faces {
            faces.push(f.rotate_x(theta))
        }
        Object::new(verts, faces, self.origin)
    }
    pub fn rotate_y(&self, theta: f64) -> Object {
        let mut verts = vec![];
        for v in &self.verticies {
            verts.push(v.rotate_y(theta))
        }
        let mut faces = vec![];
        for f in &self.faces {
            faces.push(f.rotate_y(theta))
        }
        Object::new(verts, faces, self.origin)
    }
    pub fn rotate_z(&self, theta: f64) -> Object {
        let mut verts = vec![];
        for v in &self.verticies {
            verts.push(v.rotate_z(theta))
        }
        let mut faces = vec![];
        for f in &self.faces {
            faces.push(f.rotate_z(theta))
        }
        Object::new(verts, faces, self.origin)
    }
}
impl fmt::Display for Object {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        let mut verts: String = "Verticies:".to_owned();
        for vert in &self.verticies {
            let a = "\n(".to_string() + &vert.x.to_string() + ", " + &vert.y.to_string() + ", " + &vert.z.to_string() + ")";
            verts += &a;
        }

        let mut triangles: String = "Faces:".to_owned();
        for face in &self.faces {
            let vert1 = "(".to_string() + &face.vertex1.x.to_string() + ", " + &face.vertex1.y.to_string() + ", " + &face.vertex1.z.to_string() + ")";
            let vert2 = "(".to_string() + &face.vertex2.x.to_string() + ", " + &face.vertex2.y.to_string() + ", " + &face.vertex2.z.to_string() + ")";
            let vert3 = "(".to_string() + &face.vertex3.x.to_string() + ", " + &face.vertex3.y.to_string() + ", " + &face.vertex3.z.to_string() + ")";
            let verts = "\n( ".to_owned() + &vert1 + ", " + &vert2 + ", " + &vert3 + " )";
            triangles += &verts
        }
        write!(f, "{}\n{}", verts, triangles)
    }
}
impl ops::Add<Vec3> for Object{
    type Output = Object;

    fn add(self, translation: Vec3) -> Object{
        let mut product = self.clone();
        product.origin = Vec3::new(self.origin.x + translation.x , self.origin.y + translation.y, self.origin.z + translation.z);
        product
    }
}
impl ops::Add<(f64, f64, f64)> for Object{
    type Output = Object;

    fn add(self, translation: (f64, f64, f64)) -> Object{
        let mut product = self.clone();
        product.origin = Vec3::new(self.origin.x + translation.0 , self.origin.y + translation.0, self.origin.z + translation.0);
        product
    }
}
impl ops::Sub<Vec3> for Object{
    type Output = Object;

    fn sub(self, translation: Vec3) -> Object{
        let mut product = self.clone();
        product.origin = Vec3::new(self.origin.x - translation.x , self.origin.y - translation.y, self.origin.z - translation.z);
        product
    }
}
