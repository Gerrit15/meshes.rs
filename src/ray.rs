use crate::{Vec3, Object};

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
    pub fn cast(self, scene: &Vec<Object>) -> usize {
        let mut distance = (scene[0].faces[0].closest_point(self.into()) - self.into()).magnatude();
        loop {
            for o in scene {
                for f in &o.faces {
                    let distance_from_face = (f.closest_point(self.into()) - self.into()).magnatude(); 
                    println!("distance: {}", distance_from_face);
                    if distance_from_face > distance {distance = distance_from_face}
                }
            }
            break
        }
        0
    }
}

impl Into<Vec3> for Ray {
    fn into(self) -> Vec3 {
        self.origin + self.direction
    }
}
