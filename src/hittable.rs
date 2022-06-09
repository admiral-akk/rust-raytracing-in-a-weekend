use crate::{math::vector, Ray, Vec3};

pub mod sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord);
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

pub const DEFAULT: HitRecord = HitRecord {
    point: vector::ZERO,
    normal: vector::ZERO,
    t: f32::INFINITY,
};

impl HitRecord {
    pub fn hit(&self) -> bool {
        self.t < f32::INFINITY && self.t > 0.0001
    }
}
