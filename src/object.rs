use crate::Vec3;
use crate::Triangle;
use std::fmt;

pub struct Object {
    pub verticies: Vec<Vec3>,
    pub origin: Vec3,
    pub faces: Vec<Triangle>
}

impl Object {
    /*
    * Look into Delaunay Triangulation, this should do the trick
    *
    pub fn new(verticies: Vec<Vec3>, faces: Vec<Triangle>) -> Object {
        Object { verticies, faces }
    }*/

    pub fn new_rect(length: f64, width: f64, height: f64, origin: Vec3, rotation: (f64, f64, f64)) -> Object {
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

        let mut i = 0;
        while (i as usize) < verts.len() {
            let v = verts[i].rotate_x(rotation.0).rotate_y(rotation.1).rotate_z(rotation.2);
            verts[i] = v;
            i += 1
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
        Object {
            verticies: verts,
            origin,
            faces: triangles
        }
    }
    pub fn sphere() {}
    pub fn pyramid() {}
    pub fn taurus() {}
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
