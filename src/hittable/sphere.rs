use crate::{Ray, Vec3};

use super::Hittable;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_record: &mut super::HitRecord) {
        let diff = self.pos - ray.pos;
        let c = diff * diff - self.radius * self.radius;

        let b = -2.0 * (ray.dir * diff);
        let a = ray.dir * ray.dir;
        let t = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        if t < 0.0001 {
            return;
        }
        hit_record.t = t;
        hit_record.normal = (ray.project(t) - self.pos).normalized();
        hit_record.point = ray.project(t);
    }
}
