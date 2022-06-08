use crate::{Ray, Vec3};

pub mod sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord);
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}
