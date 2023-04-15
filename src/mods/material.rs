use crate::{Color, HitRecord, Ray, Vec3};

trait Scatterable {
    fn scatter(
        &self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray
    ) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    None
}
impl Scatterable for Material {
    fn scatter(
        &self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray 
    ) -> bool {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
            _ => false
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Lambertian{
    albedo: Color
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}
impl Scatterable for Lambertian {
    fn scatter(
        &self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray 
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true
    }
}