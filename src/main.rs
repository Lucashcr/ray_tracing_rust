use std::{fs::File, f64::INFINITY};
use std::io::prelude::*;

mod color; use color::Color;
mod ray; use ray::Ray;
mod hittable; use hittable::{Sphere, HitRecord, HittableList};
mod vec3; use vec3::Vec3;


fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord {
        p: Vec3::zero(), normal: Vec3::zero(), t: 0., front_face: false 
    };

    if world.hit(r, 0., INFINITY, &mut rec) {
        return ((rec.normal + Color::new(1., 1., 1.).to_vec3()) * 0.5).to_color();
    }

    let unit_dir = r.direction().unit_vec();
    let t = 0.5 * (1. - unit_dir.y());
    Color::new(1., 1., 1.)*t + Color::new( 0.5, 0.7, 1. )*(1.0-t)

    // let t = s.hit(r, t_min, t_max, rec); // hit_sphere(&Vec3::new(0., 0., -1.), 0.5, r);
    // if t > 0.0 {
    //     let N = (r.at(t) - Vec3::new(0., 0., -1.)).unit_vec();
    //     let rec = HitRecord { p: (), normal: N, t: t };
    //     return Color::new(N.x() + 1., N.y() + 1., N.z() + 1.) * 0.85;
    //     // return Color::new(2., 1., 0.);
    // }
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("hello-image.ppm")?;

    // Parâmetros da imagem
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Parâmetros de câmera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner = 
        origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., FOCAL_LENGTH)
    ;

    let mut world = HittableList::new();
    world.add( Sphere{ center: Vec3::new(0., 0., -1.), radius: 0.5 } );
    world.add( Sphere{ center: Vec3::new(0., -100.5, -1.), radius: 100. } );

    let mut content = String::from(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));
    
    // Desenhando background
    for j in (0..IMAGE_HEIGHT).rev() {
        // println!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT-1) as f64;
            let r = Ray::new(
                origin, lower_left_corner + horizontal*u + vertical*v - origin
            );
            let pixel_color = ray_color(&r, &world);
            content += &pixel_color.to_string();
            content += "\n";
        }
    }

    file.write_all(content.as_str().as_bytes())?;
    print!("Done!\n");

    Ok(())
}
