use std::io::prelude::*;
use std::{f64::INFINITY, fs::File};

use rand::Rng;

mod mods;
use crate::mods::{camera::Camera, color::Color, hittable::*, material::*, ray::Ray, vec3::Vec3};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::black();
    }

    let mut rec = HitRecord {
        p: Vec3::zero(),
        normal: Vec3::zero(),
        material: Material::Lambertian(Lambertian::new(Color::black())),
        t: 0.,
        front_face: false,
    };

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    let unit_dir = r.direction().unit_vec();
    let t = 0.5 * (1. - unit_dir.y());
    Color::new(1., 1., 1.) * t + Color::new(0.5, 0.7, 1.) * (1.0 - t)
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("hello-image.ppm")?;

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXELS: u32 = 64;
    const MAX_DEPTH: i32 = 16;

    let cam = Camera::new(ASPECT_RATIO);
    // Parâmetros da imagem
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Parâmetros de câmera
    let mut world = HittableList::new();
    world.clear();

    world.add(Hittable::Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
        material: Material::Lambertian(Lambertian::new(Color::yellow())),
    });
    world.add(Hittable::Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
        material: Material::Lambertian(Lambertian::new(Color::red())),
    });

    let mut content = String::from(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    let mut rng = rand::thread_rng();

    // Desenhando background
    for j in (0..IMAGE_HEIGHT).rev() {
        // println!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::black();
            for _ in 0..SAMPLES_PER_PIXELS {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            content += &(pixel_color / SAMPLES_PER_PIXELS as f64).to_string();
            content += "\n";
        }
        println!("{}%", 100 - 100 * j / IMAGE_HEIGHT);
    }

    file.write_all(content.as_str().as_bytes())?;
    print!("Done!\n");

    Ok(())
}
