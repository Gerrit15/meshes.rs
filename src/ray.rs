use crate::{Vec3, Object, Triangle};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub location: Vec3,
    pub direction: Vec3,
    pub steps: u64
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            location: origin,
            direction,
            steps: 0
        }
    }

    pub fn cast(mut self, scene: &Vec<Object>, max_steps: u64) -> (Option<Object>, Ray){
        loop {
            //set up this way because I'm not sure if I want to return an index or an object
            let mut i = 0;
            let mut r = self.distance_to_face(&scene[0].faces[0]);
            while i < scene.len() {
                for f in &scene[i].faces {
                    let dist = self.distance_to_face(f);
                    if dist < r {r = dist}
                }
                if r < 0.01 {return (Some(scene[i].clone()), self)}
                i += 1;
            }
            println!("radius: {}", r);
            println!("Location: {}", self.location);
            self.location += self.direction * r;
            self.steps += 1;
            if self.steps >= max_steps {return (None, self)}
        }
    }

    fn distance_to_face(&self, face: &Triangle) -> f64 {
        let distance_from_face = face.closest_point(self.location) - self.location;
        return distance_from_face.magnatude()
    }
}
