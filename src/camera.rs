use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        Camera {
            origin: Vec3::zero(),
            lower_left_corner: Vec3::zero() - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length),
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin, 
            self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin
        )
    }
}