use crate::{
    hittable::{hit_record, hit_record::HitRecord},
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

impl<'a> World {
    pub fn hit(&'a self, ray: &Ray) -> HitRecord<'a> {
        let mut ret: HitRecord = hit_record::DEFAULT;
        let mut temp: HitRecord = hit_record::DEFAULT;
        for hittable in &self.objects {
            hittable.hit(ray, &mut temp);
            if !temp.hit() {
                continue;
            }
            if temp.t < ret.t {
                ret.t = temp.t;
                ret.normal = temp.normal;
                ret.point = temp.point;
                ret.object = Some(hittable);
            }
        }
        return ret;
    }
}
