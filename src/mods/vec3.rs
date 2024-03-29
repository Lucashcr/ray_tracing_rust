use std::ops;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x+rhs.x, y: self.y+rhs.y, z: self.z+rhs.z }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x-rhs.x, y: self.y-rhs.y, z: self.z-rhs.z }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 { x: self.x*rhs, y: self.y*rhs, z: self.z*rhs }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> f64 {
        self.dot(&rhs)
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 { x: self.x/rhs, y: self.y/rhs, z: self.z/rhs }
    }
}
impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { x: 0., y: 0., z: 0. }
    }
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 { x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>() }
    }
    pub fn limited_random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 { x: rng.gen_range(min..max), y: rng.gen_range(min..max), z: rng.gen_range(min..max) }
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::limited_random(-1., 1.);
        while p.squared_length() >= 1. {
            p = Vec3::limited_random(-1., 1.);
        }
        return p;
    }
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vec()
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn near_zero(&self) -> bool {
        let s: f64 = 1./100000000.;
        return (self.x < s) && (self.y < s) && (self.z < s);

    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn squared_length(&self) -> f64 {
        self.dot(self)
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    // pub fn to_string(&self) -> String {
    //     format!("{} {} {}", self.x, self.y, self.z)
    // }
    // pub fn to_color_string(&self) -> String {
    //     format!("{} {} {}", self.x as u32, self.y as u32, self.z as u32)
    // }
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x*&rhs.x + self.y*rhs.y + self.z*rhs.z
    }
    // pub fn cross(self, rhs: &Vec3) -> Vec3 {
    //     Vec3 { 
    //         x: self.y * rhs.z - self.z * rhs.y,
    //         y: self.z * rhs.x - self.x * rhs.z, 
    //         z: self.x * rhs.y - self.y * rhs.x
    //     }
    // }
    pub fn unit_vec(self) -> Vec3{
        self / self.length()
    }
    // pub fn to_color(&self) -> Color {
    //     Color::new(self.x, self.y, self.z)           
    // }
}