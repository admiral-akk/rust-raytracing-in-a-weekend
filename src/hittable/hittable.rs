use crate::Ray;

use super::hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord);
}
