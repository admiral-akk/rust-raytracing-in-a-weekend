use crate::{Ray, Vec3};

use super::{hit_record::HitRecord, hittable::Hittable};

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub const fn new(pos: Vec3, radius: f32) -> Self {
        Self { pos, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) {
        let diff = self.pos - ray.pos;
        let c = diff * diff - self.radius * self.radius;

        let b = -2.0 * (ray.dir * diff);
        let a = ray.dir * ray.dir;
        let mut t = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t < 0.001 {
            t = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        }
        hit_record.t = t;
        hit_record.point = ray.project(t);
        hit_record.normal = (hit_record.point - self.pos).normalized();
    }
}
