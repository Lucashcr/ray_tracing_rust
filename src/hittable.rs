use std::vec;

use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face { *outward_normal } else { *outward_normal * (-1.) }; 
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64 
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center: center, radius: radius }
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin_center = *r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = origin_center.dot(r.direction());
        let c = origin_center.squared_length() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);

        true
    }
}

pub struct HittableList {
    objects: vec::Vec<Sphere>
}

impl HittableList {
    pub fn new() -> HittableList { 
        let list = HittableList { objects: Vec::new() };
        return list
    }
    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn add(&mut self, object: Sphere) {
        self.objects.push(object);
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::zero(), normal: Vec3::zero(), t: 0., front_face: false 
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in (*self.objects).into_iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = rec.t;
                rec.front_face = temp_rec.front_face;
                rec.normal = temp_rec.normal;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
            }
            // println!(
            //     "Sphere - c=({}, {}, {}) / r={} / hit={}", 
            //     object.center.x(), object.center.y(), object.center.z(), 
            //     object.radius, hit_anything
            // );
        }

        return hit_anything;
    }
}
