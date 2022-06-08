use crate::{Ray, Vec3};

use super::Hittable;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn is_hit(&self, ray: &Ray) -> bool {
        let diff = self.pos - ray.pos;
        let c = diff.len_sq() - self.radius * self.radius;

        let b = -2.0 * (ray.dir * diff);
        let a = ray.dir * ray.dir;
        if b * b - 4.0 * a * c >= 0.0 {
            return true;
        }
        return false;
    }

    fn hit_normal(&self, ray: &Ray) -> Vec3 {
        let diff = self.pos - ray.pos;
        let c = diff.len_sq() - self.radius * self.radius;

        let b = -2.0 * (ray.dir * diff);
        let a = ray.dir * ray.dir;
        let t = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        return (ray.project(t) - self.pos).normalized();
    }
}
