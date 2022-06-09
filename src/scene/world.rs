use crate::{
    hittable::{hit_record, hit_record::HitRecord, hittable::Hittable},
    Ray,
};

use super::object::Object;

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Object) {
        self.objects.push(object);
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) {
        hit_record.t = f32::INFINITY;
        let mut temp: HitRecord = hit_record::DEFAULT;
        for hittable in &self.objects {
            hittable.hit(ray, &mut temp);
            if !temp.hit() {
                continue;
            }
            if temp.t < hit_record.t {
                hit_record.t = temp.t;
                hit_record.normal = temp.normal;
                hit_record.point = temp.point;
            }
        }
    }
}
