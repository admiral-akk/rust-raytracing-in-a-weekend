use crate::{math::vector, scene::object::Object, Vec3};

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub object: Option<&'a Object>,
}

pub const DEFAULT: HitRecord = HitRecord {
    point: vector::ZERO,
    normal: vector::ZERO,
    t: f32::INFINITY,
    object: None,
};

impl HitRecord<'_> {
    pub fn hit(&self) -> bool {
        self.t < f32::INFINITY && self.t > 0.0001
    }
}
