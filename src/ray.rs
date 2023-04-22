use crate::{Vec3, Object, Triangle};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn cast(self, scene: &Vec<Object>) -> (Option<Object>, Ray){
/*        loop {
            let mut distance = (scene[0].faces[0].closest_point(self.origin) - self.origin).magnatude();
            for o in scene {
                if distance == 0.0 {return (Some(o.clone()), self)}
                if distance <= 0.01 {return (Some(o.clone()), self)}
                if distance >= (10.0 * 10_i32.pow(6) as f64) {return (None, self)}
                for f in &o.faces {
                    let distance_from_face = self.distance_to_face(f);
                    if distance_from_face < distance {distance = distance_from_face}
                }
            }
            self.direction += (self.direction * distance);
            println!("Distance: {distance}\nSelf direction: {}", self.direction);
        }
        */
        let mut current_pos = self.origin;
        loop {
            let mut distance = (scene[0].faces[0].closest_point(self.origin) - self.origin).magnatude();
            for o in scene {
                if distance <= 0.01 {return (Some(o.clone()), self)}
                if distance >= 1000000.0 {return (Some(o.clone()), self)}
                for f in &o.faces {
                    let distance_from_face = self.distance_to_face(f);
                    if distance_from_face < distance {distance = distance_from_face}
                }
            }
            current_pos += self.direction*distance;
        }
    }

    fn distance_to_face(&self, face: &Triangle) -> f64 {
        let point: Vec3 = (*self).into();
        let distance_from_face = face.closest_point(point) - point;
        return distance_from_face.magnatude()
    }
}

impl Into<Vec3> for Ray {
    fn into(self) -> Vec3 {
        self.origin + self.direction
    }
}
