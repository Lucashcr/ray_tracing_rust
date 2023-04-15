use std::vec;

use crate::{mods::material::Material, Ray, Vec3};
// use crate::mods::material::Material;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * (-1.)
        };
    }
}

pub enum Hittable {
    Sphere {
        center: Vec3,
        radius: f64,
        material: Material,
    },
}

impl Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere {
                center,
                radius,
                material,
            } => {
                let origin_center = *r.origin() - *center;
                let a = r.direction().squared_length();
                let half_b = origin_center.dot(r.direction());
                let c = origin_center.squared_length() - radius * radius;

                let discriminant = half_b * half_b - a * c;

                if discriminant < 0. {
                    return false;
                }

                let root = (-half_b - discriminant.sqrt()) / a;
                if root < t_min || root > t_max {
                    let root = (-half_b + discriminant.sqrt()) / a;
                    if root < t_min || root > t_max {
                        return false;
                    }
                }

                rec.t = root;
                rec.p = r.at(rec.t);
                rec.material = *material;
                let outward_normal = (rec.p - *center) / *radius;
                rec.set_face_normal(&r, &outward_normal);

                return true;
            }
        }
    }
}

pub struct HittableList {
    objects: vec::Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            material: Material::None,
            t: 0.,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in (*self.objects).into_iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = rec.t;

                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}
