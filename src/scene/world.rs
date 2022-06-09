use crate::{
    hittable::{self, HitRecord, Hittable},
    Ray,
};

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
        let mut temp: HitRecord = hittable::DEFAULT;
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
pub struct Object {
    hitbox: Box<dyn Hittable>,
}

impl Object {
    pub fn new(hittable: Box<dyn Hittable>) -> Object {
        Object { hitbox: hittable }
    }
    pub fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) {
        self.hitbox.hit(ray, hit_record);
    }
}
