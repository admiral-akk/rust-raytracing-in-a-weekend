use crate::{math::vector, BoundingBox, Ray, Vec3};

use super::hittable::Hittable;

pub struct Sphere {
    pos: Vec3,
    radius_sq: f32,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32) -> Self {
        Self {
            pos,
            radius_sq: radius * radius,
        }
    }
}

impl Hittable for Sphere {
    // This uses the quadratic equation. a == 1, since the ray.dir is normalized.
    fn hit(&self, ray: &Ray) -> f32 {
        let diff = &self.pos - &ray.pos;
        let c = &diff * &diff - self.radius_sq;

        let b = -(&ray.dir * &diff);
        let dis = b * b - c;
        if dis < 0.0 {
            return f32::INFINITY;
        }
        let discriminant = dis.sqrt();
        let mut t = -discriminant - b;
        if t < 0.0001 {
            t = discriminant - b;
        }
        if t < 0.0001 {
            return f32::INFINITY;
        }
        return t;
    }

    fn hit_normal(&self, _ray: &Ray, hit_point: &Vec3) -> Vec3 {
        return (hit_point - &self.pos).normalized();
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(
            self.pos - (self.radius_sq.sqrt() * vector::ONE),
            self.pos + (self.radius_sq.sqrt() * vector::ONE),
        )
    }
}
