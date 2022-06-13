use crate::{math::vector, scene::object::Object, Hittable, Vec3};

use core::fmt::Debug;
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

impl PartialEq for HitRecord<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.normal == other.normal && self.t == other.t
    }
}

impl Debug for HitRecord<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.object.is_some() {
            return f
                .debug_struct("HitRecord")
                .field("point", &self.point)
                .field("normal", &self.normal)
                .field("t", &self.t)
                .field("bb", &self.object.unwrap().bounds())
                .finish();
        }
        return f
            .debug_struct("HitRecord")
            .field("point", &self.point)
            .field("normal", &self.normal)
            .field("t", &self.t)
            .finish();
    }
}

impl HitRecord<'_> {
    pub fn hit(&self) -> bool {
        self.t < f32::INFINITY && self.t > 0.0001
    }
}
