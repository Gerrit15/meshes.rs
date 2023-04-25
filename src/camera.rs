use crate::Vec3;

pub struct Camera {
    pub location: Vec3,
    pub direction: Vec3,
    pub resolution: (u64, u64),
    pub pixels: Vec<Vec<Vec3>>
    //focal length?
}

impl Camera {
    pub fn new(location: Vec3, direction: Vec3, vres: u64, hres: u64) -> Camera {
        let mut pixels = vec![];
        let xstep = 1.0/hres as f64;
        let ystep = 1.0/vres as f64;
        let mut i = 0;
        while i < vres {
            let mut buff = vec![];
            let mut j = 0;
            while j < hres {
                buff.push(Vec3::new(xstep * j as f64, ystep * i as f64, 0.0));
                j += 1;
            }
            pixels.push(buff);
            i += 1;
        }
        Camera {
            location,
            direction,
            resolution: (vres, hres),
            pixels
        }
    }
    //rotate, impl movement
}
