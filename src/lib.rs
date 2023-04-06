//this is a struct that holds a vector of x,y,z f64 numbers. 
//It is designed to not be mutable, all methods will return a new Vec3 or appropriate data type <-- subject to change, some day
//It impliments adding, subtracting Vec3, and will produce a dot product in the form of f64 upon
//being used to multiply.
//It supports scaling by every number in the standard library I could think of
//It has 5 associated methods:
//   - rotate_x(theta)
//   - rotate_y(theta)
//   - rotate_z(theta)
//   - normalize()
//   - magnatude()
//   - cross(B)
//
//The rotate set of methods will rotate the vector around their indicated axis by theta degrees. 
//While radians are *clearly* superior, they also have the disatvantage of being decimals
//(especially when you include pi) so it felt more suitable to use degrees, which are only
//sometimes decimals and can be represented more directly then radians
//
//The normalize() method will return a normalized vector, meaning that the x, y, and z components
//will be divided by the magnatude of the vector, so that it rests inside the unit circle.
//
//The magnatude() method is quite simple, it's an extension of pythagorean theorem into three
//dimensions, and returns a f64 
//
//The cross() method returns the cross product of itself and B. I will not be elaborating on how it
//works, because I in fact do not 100% understand how it works. 
use std::ops;
use std::fmt;
#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }
    pub fn new_from_tuple(a: (f64, f64, f64)) -> Vec3 {
        Vec3 { x: a.0, y: a.1, z: a.2 }
    }
    pub fn magnatude(&self) -> f64 {
        return (self.x*self.x+self.y*self.y+self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        let magnatude = self.magnatude();
        return self.clone()/magnatude
    }
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
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, translation: Vec3) -> Vec3{
        Vec3::new(self.x + translation.x , self.y + translation.y, self.z + translation.y)
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, translation: Vec3) -> Vec3{
        Vec3::new(self.x - translation.x , self.y - translation.y, self.z - translation.y)
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

//and so begins the scaling
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: f64) -> Vec3{
        Vec3::new(self.x * scaler, self.y * scaler, self.z * scaler)
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: f32) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<u8> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: u8) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<u16> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: u16) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<u32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: u32) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<u64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: u64) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<u128> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: u128) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<i8> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: i8) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<i16> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: i16) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: i32) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: i64) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<i128> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: i128) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<usize> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: usize) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<isize> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: isize) -> Vec3{
        Vec3::new(self.x * scaler as f64, self.y * scaler as f64, self.z * scaler as f64)
    }
}
impl ops::Mul<(u8, u8, u8)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (u8, u8, u8)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(u16, u16, u16)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (u16, u16, u16)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(u32, u32, u32)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (u32, u32, u32)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(u64, u64, u64)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (u64, u64, u64)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(u128, u128, u128)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (u128, u128, u128)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(i8, i8, i8)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (i8, i8, i8)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(i16, i16, i16)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (i16, i16, i16)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(i32, i32, i32)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (i32, i32, i32)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(i64, i64, i64)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (i64, i64, i64)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(i128, i128, i128)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (i128, i128, i128)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(f32, f32, f32)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (f32, f32, f32)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(usize, usize, usize)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (usize, usize, usize)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Mul<(isize, isize, isize)> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: (isize, isize, isize)) -> Vec3{
        Vec3::new(self.x * scaler.0 as f64, self.y * scaler.1 as f64, self.z * scaler.1 as f64)
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: f64) -> Vec3{
        Vec3::new(self.x / scaler, self.y / scaler, self.z / scaler)
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: f32) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<u8> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: u8) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<u16> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: u16) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<u32> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: u32) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<u64> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: u64) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<u128> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: u128) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<i8> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: i8) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<i16> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: i16) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<i32> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: i32) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<i64> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: i64) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<i128> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: i128) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<usize> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: usize) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<isize> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: isize) -> Vec3{
        Vec3::new(self.x / scaler as f64, self.y / scaler as f64, self.z / scaler as f64)
    }
}
impl ops::Div<(u8, u8, u8)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (u8, u8, u8)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(u16, u16, u16)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (u16, u16, u16)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(u32, u32, u32)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (u32, u32, u32)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(u64, u64, u64)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (u64, u64, u64)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(u128, u128, u128)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (u128, u128, u128)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(i8, i8, i8)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (i8, i8, i8)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(i16, i16, i16)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (i16, i16, i16)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(i32, i32, i32)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (i32, i32, i32)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(i64, i64, i64)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (i64, i64, i64)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(i128, i128, i128)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (i128, i128, i128)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(f32, f32, f32)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (f32, f32, f32)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(f64, f64, f64)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (f64, f64, f64)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(usize, usize, usize)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (usize, usize, usize)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
impl ops::Div<(isize, isize, isize)> for Vec3 {
    type Output = Vec3;
    fn div(self, scaler: (isize, isize, isize)) -> Vec3{
        Vec3::new(self.x / scaler.0 as f64, self.y / scaler.1 as f64, self.z / scaler.2 as f64)
    }
}
