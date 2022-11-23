use std::fs::File;
use std::io::prelude::*;

mod vec3; use vec3::Vec3;
mod ray; use ray::Ray;
mod color; use color::Color;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let origin_center = *r.origin() - *center;
    let a = r.direction().squared_length();
    let b = 2.0 * origin_center.dot(r.direction());
    let c = origin_center.squared_length() - radius*radius;
    let discriminant = b*b - 4.*a*c;
    discriminant > 0.
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Vec3::new(0., 0., -1.), 0.5, r) {
        return Color::new(2., 1., 0.);
    }
    let unit_dir = r.direction().unit_vec();
    let t = 0.5 * (unit_dir.y() + 1.0);
    Color::new(1., 1., 1.)*t + Color::new( 0.5, 0.7, 1. )*(1.0-t)
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("hello-image.ppm")?;

    // Parâmetros da imagem
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Parâmetros de câmera
    const VIEWPORT_WIDTH: f64 = 2.0;
    const VIEWPORT_HEIGHT: f64 = ASPECT_RATIO * VIEWPORT_WIDTH;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner = 
        origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., FOCAL_LENGTH)
    ;

    let mut content = String::from(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));
    
    // Desenhando background
    for j in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT-1) as f64;
            let r = Ray::new(
                origin, lower_left_corner + horizontal*u + vertical*v - origin
            );
            let pixel_color = ray_color(&r);
            content += &pixel_color.to_string();
            content += "\n";
        }
    }

    file.write_all(content.as_str().as_bytes())?;
    print!("Done!\n");

    Ok(())
}
