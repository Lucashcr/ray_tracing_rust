use std::ops;

// use crate::vec3::Vec3;


fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; } 
    if x > max { return max; }
    x
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color { r: self.r+rhs.r, g: self.g+rhs.g, b: self.b+rhs.b }
    }
}
impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        Color { r: self.r-rhs.r, g: self.g-rhs.g, b: self.b-rhs.b }
    }
}
impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        Color { r: self.r*rhs, g: self.g*rhs, b: self.b*rhs }
    }
}
impl ops::Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Color {
        Color { r: self.r/rhs, g: self.g/rhs, b: self.b/rhs }
    }
}
impl Color {
    pub fn correct_color(self) -> Color {
        if 0. <= self.r && self.r <= 1. {
            if 0. <= self.g && self.g <= 1. {
                if 0. <= self.b && self.b <= 1. {
                    return self;
                }
            }
        }
        Color { r: self.r, g: self.g, b: self.b } / self.r.max(self.g).max(self.b)
    }

    pub fn black() -> Color {
        Color { r: 0., g: 0., b: 0. }
    }
    // pub fn white() -> Color {
    //     Color { r: 1., g: 1., b: 1. }
    // }
    // pub fn red() -> Color {
    //     Color { r: 1., g: 0., b: 0. }
    // }
    // pub fn green() -> Color {
    //     Color { r: 0., g: 1., b: 0. }
    // }
    // pub fn blue() -> Color {
    //     Color { r: 0., g: 0., b: 1. }
    // }
    // pub fn yellow() -> Color {
    //     Color { r: 1., g: 1., b: 0. }
    // }
    // pub fn magenta() -> Color {
    //     Color { r: 1., g: 0., b: 1. }
    // }
    // pub fn cyan() -> Color {
    //     Color { r: 0., g: 1., b: 1. }
    // }
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color::correct_color(Color { r: r, g: g, b: b })
    }
    pub fn r(&self) -> f64 {
        self.r
    }
    pub fn g(&self) -> f64 {
        self.g
    }
    pub fn b(&self) -> f64 {
        self.b
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}", 
            (256. * clamp(self.r.sqrt(), 0., 0.999)) as u32, 
            (256. * clamp(self.g.sqrt(), 0., 0.999)) as u32, 
            (256. * clamp(self.b.sqrt(), 0., 0.999)) as u32, 
        )
    }
    // pub fn to_vec3(&self) -> Vec3 {
    //     Vec3::new(self.r, self.g, self.b)
    // }
}