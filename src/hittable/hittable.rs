use crate::{Ray, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> f32;
    fn hit_normal(&self, ray: &Ray, hit_point: &Vec3) -> Vec3;
}
