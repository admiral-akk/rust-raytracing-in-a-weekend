use crate::{Ray, Vec3};

pub mod sphere;

pub trait Hittable {
    fn is_hit(&self, ray: Ray) -> bool;
    fn hit_normal(&self, ray: Ray) -> Vec3;
}
