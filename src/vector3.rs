use std::ops;
use std::fmt;
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }
    pub fn magnatude(&self) -> f64 {
        return (self.x*self.x+self.y*self.y+self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        return self.clone()/self.magnatude()
    }

    //these functions use degrees because while radians are superior, 
    //degrees are represented with less sig figs

    /*
    * [  1       0           0       ]
    * [  0  cos(theta)  -sin(theta)  ]
    * [  0  sin(theta)  cos(theta)]  ]
    *
    *  x' = 1x + 0y + 0z
    *  y' = 0x + cos(theta)y + -sin(theta)z
    *  z' = 0x + sin(theta)y + cos(theta)
    */
    pub fn rotate_x(&self, degrees: f64) -> Vec3 {
        let sine = match degrees as u32 {
            0 => 0.0,
            30 => 0.5,
            90 => 1.0,
            _ => degrees.to_radians().sin()
        };
        let cosine = match degrees as u32 {
            0 => 1.0,
            60 => 0.5,
            90 => 0.0,
            _ => degrees.to_radians().cos()
        };
        let y_prime = cosine*self.y + sine*self.z * -1.0;
        let z_prime = sine*self.y + cosine*self.z;
        return Vec3::new(self.x, y_prime, z_prime)
    }

    /*
    * [ cos(theta)  0  sin(theta)  ]
    * [     0       1      0       ]
    * [ -sin(theta) 0  cos(theta)  ]
    *
    *  x' = cos(theta)x + 0y + sin(theta)z
    *  y' = 0x + 1y + 0z
    *  z' = -sin(theta)x + 0y + cos(theta)z
    */
    pub fn rotate_y(&self, degrees: f64) -> Vec3 {
        let sine = match degrees as u32 {
            0 => 0.0,
            30 => 0.5,
            90 => 1.0,
            _ => degrees.to_radians().sin()
        };
        let cosine = match degrees as u32 {
            0 => 1.0,
            60 => 0.5,
            90 => 0.0,
            _ => degrees.to_radians().cos()
        };
        let x_prime = cosine*self.x + sine*self.z;
        let z_prime = (-1.0)*sine*self.x + cosine*self.z;
        return Vec3::new(x_prime, self.y, z_prime)
    }

    /*
    * [  cos(theta)   -sin(theta)    0  ]
    * [  sin(theta)    cos(theta)    0  ]
    * [      0            0          1  ]
    *
    *  x' = cos(theta)x + -sin(theta)y + 0z
    *  y' = sin(theta)x + cos(theta)y + 0z
    *  z' = 0x + 0y + 1z
    */
    pub fn rotate_z(&self, degrees: f64) -> Vec3 {
        let sine = match degrees as u32 {
            0 => 0.0,
            30 => 0.5,
            90 => 1.0,
            _ => degrees.to_radians().sin()
        };
        let cosine = match degrees as u32 {
            0 => 1.0,
            60 => 0.5,
            90 => 0.0,
            _ => degrees.to_radians().cos()
        };
        let x_prime = cosine*self.x + (-1.0)*sine*self.y;
        let y_prime = sine*self.x + cosine*self.y;
        return Vec3::new(x_prime, y_prime, self.z)
    }

    /*
    * AxB = (ax, ay, az)x(bx, by, bz)
    * AxB = (x', y', z')
    * x' = ay*bz - az*by 
    * y' = az*bx - ax*bz
    * z' = ax*by - ay*bx
    */
    //tbh idk how this works. but it does! :D
    pub fn cross(&self, b: Vec3) -> Vec3 {
        let x_prime = self.y*b.z - self.z*b.y;
        let y_prime = self.z*b.x - self.x*b.z;
        let z_prime = self.x*b.y - self.y*b.x;
        Vec3::new(x_prime, y_prime, z_prime)
    }
    pub fn reflect(&self, n: Vec3) -> Vec3{
        // d' = d -2(d*n)n
        self.clone() + (-2.0 * (self.clone() * n.clone()))*n.clone()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, translation: Vec3) -> Vec3{
        Vec3::new(self.x + translation.x , self.y + translation.y, self.z + translation.z)
    }
}
impl ops::Add<(f64, f64, f64)> for Vec3 {
    type Output = Vec3;

    fn add(self, translation: (f64, f64, f64)) -> Vec3{
        Vec3::new(self.x + translation.0 , self.y + translation.0, self.z + translation.0)
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, translation: Vec3) -> Vec3{
        Vec3::new(self.x - translation.x , self.y - translation.y, self.z - translation.z)
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, multiplier: Vec3) -> f64 {
        self.x*multiplier.x + self.y*multiplier.y + self.z*multiplier.z
    }
}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "X: {}, Y: {}, Z: {}", self.x, self.y, self.z)
    }
}
impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}
impl Copy for Vec3 {}
impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}
//my new favorite thing ever
macro_rules! impl_muli_divi {
    ($($typ:ty), *) => {
        $(
            impl ops::Mul<$typ> for Vec3{
                type Output = Vec3;

                fn mul(self, scaler: $typ) -> Vec3{
                    Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
                }
            }
            impl ops::Mul<Vec3> for $typ{
                type Output = Vec3;

                fn mul(self, vec: Vec3) -> Vec3{
                    Vec3::new(vec.x * self as f64, vec.y * self as f64, vec.z * self as f64)
                }
            }
            impl ops::Div<$typ> for Vec3{
                type Output = Vec3;

                fn div(self, scaler: $typ) -> Vec3{
                    Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
                }
            }
            impl ops::Div<Vec3> for $typ{
                type Output = Vec3;

                fn div(self, vec: Vec3) -> Vec3{
                    Vec3::new(vec.x / self as f64, vec.y / self as f64, vec.z / self as f64)
                }
            }
            impl ops::MulAssign<$typ> for Vec3{
                fn mul_assign(&mut self, scaler: $typ) {
                    *self = Vec3 {
                        x: self.x * scaler as f64,
                        y: self.y * scaler as f64,
                        z: self.z * scaler as f64
                    }
                }
            }
            impl ops::DivAssign<$typ> for Vec3{
                fn div_assign(&mut self, scaler: $typ) {
                    *self = Vec3 {
                        x: self.x / scaler as f64,
                        y: self.y / scaler as f64,
                        z: self.z / scaler as f64
                    }
                }
            }
        )*
    };
}
/*macro_rules! impl_muli_divi_tuple {
    ($(($typa:ty, $typb:ty, $typc:ty)), *) => {
        $(
          impl ops::Mul<($typa, $typb, $typc)> for Vec3{
                type Output = Vec3;

                fn mul(self, scaler: ($typa, $typb, $typc)) -> Vec3{
                    Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.2 as f64)
                }
            }
            impl ops::Div<($typa, $typb, $typc)> for Vec3{
                type Output = Vec3;

                fn div(self, scaler: ($typa, $typb, $typc)) -> Vec3{
                    Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.0 as f64)
                }
            }
            impl ops::MulAssign<($typa, $typb, $typc)> for Vec3{
                fn mul_assign(&mut self, scaler: ($typa, $typb, $typc)) {
                    *self = Vec3 {
                        x: self.x * scaler.0 as f64,
                        y: self.y * scaler.1 as f64,
                        z: self.z * scaler.2 as f64
                    }
                }
            }
            impl ops::DivAssign<($typa, $typb, $typc)> for Vec3{
                fn div_assign(&mut self, scaler: ($typa, $typb, $typc)) {
                    *self = Vec3 {
                        x: self.x / scaler.0 as f64,
                        y: self.y / scaler.1 as f64,
                        z: self.z / scaler.2 as f64
                    }
                }
            }
        )*
    }
}*/
impl_muli_divi!(
    f32, f64,
    u8, u16, u32, u64, u128, usize, 
    i8, i16, i32, i64, i128, isize);
