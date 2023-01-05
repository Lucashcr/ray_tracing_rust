use std::ops;

use crate::vec3::Vec3;


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
    fn correct_color(color: Color) -> Color {
        if 0. <= color.r && color.r <= 1. {
            if 0. <= color.g && color.g <= 1. {
                if 0. <= color.b && color.b <= 1. {
                    return Color { r: color.r, g: color.g, b: color.b };
                }
            }
        }
        Color { r: color.r, g: color.g, b: color.b } / color.r.max(color.g).max(color.b)
    }

    pub fn black() -> Color {
        Color { r: 0., g: 0., b: 0. }
    }
    pub fn white() -> Color {
        Color { r: 1., g: 1., b: 1. }
    }
    pub fn red() -> Color {
        Color { r: 1., g: 0., b: 0. }
    }
    pub fn green() -> Color {
        Color { r: 0., g: 1., b: 0. }
    }
    pub fn blue() -> Color {
        Color { r: 0., g: 0., b: 1. }
    }
    pub fn yellow() -> Color {
        Color { r: 1., g: 1., b: 0. }
    }
    pub fn magenta() -> Color {
        Color { r: 1., g: 0., b: 1. }
    }
    pub fn cyan() -> Color {
        Color { r: 0., g: 1., b: 1. }
    }
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
        format!("{} {} {}", (self.r*255.) as u32, (self.g*255.) as u32, (self.b*255.) as u32)
    }
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
}