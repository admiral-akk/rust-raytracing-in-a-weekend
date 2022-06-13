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

impl World {
    pub fn hit<'a>(&'a self, ray: &Ray) -> HitRecord<'a> {
        let mut ret: HitRecord = hit_record::DEFAULT;
        for hittable in &self.objects {
            let t = hittable.hit(ray);
            if t >= ret.t {
                continue;
            }
            ret.t = t;
            ret.object = Some(hittable);
        }
        if ret.hit() {
            ret.point = ray.project(ret.t);
            ret.normal = ret.object.unwrap().hit_normal(ray, &ret.point);
        }
        return ret;
    }
}
