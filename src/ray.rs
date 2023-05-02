use crate::{Vec3, Object, Triangle};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub location: Vec3,
    pub direction: Vec3,
    pub steps: u64,
    pub distance: f64
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            location: origin,
            direction,
            steps: 0,
            distance: 0.0
        }
    }

    pub fn cast(mut self, scene: &Vec<Object>, max_steps: u64) -> (Option<(Object, usize)>, Ray){
        loop {
            //set up this way because I'm not sure if I want to return an index or an object
            //so for now, both
            let mut i = 0;
            println!("Origin (in cast): {}", scene[0].origin);
            let mut r = self.distance_to_face(&scene[0].faces[0], scene[0].origin);
            while i < scene.len() {
                for f in &scene[i].faces {
                    let dist = self.distance_to_face(f, scene[i].origin);
                    if dist < r {r = dist}
                }
                if r < 0.0001 {return (Some((scene[i].clone(), i)), self)}
                i += 1;
            }
            self.location += self.direction * r;
            self.distance += r;
            self.steps += 1;
            if self.steps >= max_steps {return (None, self)}
        }
    }

    fn distance_to_face(&self, face: &Triangle, origin: Vec3) -> f64 {
        let distance_from_face = face.closest_point(origin, self.location) - self.location;
        println!("Distance in distance function: {}", distance_from_face);
        return distance_from_face.magnatude()
    }
}
